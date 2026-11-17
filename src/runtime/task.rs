// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

extern crate alloc;

use alloc::rc::Rc;

use crate::runtime::{executor::EXECUTOR, local::TaskLocalStorage};

pub struct TaskMetadata {
    pub(crate) tls: Rc<TaskLocalStorage>,
}

pub type Task<T> = async_task::Task<T, TaskMetadata>;

pub type FallibleTask<T> = async_task::FallibleTask<T, TaskMetadata>;

pub fn spawn<T>(future: impl Future<Output = T> + 'static) -> Task<T> {
    EXECUTOR.spawn(future)
}

pub fn block_on<T>(future: impl Future<Output = T>) -> T {
    EXECUTOR.block_on(future)
}
