// SPDX-FileCopyrightText: 2023 Gavin Niederman
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! Asynchronous multitasking.
//!
//! This module provides utilities for working with asynchronous tasks.
//!
//! A *task* is a light weight, non-blocking unit of execution. Tasks allow you to cooperatively
//! perform work in the background without blocking other code from running.
//!
//! - Tasks are **light weight**. Because tasks are scheduled and managed by vex-hal, creating new
//!   tasks or switching between tasks does not require a context switch and has fairly low
//!   overhead. Creating, running, and destroying large numbers of tasks is relatively cheap in
//!   comparison to traditional threads.
//!
//! - Tasks are scheduled **cooperatively**. Most operating system threads implement *preemptive
//!   multitasking*. This is a scheduling technique where the operating system allows each thread to
//!   run for a period of time, and then forcibly preempts it, temporarily pausing that thread and
//!   switching to another. Tasks, on the other hand, implement *cooperative multitasking*. In
//!   cooperative multitasking, a task will run until it voluntarily yields using an `await` point,
//!   giving control back to the vex-hal runtime's scheduler. When a task yields by `await`ing
//!   something, the vex-hal runtime switches to executing a different task.
//!
//! - Tasks are **non-blocking**. Typically, when an OS thread performs I/O or must synchronize with
//!   another thread, it *blocks*, allowing the OS to schedule another thread. When a task cannot
//!   continue executing, it should yield instead, allowing the vex-hal runtime to schedule another
//!   task in its place. Tasks should generally not perform operations that could block the CPU for
//!   a long period of time without an `await` point, as this would prevent other tasks from
//!   executing as well. This includes situations involving long-running "tight loops" (`loop {}`)
//!   without `await` points.
//!
//! # Spawning Tasks
//!
//! Perhaps the most important function in this module is [`spawn`]. This function can be thought of
//! as an async equivalent to the standard library’s `thread::spawn`. It takes
//! an `async` block or other [future](core::future), and creates a new task that runs it
//! concurrently in the background.
//!
//! If a task must outlive the scope it was spawned in, you can
//! [`detach`](Task::detach) it. This lets the task run in the background beyond its
//! current scope. When we `detach` a task, we lose its [`Task`] handle and therefore have no way
//! to `await` its output. As a result, detached tasks may run forever with no way of being
//! stopped.
//!
//! More complex use-cases may require you to hold ownership of shared state *across*
//! `await`-points. In these cases, a simple `Rc<RefCell<T>>` will not suffice, since another
//! running task may claim ownership of the data, which would cause the program to panic. Doing this
//! effectively requires the use of a *synchronization primitive* like a `Mutex` or `RwLock` to
//! manage safe access to shared state across multiple running tasks.

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
