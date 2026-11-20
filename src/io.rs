// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

extern crate alloc;

use core::fmt::{self, Write};

#[cfg(not(feature = "platform-mock"))]
use crate::platform::{STDIO_CHANNEL, vexSerialWriteBuffer};

pub struct Stdout;

impl fmt::Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        #[cfg(not(feature = "platform-mock"))]
        if unsafe { vexSerialWriteBuffer(STDIO_CHANNEL, s.as_ptr(), s.len() as u32) } >= 0 {
            Ok(())
        } else {
            Err(fmt::Error)
        }
        #[cfg(feature = "platform-mock")]
        {
            extern crate std;
            std::print!("{s}");
            Ok(())
        }
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let _ = Stdout.write_fmt(args);
}

/// Prints to the standard output.
///
/// Equivalent to the [`println!`](crate::println) macro except that a newline is not printed at
/// the end of the message.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::io::_print(::core::format_args!($($arg)*));
    };
}

/// Prints to the standard output, with a newline.
///
/// This macro uses the same syntax as [`format`](alloc::format), but writes to the standard output
/// instead. See [`core::fmt`] for more information.
#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n");
    };
    ($($arg:tt)*) => {
        $crate::print!("{}\n", ::core::format_args!($($arg)*));
    };
}
