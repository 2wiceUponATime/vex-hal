// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

pub use vex_sdk_jumptable::*;

pub fn exit() -> ! {
    unsafe { vexSystemExitRequest() };
    loop {}
}

pub fn yield_now() {
    unsafe { vexTasksRun() };
}
