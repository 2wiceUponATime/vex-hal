// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

pub(crate) struct SingleThreaded<T>(T);

impl<T> SingleThreaded<T> {
    /// # Safety
    /// The caller guarantees the value is only ever accessed from one execution context for its
    /// whole lifetime (single-core cooperative scheduler, or a test harness pinned to one thread).
    /// Otherwise the interior `RefCell` borrow flags race, which is undefined behavior.
    pub const unsafe fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> core::ops::Deref for SingleThreaded<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// SAFETY: Guaranteed by the contract on `SingleThreaded::new` - one execution context means `&Self`
// shared across threads never yields concurrent access.
unsafe impl<T> Sync for SingleThreaded<T> {}
