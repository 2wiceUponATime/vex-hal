// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

use core::ffi::c_void;


mock! {
    pub fn vexEventBroadcastAndWait(index: u32, timeout: i32);
    pub fn vexEventBroadcast(index: u32);
    pub fn vexEventAdd(index: u32, mask: u32, callback: unsafe extern "C" fn());
    pub fn vexEventUserIndexGet() -> i32;
    pub fn vexEventAddWithArg(
        index: u32,
        mask: u32,
        callback: unsafe extern "C" fn(*mut c_void),
        arg: *mut c_void,
    );
    pub fn vexEventsCleanup();
    pub fn vexEventsDump();
    pub fn vexEventGetArg(event: *mut c_void) -> *mut c_void;
    pub fn vexEventsGetMax() -> i32;
    pub fn vexEventsGetCount() -> i32;
}
