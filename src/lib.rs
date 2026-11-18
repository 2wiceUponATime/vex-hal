// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! # vex-hal
//!
//! A Rust runtime for VEX EXP and V5 robots.
//!
//! # Example
//!
//! ```
//! #![no_std]
//! #![no_main]
//!
//! use vex_hal::prelude::*;
//!
//! #[vex_hal::main]
//! async fn main(_peripherals: Peripherals) {
//!     println!("Hello, World!")
//! }
//! ```

#![no_std]

use core::panic::PanicInfo;
pub use vex_hal_macros::main;

mod allocator;
pub mod io;
pub mod math;
pub mod peripherals;
pub mod platform;
pub mod prelude;
pub mod runtime;
pub mod thread;
pub mod time;

unsafe extern "C" {
    #[cfg(feature = "platform-v5")]
    static __stack_top: u8;
    static __bss_start: u8;
    static __bss_end: u8;
    static mut __heap_start: u8;
    static mut __heap_end: u8;
}

unsafe extern "Rust" {
    fn __vex_hal_main();
}

/// V5 entry point. User code gets its own CPU, so we set up our own stack and
/// never return to VEXos.
#[cfg(feature = "platform-v5")]
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".boot")]
#[instruction_set(arm::a32)]
unsafe extern "C" fn _boot() -> ! {
    core::arch::naked_asm!(
        "ldr sp, ={stack_top}",
        "ldr r0, ={bss_start}",
        "ldr r1, ={bss_end}",
        "movs r2, #0",
        "1:",
        "cmp r0, r1",
        "bhs 2f",
        "str r2, [r0], #4",
        "b 1b",
        "2:",
        "blx {start}",
        stack_top = sym __stack_top,
        bss_start = sym __bss_start,
        bss_end = sym __bss_end,
        start = sym start,
    )
}

#[cfg(feature = "platform-v5")]
unsafe extern "C" fn start() -> ! {
    unsafe {
        __vex_hal_main();
        platform::exit();
    }
}

/// EXP/IQ2 entry point, mirroring the SDK's `vexStartup`: VEXos calls this as a
/// normal function on its own stack. We clear BSS in assembly (before any Rust
/// code can observe it), then tail-call [`init`], which returns directly to
/// VEXos.
#[cfg(any(feature = "platform-exp", feature = "platform-iq2"))]
#[unsafe(naked)]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".boot")]
unsafe extern "C" fn _boot() -> core::ffi::c_int {
    core::arch::naked_asm!(
        "ldr r0, ={bss_start}",
        "ldr r1, ={bss_end}",
        "movs r2, #0",
        "1:",
        "cmp r0, r1",
        "bhs 2f",
        "str r2, [r0], #4",
        "b 1b",
        "2:",
        "b {init}", // tail call: init returns straight to VEXos
        bss_start = sym __bss_start,
        bss_end = sym __bss_end,
        init = sym init,
    )
}

/// Registers the user program as the `MAIN` task with
/// `vexTaskAdd(start, 2, "MAIN")` and returns 0 so the VEXos scheduler can run it.
#[cfg(any(feature = "platform-exp", feature = "platform-iq2"))]
unsafe extern "C" fn init() -> core::ffi::c_int {
    unsafe { platform::vexTaskAdd(start, 2, c"MAIN".as_ptr()) };
    0
}

#[cfg(any(feature = "platform-exp", feature = "platform-iq2"))]
unsafe extern "C" fn start() -> core::ffi::c_int {
    unsafe {
        __vex_hal_main();
        platform::exit();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    platform::exit();
}
