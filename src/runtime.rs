// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

use crate::runtime::executor::with_executor;

pub mod executor;
pub mod local;
pub mod reactor;
pub mod sleep;
pub mod task;

/// Synchronization primitives for async code.
///
/// vexide programs often use async [tasks](crate::task) to run multiple operations concurrently.
/// These primitives provide methods for tasks to safely communicate with each other and share data.
/// This is vexide's async equivalent to the [`std::sync` module].
///
/// [`std::sync` module]: https://doc.rust-lang.org/stable/std/sync/index.html
pub mod sync {
    pub use async_lock::{
        Barrier, BarrierWaitResult, Mutex, MutexGuard, OnceCell, RwLock, RwLockReadGuard,
        RwLockWriteGuard,
    };
    pub use futures_util::{join, select_biased, try_join};
}

/// Blocks the current task until a return value can be extracted from the provided future.
///
/// Does not poll all futures to completion.
pub fn block_on<T>(future: impl Future<Output = T>) -> T {
    with_executor(|ex| ex.block_on(future))
}
