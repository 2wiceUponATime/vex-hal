// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! VEXos System Functions

use core::ffi::VaList;
use core::ffi::c_char;
use core::ffi::c_void;

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

mock! {
    /// special use only ! Talk to James.
    pub fn vexScratchMemoryPtr(ptr: *mut *mut c_void) -> i32;
    pub fn vexScratchMemoryLock() -> bool;
    pub fn vexScratchMemoryUnlock();
    pub fn vexBreak(filehash: u32, tag: u32);
    pub fn vex_vsprintf(out: *mut c_char, format: *const c_char, args: VaList);
    pub fn vex_vsnprintf(
        out: *mut c_char,
        max_len: u32,
        format: *const c_char,
        args: VaList,
    );
    /// Returns the time since program start with millisecond precision.
    pub fn vexSystemTimeGet() -> u32;
    pub fn vexGettime(time: *mut time);
    pub fn vexGetdate(date: *mut date);
    pub fn vexSystemMemoryDump();
    pub fn vexSystemDigitalIO(pin: u32, value: u32);
    pub fn vexSystemStartupOptions() -> u32;
    /// Exits the current user program, returning to the main program screen.
    pub fn vexSystemExitRequest();
    /// Returns the time since program start with microsecond precision.
    pub fn vexSystemHighResTimeGet() -> u64;
    /// Returns the time since power on with microsecond precision.
    pub fn vexSystemPowerupTimeGet() -> u64;
    /// Returns the address in memory of a linked file to the current user program, or `0`, if no file is linked.
    ///
    /// VEXos's internal filesystem structure allows user programs to "link" other external binary packages =>
    /// similar
    pub fn vexSystemLinkAddrGet() -> u32;
    pub fn vexSystemUsbStatus() -> u32;
    pub fn vexSystemTimerStop();
    pub fn vexSystemTimerClearInterrupt();
    pub fn vexSystemTimerReinitForRtos(
        priority: u32,
        handler: unsafe extern "C" fn(data: *mut c_void),
    ) -> i32;
    pub fn vexSystemApplicationIRQHandler(ulICCIAR: u32);
    pub fn vexSystemWatchdogReinitRtos() -> i32;
    pub fn vexSystemWatchdogGet() -> u32;
    pub fn vexSystemBoot();
    pub fn vexSystemUndefinedException();
    pub fn vexSystemFIQInterrupt();
    pub fn vexSystemIRQInterrupt();
    pub fn vexSystemSWInterrupt();
    pub fn vexSystemDataAbortInterrupt();
    pub fn vexSystemPrefetchAbortInterrupt();
}
