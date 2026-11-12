// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

#[cfg(feature = "platform-exp")]
mod exp;
#[cfg(feature = "platform-exp")]
pub use exp::*;

#[cfg(feature = "platform-iq2")]
mod iq2;
#[cfg(feature = "platform-iq2")]
pub use iq2::*;

#[cfg(feature = "platform-v5")]
mod v5;
#[cfg(feature = "platform-v5")]
pub use v5::*;

#[cfg(any(feature = "platform-exp", feature = "platform-iq2"))]
macro_rules! jump_table {
    {
        $(
            $(#[$attr:meta])*
            $offset:literal => $vis:vis fn $name:ident($($arg:ident: $ty:ty),* $(,)?) $(-> $ret:ty)?;
        )*
    } => {
        $(
            $(#[$attr])*
            #[allow(non_snake_case)]
            /// # Safety
            /// Calls to jumptable functions are unsafe because jumptable functions are owned by
            /// VEXos and we cannot guarantee their safety.
            $vis unsafe fn $name($($arg: $ty),*) $(-> $ret)? {
                unsafe {
                    let f = *(($crate::platform::JUMP_TABLE_START + $offset)
                        as *const unsafe extern "C" fn($($ty),*) $(-> $ret)?);
                    f($($arg),*)
                }
            }
        )*
    };
}

#[cfg(any(feature = "platform-exp", feature = "platform-iq2"))]
use jump_table;

pub const STDIO_CHANNEL: u32 = 1;
