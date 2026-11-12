#![no_std]
#![no_main]

use vex_hal::{
    self as _,
    platform::{STDIO_CHANNEL, vexSerialWriteBuffer},
};

#[vex_hal::main]
fn main() {
    let string = "Hello, World!";
    unsafe {
        vexSerialWriteBuffer(STDIO_CHANNEL, string.as_ptr(), string.len() as u32);
    }
}
