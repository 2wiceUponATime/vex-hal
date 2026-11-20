// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! VEXos System Functions

use core::ffi::VaList;
use core::ffi::c_char;
use core::ffi::c_void;

use crate::platform::JUMP_TABLE_START;

use crate::platform::jump_table;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct time {
    /// Hours
    pub ti_hour: u8,
    /// Minutes
    pub ti_min: u8,
    /// Seconds
    pub ti_sec: u8,
    /// Hundredths of seconds
    pub ti_hund: u8,
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct date {
    /// Year - 1980
    pub da_year: u16,
    /// Day of the month
    pub da_day: u8,
    /// Month (1 = Jan)
    pub da_mon: u8,
}

jump_table! {
    0x998 => pub fn vexScratchMemoryLock() -> bool;
    0x99c => pub fn vexScratchMemoryUnlock();
    0x0c8 => pub fn vexBreak(filehash: u32, tag: u32);
    0x0f4 => pub fn vex_vsprintf(out: *mut c_char, format: *const c_char, args: VaList);
    0x0f8 => pub fn vex_vsnprintf(
        out: *mut c_char,
        max_len: u32,
        format: *const c_char,
        args: VaList,
    );
    /// Returns the time since program start with millisecond precision.
    0x118 => pub fn vexSystemTimeGet() -> u32;
    0x11c => pub fn vexGettime(time: *mut time);
    0x120 => pub fn vexGetdate(date: *mut date);
    0x124 => pub fn vexSystemMemoryDump();
    0x128 => pub fn vexSystemDigitalIO(pin: u32, value: u32);
    0x12c => pub fn vexSystemStartupOptions() -> u32;
    /// Exits the current user program, returning to the main program screen.
    0x130 => pub fn vexSystemExitRequest();
    /// Returns the time since program start with microsecond precision.
    0x134 => pub fn vexSystemHighResTimeGet() -> u64;
    /// Returns the time since power on with microsecond precision.
    0x138 => pub fn vexSystemPowerupTimeGet() -> u64;
    /// Returns the address in memory of a linked file to the current user program, or `0`, if no file is linked.
    ///
    /// VEXos's internal filesystem structure allows user programs to "link" other external binary packages =>
    /// similar
    0x13c => pub fn vexSystemLinkAddrGet() -> u32;
    0x174 => pub fn vexSystemUsbStatus() -> u32;
    /// Unimplemented as of VEXos 1.0.8.
    0x8c0 => pub fn vexSystemTimerStop();
    /// Unimplemented as of VEXos 1.0.8.
    0x8c4 => pub fn vexSystemTimerClearInterrupt();
    /// Unimplemented as of VEXos 1.0.8.
    0x8c8 => pub fn vexSystemTimerReinitForRtos(
        priority: u32,
        handler: unsafe extern "C" fn(data: *mut c_void),
    ) -> i32;
    /// Unimplemented as of VEXos 1.0.8.
    0x8cc => pub fn vexSystemApplicationIRQHandler(ulICCIAR: u32);
    /// Unimplemented as of VEXos 1.0.8.
    0x8d0 => pub fn vexSystemWatchdogReinitRtos() -> i32;
    /// Unimplemented as of VEXos 1.0.8.
    0x8d4 => pub fn vexSystemWatchdogGet() -> u32;
    /// Unimplemented as of VEXos 1.0.8.
    0x910 => pub fn vexSystemBoot();
    /// Unimplemented as of VEXos 1.0.8.
    0x914 => pub fn vexSystemUndefinedException();
    /// Unimplemented as of VEXos 1.0.8.
    0x918 => pub fn vexSystemFIQInterrupt();
    /// Unimplemented as of VEXos 1.0.8.
    0x91c => pub fn vexSystemIRQInterrupt();
    /// Unimplemented as of VEXos 1.0.8.
    0x920 => pub fn vexSystemSWInterrupt();
    /// Unimplemented as of VEXos 1.0.8.
    0x924 => pub fn vexSystemDataAbortInterrupt();
    /// Unimplemented as of VEXos 1.0.8.
    0x928 => pub fn vexSystemPrefetchAbortInterrupt();
}

/// special use only ! Talk to James.
///
/// # Safety
///
/// `ptr` must be either null or valid for writes of `*mut c_void`, and properly aligned.
#[allow(non_snake_case)]
pub unsafe fn vexScratchMemoryPtr(ptr: *mut *mut c_void) -> i32 {
    if ptr.is_null() {
        return 0;
    }
    // SAFETY: the scratch memory size and pointer are located at fixed, aligned offsets in the
    // jump table, which is always mapped and readable.
    let size = unsafe { core::ptr::read_volatile((JUMP_TABLE_START + 0x1c) as *const i32) };
    let scratch = if size > 0 {
        // SAFETY: see above.
        unsafe { core::ptr::read_volatile((JUMP_TABLE_START + 0x18) as *const *mut c_void) }
    } else {
        core::ptr::null_mut()
    };
    // SAFETY: `ptr` is non-null, and the caller guarantees it is safe to write to.
    unsafe { *ptr = scratch };
    size
}

/// Returns the current VEXos firmware version, encoded as big-endian bytes.
///
/// # Safety
///
/// This function is sound to call; the version word lives at a fixed, aligned, always-mapped
/// offset in the jump table. It is marked `unsafe` only for signature parity with `vex-sdk`.
#[allow(non_snake_case)]
pub unsafe fn vexSystemVersion() -> u32 {
    // SAFETY: the version word lives at a fixed, aligned offset in the jump
    // table, which is always mapped and readable.
    unsafe { core::ptr::read_volatile((JUMP_TABLE_START + 0x1000) as *const u32) }
}
