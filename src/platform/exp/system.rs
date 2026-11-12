// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

use super::jump_table;

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
    /// special use only ! Talk to James.
    #[cfg(any())]
    0x000 => pub fn vexScratchMemoryPtr(ptr: *mut *mut core::ffi::c_void) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexScratchMemoryLock() -> bool;
    #[cfg(any())]
    0x000 => pub fn vexScratchMemoryUnlock();
    /// Returns the time since program start with millisecond precision.
    #[cfg(any())]
    0x000 => pub fn vexSystemTimeGet() -> u32;
    #[cfg(any())]
    0x000 => pub fn vexGettime() -> time;
    #[cfg(any())]
    0x000 => pub fn vexGetdate() -> date;
    #[cfg(any())]
    0x000 => pub fn vexSystemMemoryDump();
    #[cfg(any())]
    0x000 => pub fn vexSystemDigitalIO(pin: u32, value: u32);
    #[cfg(any())]
    0x000 => pub fn vexSystemStartupOptions() -> u32;
    /// Exits the current user program, returning to the main program screen.
    0x130 => pub fn vexSystemExitRequest();
    /// Returns the time since program start with microsecond precision.
    #[cfg(any())]
    0x000 => pub fn vexSystemHighResTimeGet() -> u64;
    /// Returns the time since power on with microsecond precision.
    #[cfg(any())]
    0x000 => pub fn vexSystemPowerupTimeGet() -> u64;
    /// Returns the address in memory of a linked file to the current user program, or `0`, if no file is linked.
    ///
    /// VEXos's internal filesystem structure allows user programs to "link" other external binary packages =>
    /// similar
    #[cfg(any())]
    0x000 => pub fn vexSystemLinkAddrGet() -> u32;
    #[cfg(any())]
    0x000 => pub fn vexSystemUsbStatus() -> u32;
    #[cfg(any())]
    0x000 => pub fn vexSystemTimerStop();
    #[cfg(any())]
    0x000 => pub fn vexSystemTimerClearInterrupt();
    #[cfg(any())]
    0x000 => pub fn vexSystemTimerReinitForRtos(priority: u32, handler: unsafe extern "C" fn(data: *mut c_void)) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexSystemApplicationIRQHandler(ulICCIAR: u32);
    #[cfg(any())]
    0x000 => pub fn vexSystemWatchdogReinitRtos() -> i32;
    #[cfg(any())]
    0x000 => pub fn vexSystemWatchdogGet() -> u32;
    #[cfg(any())]
    0x000 => pub fn vexSystemBoot();
    #[cfg(any())]
    0x000 => pub fn vexSystemUndefinedException();
    #[cfg(any())]
    0x000 => pub fn vexSystemFIQInterrupt();
    #[cfg(any())]
    0x000 => pub fn vexSystemIRQInterrupt();
    #[cfg(any())]
    0x000 => pub fn vexSystemSWInterrupt();
    #[cfg(any())]
    0x000 => pub fn vexSystemDataAbortInterrupt();
    #[cfg(any())]
    0x000 => pub fn vexSystemPrefetchAbortInterrupt();
}
