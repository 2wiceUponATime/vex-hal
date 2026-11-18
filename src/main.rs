// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

#![no_std]
#![no_main]

use vex_hal::prelude::*;

#[vex_hal::main]
fn main(_peripherals: Peripherals) {
    println!("Hello, World!");
}
