use core::fmt::{self, Write};

use crate::platform::{STDIO_CHANNEL, vexSerialWriteBuffer};

pub struct Stdout;

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if unsafe { vexSerialWriteBuffer(STDIO_CHANNEL, s.as_ptr(), s.len() as u32) } >= 0 {
            Ok(())
        } else {
            Err(fmt::Error)
        }
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let _ = Stdout.write_fmt(args);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::io::_print(::core::format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n");
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", core::format_args!($($arg)*));
    };
}
