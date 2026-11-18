// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! VEXos Task Scheduler Functions

#![allow(non_camel_case_types)]

use core::ffi::{c_char, c_int, c_uchar, c_void};

use crate::platform::{c_enum, jump_table};

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

jump_table! {
    /// Creates an RTOS task
    ///
    /// Derived from <https://github.com/jpearman/vexcode-lvgllib6_X/blob/8ead3dab49665d4c98225d612672be28c7c2a425/src/v5lvgl.c#L17>
    0x028 => pub fn vexTaskAdd(
        callback: rmTask_t,
        interval: c_int,
        label: *const c_char,
    );
    0x02c => pub fn vexTaskAddWithPriority(
        callback: rmTask_t,
        interval: c_int,
        label: *const c_char,
        priority: c_int,
    );
    0x030 => pub fn vexTaskAddSimple(
        callback: rmTaskSimple_t,
        interval: c_int,
        label: *const c_char,
    );
    0x034 => pub fn vexTaskAddSimpleWithPriority(
        callback: rmTaskSimple_t,
        interval: c_int,
        label: *const c_char,
        priority: c_int,
    );
    0x038 => pub fn vexTaskStop(task: rmTask_t);
    0x03c => pub fn vexTaskSuspend(task: rmTask_t);
    0x040 => pub fn vexTaskResume(task: rmTask_t);
    0x044 => pub fn vexTaskSuspendCurrent();
    0x048 => pub fn vexTaskResumeCurrent();
    0x04c => pub fn vexTaskProgramSuspend();
    0x050 => pub fn vexTaskProgramResume();
    0x054 => pub fn vexTaskPriorityGet(task: rmTask_t) -> i32;
    0x058 => pub fn vexTaskPrioritySet(task: rmTask_t, priority: i32);
    /// Ticks the VEXos scheduler
    ///
    /// The EXP Brain has a single Cortex-M7 core that is shared between VEXos and user code.
    /// User code does not run on a dedicated processor; it runs as a task inside the same
    /// cooperative scheduler as VEXos's own background tasks (the SDK's startup code registers
    /// the user's `main` as a task with a 2mS interval). Those background tasks handle device
    /// reads, serial flushing, USB, and other operations that must occur alongside user code,
    /// so they only make progress when the running task yields to the scheduler.
    ///
    /// This function runs the scheduler from a context that is not itself a scheduled task. For
    /// example, the SDK's `_exit` calls it in a loop when no task is currently running.
    0x05c => pub fn vexTasksRun() -> i32;
    0x060 => pub fn vexTaskYield();
    0x064 => pub fn vexTaskCheckTimeslice();
    0x068 => pub fn vexTaskGetIndex() -> i32;
    /// Sets the current task to sleep for the specified amount of time (in milliseconds).
    ///
    /// Derived from <https://github.com/jpearman/vexcode-lvgllib6_X/blob/8ead3dab49665d4c98225d612672be28c7c2a425/src/v5lvgl.c#L18>
    0x06c => pub fn vexTaskSleep(time: u32);
    0x070 => pub fn vexSemaphoreInit(semaphore: *mut rmSemaphore);
    0x074 => pub fn vexSemaphoreLock(semaphore: *mut rmSemaphore, time: u32);
    0x078 => pub fn vexSemaphoreUnlock(semaphore: *mut rmSemaphore);
    0x07c => pub fn vexSemaphoreGetOwner(semaphore: *mut rmSemaphore) -> c_int;
    0x080 => pub fn vexTasksDump();
    0x084 => pub fn vexTaskGetCallback(index: u32) -> *mut c_void;
    0x088 => pub fn vexTaskWaitForExit(task: rmTask_t);
    0x08c => pub fn vexTaskStateGet(task: rmTask_t) -> rmTaskStates;
    0x090 => pub fn vexTaskGetTaskIndex(task: rmTask_t) -> i32;
    0x094 => pub fn vexTaskStopAll();
    0x098 => pub fn vexTaskStopAllUser(maintask: rmTask_t);
    0x09c => pub fn vexTaskRemoveAllUser(maintask: rmTask_t);
    0x140 => pub fn vexTaskHardwareConcurrency() -> i32;
    0x144 => pub fn vexTaskCompletionIdSet(id: i32);
    0xf50 => pub fn vexTaskAddWithArg(
        callback: rmTaskArg_t,
        interval: c_int,
        arg: *mut c_void,
        label: *const c_char,
    ) -> i32;
    0xf54 => pub fn vexTaskAddWithPriorityWithArg(
        callback: rmTaskArg_t,
        interval: c_int,
        arg: *mut c_void,
        label: *const c_char,
        priority: c_int,
    ) -> i32;
    0xf58 => pub fn vexTaskStopWithId(task: rmTaskArg_t, callback_id: c_int);
    0xf5c => pub fn vexTaskSuspendWithId(task: rmTaskArg_t, callback_id: c_int);
    0xf60 => pub fn vexTaskResumeWithId(task: rmTaskArg_t, callback_id: c_int);
    0xf64 => pub fn vexTaskPriorityGetWithId(task: rmTaskArg_t, callback_id: c_int) -> i32;
    0xf68 => pub fn vexTaskPrioritySetWithId(task: rmTaskArg_t, callback_id: c_int, priority: i32);
    0xf6c => pub fn vexTaskStateGetWithId(task: rmTaskArg_t, callback_id: c_int) -> rmTaskStates;
    0xf70 => pub fn vexTaskGetTaskIndexWithId(task: rmTaskArg_t, callback_id: c_int) -> i32;
    /// Unimplemented as of VEXos 1.0.8.
    ///
    /// Performs simple background processing.
    0xf74 => pub fn vexBackgroundProcessing();
    0xf78 => pub fn vexTaskWaitForExitWithId(task: rmTaskArg_t, callback_id: c_int);
    0xf7c => pub fn vexTaskGet(index: u32) -> *mut c_void;
    /// Gets a tasks's callback function and internal ID.
    ///
    /// Derived from <https://github.com/jpearman/V5_CompetitionTest/blob/efb7214b983d30d5583e39b343161c26d7187766/include/comp_debug.h#L41>
    0xf80 => pub fn vexTaskGetCallbackAndId(
        index: u32,
        callback_id: *mut c_int,
    ) -> *mut c_void;
    0x148 => pub fn vexTaskStackSizeGet() -> i32;
    0x14c => pub fn vexTaskStackDefaultSizeGet() -> i32;
    0x150 => pub fn vexTaskStackUseGet(index: u32) -> i32;
    0x154 => pub fn vexTaskStackTopGet(index: u32) -> *mut c_void;
    0x158 => pub fn vexTaskFree(index: u32) -> i32;
    0x15c => pub fn vexTaskGetArgs(index: u32, r#type: *mut u16) -> *mut c_void;
    0x160 => pub fn vexTaskSetArgs(index: u32, args: *mut c_void);
    0x0cc => pub fn vexTaskBreakpointSet(filehash: u32, tag: u32);
    0x0d0 => pub fn vexTaskBreakpointDump();
}
