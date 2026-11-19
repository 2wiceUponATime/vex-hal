// SPDX-FileCopyrightText: 2023 Gavin Niederman
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

extern crate alloc;

use alloc::{boxed::Box, collections::BTreeMap, rc::Rc};
use core::{
    any::Any,
    cell::{BorrowError, BorrowMutError, Cell, RefCell, UnsafeCell},
    ptr,
    sync::atomic::{AtomicU32, Ordering},
};

use spin::LazyLock;

use crate::runtime::executor::EXECUTOR;

#[derive(Debug)]
pub struct LocalKey<T: 'static> {
    init: fn() -> T,
    key: LazyLock<u32>,
}

unsafe impl<T> Sync for LocalKey<T> {}
unsafe impl<T> Send for LocalKey<T> {}

#[macro_export]
macro_rules! task_local {
    {
        $(#[$attr:meta])*
        $vis:vis static $name:ident: $type:ty = $init:expr;
    } => {
        $(#[$attr])*
        $vis static $name: $crate::runtime::local::LocalKey<$type> = {
            fn init() -> $type { $init }
            $crate::runtime::local::LocalKey::new(init)
        };
    };

    {
        $(#[$attr:meta])*
        $vis:vis static $name:ident: $type:ty = $init:expr;
        $($rest:tt)*
    } => {
        $crate::task_local!($vis static $name: $type = $init;);
        $crate::task_local!($($rest)*);
    }
}
pub use task_local;

impl<T: 'static> LocalKey<T> {
    #[doc(hidden)]
    pub const fn new(init: fn() -> T) -> Self {
        static LOCAL_KEY_COUNTER: AtomicU32 = AtomicU32::new(0);

        Self {
            init,
            key: LazyLock::new(|| LOCAL_KEY_COUNTER.fetch_add(1, Ordering::Relaxed)),
        }
    }

    pub fn with<F: FnOnce(&T) -> R, R>(&'static self, f: F) -> R {
        TaskLocalStorage::with_current(|storage| {
            // SAFETY: get_or_init is always called with the same return type, T
            // Also, `key` is unique for this local key.
            f(unsafe { storage.get_or_init(*self.key, self.init) })
        })
    }
}

impl<T: 'static> LocalKey<Cell<T>> {
    /// Returns a copy of the contained value.
    #[must_use]
    pub fn get(&'static self) -> T
    where
        T: Copy,
    {
        self.with(Cell::get)
    }

    /// Sets the contained value.
    pub fn set(&'static self, value: T) {
        self.with(|cell| cell.set(value));
    }

    /// Takes the value of contained value, leaving [`Default::default()`] in its place.
    pub fn take(&'static self) -> T
    where
        T: Default,
    {
        self.with(Cell::take)
    }

    /// Replaces the contained value with `value`, returning the old contained value.
    pub fn replace(&'static self, value: T) -> T {
        self.with(|cell| cell.replace(value))
    }
}

impl<T: 'static> LocalKey<RefCell<T>> {
    /// Immutably borrows from the [`RefCell`] and applies the obtained reference to `f`.
    ///
    /// # Panics
    ///
    /// Panics if the value is currently mutably borrowed. For a non-panicking variant, use
    /// [`LocalKey::try_with_borrow`].
    pub fn with_borrow<F, R>(&'static self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        self.with(|cell| f(&cell.borrow()))
    }

    /// Mutably borrows from the [`RefCell`] and applies the obtained reference to `f`.
    ///
    /// # Panics
    ///
    /// Panics if the value is currently borrowed. For a non-panicking variant, use
    /// [`LocalKey::try_with_borrow_mut`].
    pub fn with_borrow_mut<F, R>(&'static self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        self.with(|cell| f(&mut cell.borrow_mut()))
    }

    /// Tries to immutably borrow the contained value, returning an error if it is currently
    /// mutably borrowed, and applies the obtained reference to `f`.
    ///
    /// This is the non-panicking variant of [`LocalKey::with_borrow`].
    ///
    /// # Errors
    ///
    /// Returns [`BorrowError`] if the contained value is currently mutably borrowed.
    pub fn try_with_borrow<F, R>(&'static self, f: F) -> Result<R, BorrowError>
    where
        F: FnOnce(&T) -> R,
    {
        self.with(|cell| cell.try_borrow().map(|value| f(&value)))
    }

    /// Tries to mutably borrow the contained value, returning an error if it is currently borrowed,
    /// and applies the obtained reference to `f`.
    ///
    /// This is the non-panicking variant of [`LocalKey::with_borrow_mut`].
    ///
    /// # Errors
    ///
    /// Returns [`BorrowMutError`] if the contained value is currently borrowed.
    pub fn try_with_borrow_mut<F, R>(&'static self, f: F) -> Result<R, BorrowMutError>
    where
        F: FnOnce(&T) -> R,
    {
        self.with(|cell| cell.try_borrow_mut().map(|value| f(&value)))
    }

    /// Sets the contained value.
    ///
    /// # Panics
    ///
    /// Panics if the value is currently borrowed.
    pub fn set(&'static self, value: T) {
        self.with_borrow_mut(|refmut| *refmut = value);
    }

    /// Takes the contained value, leaving [`Default::default()`] in its place.
    ///
    /// # Panics
    ///
    /// Panics if the value is currently borrowed.
    pub fn take(&'static self) -> T
    where
        T: Default,
    {
        self.with(RefCell::take)
    }

    /// Replaces the contained value with `value`, returning the old contained value.
    ///
    /// # Panics
    ///
    /// Panics if the value is currently borrowed.
    pub fn replace(&'static self, value: T) -> T {
        self.with(|cell| cell.replace(value))
    }
}

struct ErasedTaskLocal {
    value: Box<dyn Any>,
}

impl ErasedTaskLocal {
    #[doc(hidden)]
    fn new<T: 'static>(value: T) -> Self {
        Self {
            value: Box::new(value),
        }
    }

    /// # Safety
    ///
    /// Caller guarantees T is the right type
    unsafe fn get<T: 'static>(&self) -> &T {
        if cfg!(debug_assertions) {
            self.value.downcast_ref().unwrap()
        } else {
            unsafe { &*ptr::from_ref(&*self.value).cast() }
        }
    }
}

#[derive(Debug)]
pub(crate) struct TaskLocalStorage {
    locals: UnsafeCell<BTreeMap<u32, ErasedTaskLocal>>,
}

static FALLBACK_TLS: TaskLocalStorage = const { TaskLocalStorage::new() };

impl TaskLocalStorage {
    pub(crate) const fn new() -> Self {
        Self {
            locals: UnsafeCell::new(BTreeMap::new()),
        }
    }

    pub(crate) fn scope(value: Rc<TaskLocalStorage>, scope: impl FnOnce()) {
        let outer_scope = EXECUTOR.tls.borrow_mut().replace(value);

        scope();

        *EXECUTOR.tls.borrow_mut() = outer_scope;
    }

    /// Gets the Task Local Storage data for the current task.
    pub(crate) fn with_current<F, R>(f: F) -> R
    where
        F: FnOnce(&Self) -> R,
    {
        if let Some(tls) = EXECUTOR.tls.borrow().as_ref() {
            f(tls)
        } else {
            f(&FALLBACK_TLS)
        }
    }

    /// Gets a reference to the Task Local Storage item identified by the given key.
    ///
    /// It is invalid to call this function multiple times with the same key and a different `T`.
    pub(crate) unsafe fn get_or_init<T: 'static>(&self, key: u32, init: fn() -> T) -> &T {
        // We need to be careful to not make mutable references to values already inserted into the
        // map because the current task might have existing shared references to that data. It's
        // okay if the pointer (ErasedTaskLocal) gets moved around, we just can't assert invalid
        // exclusive access over its contents.

        let locals = self.locals.get();
        unsafe {
            // init() could initialize another task local recursively, so we need to be sure there's
            // no mutable reference to `self.locals` when we call it. We can't use the
            // entry API because of this.

            #[expect(
                clippy::map_entry,
                reason = "cannot hold mutable reference over init() call"
            )]
            if !(*locals).contains_key(&key) {
                let new_value = ErasedTaskLocal::new(init());
                (*locals).insert(key, new_value);
            }

            (*locals).get(&key).unwrap().get()
        }
    }
}

// SAFETY: Same as `EXECUTOR`
unsafe impl Sync for TaskLocalStorage {}
