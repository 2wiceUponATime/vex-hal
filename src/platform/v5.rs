// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

// `vex-sdk` provides the full SDK API surface (types and `extern "system"` function
// declarations). The matching `#[no_mangle]` implementations that dispatch through the VEXos jump
// table are supplied by `vex-sdk-jumptable`, which is linked in below.
pub use vex_sdk::*;

extern crate vex_sdk_jumptable;

pub const SIG_MAGIC: u32 = V5_SIG_MAGIC;

pub fn exit() -> ! {
    unsafe { vexSystemExitRequest() };
    // Keep yielding to VEXos while it tears us down.
    loop {
        unsafe { vexTaskSleep(1000) };
    }
}

pub fn yield_now() {
    unsafe { vexTasksRun() };
}
