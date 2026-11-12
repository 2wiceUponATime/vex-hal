// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

use core::ffi::{c_char, c_int};

use super::jump_table;

jump_table! {
    /// Creates an RTOS task
    ///
    /// Derived from <https://github.com/jpearman/vexcode-lvgllib6_X/blob/8ead3dab49665d4c98225d612672be28c7c2a425/src/v5lvgl.c#L17>
    0x28 => pub fn vexTaskAdd(callback: unsafe extern "C" fn() -> c_int, interval: c_int, label: *const c_char);

    /// Sets the current task to sleep for the specified amount of time (in milliseconds).
    ///
    /// Derived from <https://github.com/jpearman/vexcode-lvgllib6_X/blob/8ead3dab49665d4c98225d612672be28c7c2a425/src/v5lvgl.c#L18>
    0x6c => pub fn vexTaskSleep(time: u32);
}
