// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

use core::ffi::c_void;

use crate::platform::jump_table;

jump_table! {
    0x0a0 => pub fn vexEventBroadcastAndWait(index: u32, timeout: i32);
    0x0a4 => pub fn vexEventBroadcast(index: u32);
    0x0a8 => pub fn vexEventAdd(index: u32, mask: u32, callback: unsafe extern "C" fn());
    0x0ac => pub fn vexEventUserIndexGet() -> i32;
    0x0b0 => pub fn vexEventAddWithArg(
        index: u32,
        mask: u32,
        callback: unsafe extern "C" fn(*mut c_void),
        arg: *mut c_void,
    );
    0x0b4 => pub fn vexEventsCleanup();
    0x0b8 => pub fn vexEventsDump();
    0x0bc => pub fn vexEventGetArg(event: *mut c_void) -> *mut c_void;
    0x0c0 => pub fn vexEventsGetMax() -> i32;
    0x0c4 => pub fn vexEventsGetCount() -> i32;
}
