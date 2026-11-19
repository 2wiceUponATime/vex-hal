// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

use core::ffi::c_void;

mock! {
    pub trait MockTimer;
    pub fn vexDeviceTimerSet(
        index: i32,
        callback: unsafe extern "C" fn(*mut c_void),
        delay: u32,
    );
    pub fn vexDeviceTimerSetWithArg(
        index: i32,
        callback: unsafe extern "C" fn(*mut c_void),
        delay: u32,
        arg: *mut c_void,
    );
    pub fn vexDeviceTimerDump();
}
