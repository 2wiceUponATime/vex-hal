// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! VEXos Task Scheduler Functions

use core::ffi::{c_char, c_int, c_void};

use crate::platform::jump_table;

jump_table! {
    /// Creates an RTOS task
    ///
    /// Derived from <https://github.com/jpearman/vexcode-lvgllib6_X/blob/8ead3dab49665d4c98225d612672be28c7c2a425/src/v5lvgl.c#L17>
    0x028 => pub fn vexTaskAdd(
        callback: unsafe extern "C" fn() -> c_int,
        interval: c_int,
        label: *const c_char,
    );
    /// Gets a tasks's callback function and internal ID.
    ///
    /// Derived from <https://github.com/jpearman/V5_CompetitionTest/blob/efb7214b983d30d5583e39b343161c26d7187766/include/comp_debug.h#L41>
    0xf80 => pub fn vexTaskGetCallbackAndId(
        index: u32,
        callback_id: *mut c_int,
    ) -> *mut c_void;
    /// Sets the current task to sleep for the specified amount of time (in milliseconds).
    ///
    /// Derived from <https://github.com/jpearman/vexcode-lvgllib6_X/blob/8ead3dab49665d4c98225d612672be28c7c2a425/src/v5lvgl.c#L18>
    0x06c => pub fn vexTaskSleep(time: u32);
    /// Performs simple background processing.
    0xf74 => pub fn vexBackgroundProcessing();
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
    0x05c => pub fn vexTasksRun();
    0x060 => pub fn vexTaskYield();
}
