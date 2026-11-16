// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

use core::ffi::c_void;

use crate::platform::jump_table;

jump_table! {
    0x1cc => pub fn vexDeviceTimerSet(
        index: i32,
        callback: unsafe extern "C" fn(*mut c_void),
        delay: u32,
    );
    0x1d0 => pub fn vexDeviceTimerSetWithArg(
        index: i32,
        callback: unsafe extern "C" fn(*mut c_void),
        delay: u32,
        arg: *mut c_void,
    );
    0x1d4 => pub fn vexDeviceTimerDump();
}
