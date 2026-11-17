// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

#![no_std]
#![no_main]

use vex_hal::{platform::vexTaskSleep, println};

#[vex_hal::main]
fn main() {
    println!("Hello, World!");
    loop {
        unsafe { vexTaskSleep(1000) };
    }
}
