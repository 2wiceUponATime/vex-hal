// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! VEXos Task Scheduler Functions

#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_uchar, c_void};

use crate::platform::c_enum;

pub type rmTask_t = unsafe extern "C" fn() -> c_int;
pub type rmTaskSimple_t = unsafe extern "C" fn();
pub type rmTaskArg_t = unsafe extern "C" fn(*mut c_void) -> c_int;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct rmSemaphore {
    pub nOwningTask: u8,
    pub nLockCount: u8,
}

c_enum! {
    pub struct rmTaskStates: c_uchar {
        kTaskUndefined = 0,
        kTaskInitializing = 1,
        kTaskRunning = 2,
        kTaskTimerWait = 3,
        kTaskStop = 4,
        kTaskSemaphoreWait = 5,
        kTaskSemaphoreRunPending = 6,
        kTaskSuspended = 7,
        kTaskInitializeSuspended = 8,
        kTaskRunningSuspended = 9,
        kTaskTimerWaitSuspended = 10,
        kTaskSemaphoreWaitSuspended = 11,
        kTaskComplete = 99
    }
}

mock! {
    Task:
    /// Creates an RTOS task
    ///
    /// Derived from <https://github.com/jpearman/vexcode-lvgllib6_X/blob/8ead3dab49665d4c98225d612672be28c7c2a425/src/v5lvgl.c#L17>
    pub fn vexTaskAdd(
        callback: rmTask_t,
        interval: c_int,
        label: *const c_char,
    );
    pub fn vexTaskAddWithPriority(
        callback: rmTask_t,
        interval: c_int,
        label: *const c_char,
        priority: c_int,
    );
    pub fn vexTaskAddSimple(
        callback: rmTaskSimple_t,
        interval: c_int,
        label: *const c_char,
    );
    pub fn vexTaskAddSimpleWithPriority(
        callback: rmTaskSimple_t,
        interval: c_int,
        label: *const c_char,
        priority: c_int,
    );
    pub fn vexTaskStop(task: rmTask_t);
    pub fn vexTaskSuspend(task: rmTask_t);
    pub fn vexTaskResume(task: rmTask_t);
    pub fn vexTaskSuspendCurrent();
    pub fn vexTaskResumeCurrent();
    pub fn vexTaskProgramSuspend();
    pub fn vexTaskProgramResume();
    pub fn vexTaskPriorityGet(task: rmTask_t) -> i32;
    pub fn vexTaskPrioritySet(task: rmTask_t, priority: i32);
    /// Ticks the VEXos scheduler
    pub fn vexTasksRun() -> i32;
    pub fn vexTaskYield();
    pub fn vexTaskCheckTimeslice();
    pub fn vexTaskGetIndex() -> i32;
    /// Sets the current task to sleep for the specified amount of time (in milliseconds).
    ///
    /// Derived from <https://github.com/jpearman/vexcode-lvgllib6_X/blob/8ead3dab49665d4c98225d612672be28c7c2a425/src/v5lvgl.c#L18>
    pub fn vexTaskSleep(time: u32);
    pub fn vexSemaphoreInit(semaphore: *mut rmSemaphore);
    pub fn vexSemaphoreLock(semaphore: *mut rmSemaphore, time: u32);
    pub fn vexSemaphoreUnlock(semaphore: *mut rmSemaphore);
    pub fn vexSemaphoreGetOwner(semaphore: *mut rmSemaphore) -> c_int;
    pub fn vexTasksDump();
    pub fn vexTaskGetCallback(index: u32) -> *mut c_void;
    pub fn vexTaskWaitForExit(task: rmTask_t);
    pub fn vexTaskStateGet(task: rmTask_t) -> rmTaskStates;
    pub fn vexTaskGetTaskIndex(task: rmTask_t) -> i32;
    pub fn vexTaskStopAll();
    pub fn vexTaskStopAllUser(maintask: rmTask_t);
    pub fn vexTaskRemoveAllUser(maintask: rmTask_t);
    pub fn vexTaskHardwareConcurrency() -> i32;
    pub fn vexTaskCompletionIdSet(id: i32);
    pub fn vexTaskAddWithArg(
        callback: rmTaskArg_t,
        interval: c_int,
        arg: *mut c_void,
        label: *const c_char,
    ) -> i32;
    pub fn vexTaskAddWithPriorityWithArg(
        callback: rmTaskArg_t,
        interval: c_int,
        arg: *mut c_void,
        label: *const c_char,
        priority: c_int,
    ) -> i32;
    pub fn vexTaskStopWithId(task: rmTaskArg_t, callback_id: c_int);
    pub fn vexTaskSuspendWithId(task: rmTaskArg_t, callback_id: c_int);
    pub fn vexTaskResumeWithId(task: rmTaskArg_t, callback_id: c_int);
    pub fn vexTaskPriorityGetWithId(task: rmTaskArg_t, callback_id: c_int) -> i32;
    pub fn vexTaskPrioritySetWithId(task: rmTaskArg_t, callback_id: c_int, priority: i32);
    pub fn vexTaskStateGetWithId(task: rmTaskArg_t, callback_id: c_int) -> rmTaskStates;
    pub fn vexTaskGetTaskIndexWithId(task: rmTaskArg_t, callback_id: c_int) -> i32;
    /// Performs simple background processing.
    pub fn vexBackgroundProcessing();
    pub fn vexTaskWaitForExitWithId(task: rmTaskArg_t, callback_id: c_int);
    pub fn vexTaskGet(index: u32) -> *mut c_void;
    /// Gets a tasks's callback function and internal ID.
    ///
    /// Derived from <https://github.com/jpearman/V5_CompetitionTest/blob/efb7214b983d30d5583e39b343161c26d7187766/include/comp_debug.h#L41>
    pub fn vexTaskGetCallbackAndId(
        index: u32,
        callback_id: *mut c_int,
    ) -> *mut c_void;
    pub fn vexTaskStackSizeGet() -> i32;
    pub fn vexTaskStackDefaultSizeGet() -> i32;
    pub fn vexTaskStackUseGet(index: u32) -> i32;
    pub fn vexTaskStackTopGet(index: u32) -> *mut c_void;
    pub fn vexTaskFree(index: u32) -> i32;
    pub fn vexTaskGetArgs(index: u32, r#type: *mut u16) -> *mut c_void;
    pub fn vexTaskSetArgs(index: u32, args: *mut c_void);
    pub fn vexTaskBreakpointSet(filehash: u32, tag: u32);
    pub fn vexTaskBreakpointDump();
}
