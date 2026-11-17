// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! Task-local storage
//!
//! Task-local storage is a way to create global variables specific to the current task that live
//! for the entirety of the task's lifetime, almost like statics. Since they are local to the task,
//! they implement [`Send`] and [`Sync`], regardless of what the underlying data does or does not
//! implement.
//!
//! Task-locals can be declared using the [`task_local`] macro, which creates a [`LocalKey`] with
//! the same name that can be used to access the local.

extern crate alloc;

use alloc::{collections::VecDeque, rc::Rc, sync::Arc};
use core::{
    cell::RefCell,
    sync::atomic::{AtomicBool, Ordering},
    task::{Context, Poll},
};
use waker_fn::waker_fn;

use crate::{
    platform,
    runtime::{
        local::TaskLocalStorage,
        reactor::Reactor,
        task::{Task, TaskMetadata},
    },
};

type Runnable = async_task::Runnable<TaskMetadata>;

pub(crate) static EXECUTOR: Executor = Executor::new();

pub(crate) struct Executor {
    queue: RefCell<VecDeque<Runnable>>,
    reactor: RefCell<Reactor>,
    pub(crate) tls: RefCell<Option<Rc<TaskLocalStorage>>>,
}

impl Executor {
    pub const fn new() -> Self {
        Self {
            queue: RefCell::new(VecDeque::new()),
            reactor: RefCell::new(Reactor::new()),
            tls: RefCell::new(None),
        }
    }

    pub fn spawn<T>(&self, future: impl Future<Output = T> + 'static) -> Task<T> {
        let metadata = TaskMetadata {
            tls: Rc::new(TaskLocalStorage::new()),
        };

        // SAFETY: `runnable` will never be moved off this thread or shared with another thread
        // because of the `!Send + !Sync` bounds on `Self`. Both `future` and `schedule` are
        // `'static` so they cannot be used after being freed.
        //
        // TODO: Makesure that the waker can never be sent off the thread.
        let (runnable, task) = unsafe {
            async_task::Builder::new()
                .metadata(metadata)
                .spawn_unchecked(
                    move |_| future,
                    |runnable| {
                        self.queue.borrow_mut().push_back(runnable);
                    },
                )
        };

        runnable.schedule();

        task
    }

    /// Run the provided closure with the reactor.
    /// Used to ensure the thread safety of the executor.
    pub(crate) fn with_reactor(&self, f: impl FnOnce(&mut Reactor)) {
        f(&mut self.reactor.borrow_mut());
    }

    /// Wakes any expired sleepers, then polls a single task. If all tasks were sleeping,
    /// returns how long it will be until one is awake.
    pub(crate) fn tick(&self) {
        self.reactor.borrow_mut().tick();

        let runnable = {
            let mut queue = self.queue.borrow_mut();
            queue.pop_front()
        };

        if let Some(runnable) = runnable {
            TaskLocalStorage::scope(runnable.metadata().tls.clone(), || {
                runnable.run();
            });
        }
    }

    pub fn block_on<T>(&self, future: impl Future<Output = T>) -> T {
        let woken = Arc::new(AtomicBool::new(false));
        let waker = waker_fn({
            let woken = woken.clone();
            move || woken.store(true, Ordering::Relaxed)
        });
        let mut cx = Context::from_waker(&waker);

        futures_util::pin_mut!(future);

        let mut was_woken = true;
        loop {
            if was_woken && let Poll::Ready(output) = future.as_mut().poll(&mut cx) {
                return output;
            }

            platform::yield_now();

            self.tick();
            // This is updated only after the tick because another task could have woken up the
            // future in that time.
            was_woken = woken.swap(false, Ordering::Relaxed);
        }
    }
}

// SAFETY: The target is single-core and the VEXos scheduler is cooperative
// (tasks only switch at explicit yield points, see `platform::yield_now`).
// EXECUTOR is therefore only ever touched from one execution context at a
// time, so sharing it across the `static` is sound despite the RefCell.
unsafe impl Sync for Executor {}
