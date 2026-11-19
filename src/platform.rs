// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

#[cfg(feature = "platform-exp")]
mod exp;
#[cfg(feature = "platform-exp")]
pub use exp::*;

#[cfg(feature = "platform-v5")]
mod v5;
#[cfg(feature = "platform-v5")]
pub use v5::*;

#[cfg(feature = "platform-mock")]
mod mock;
#[cfg(feature = "platform-mock")]
pub use mock::*;

#[cfg(feature = "platform-exp")]
macro_rules! jump_table {
    {
        $(
            $(#[$attr:meta])*
            $offset:literal
                => $vis:vis fn $name:ident($($arg:ident: $ty:ty),* $(,)?) $(-> $ret:ty)?;
        )*
    } => {
        const _: () = {
            #[deny(unreachable_patterns)]
            const fn _check(x: usize) {
                match x {
                    $($offset => {})*
                    _ => {}
                }
            }
        };
        $(
            $(#[$attr])*
            #[allow(non_snake_case, clippy::too_many_arguments)]
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

#[cfg(any(feature = "platform-exp", feature = "platform-mock"))]
macro_rules! c_enum {
    {
        $(#[$meta:meta])*
        $vis:vis struct $name:ident: $inner:ty {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident = $value:expr
            ),+ $(,)?
        }
    } => {
        #[repr(transparent)]
        #[derive(Default, Clone, Copy, PartialEq, Eq)]
        $(#[$meta])*
        #[allow(non_camel_case_types)]
        $vis struct $name(pub $inner);

        #[allow(non_upper_case_globals)]
        impl $name {
            $(
                $(#[$variant_meta])*
                pub const $variant: Self = Self($value);
            )+
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match *self {
                    $(Self::$variant => f.write_str(stringify!($variant)),)+
                    Self(i) => f.debug_tuple(stringify!($name)).field(&i).finish(),
                }
            }
        }
    };
}

#[cfg(feature = "platform-exp")]
use {c_enum, jump_table};

#[cfg(feature = "platform-mock")]
use c_enum;

pub const STDIO_CHANNEL: u32 = 1;
/// Constant on all platforms
pub const V5_MAX_DEVICE_PORTS: usize = 32;

#[cfg(not(feature = "platform-mock"))]
macro_rules! platform_local {
    (
        $(
            $(#[$meta:meta])*
            $vis:vis fn $name:ident() -> &$t:ty = $init:expr;
        )*
    ) => {
        $(
            $(#[$meta])*
            $vis fn $name<R>(f: impl FnOnce(&$t) -> R) -> R {
                static S: $crate::single_threaded::SingleThreaded<$t> = unsafe {
                    $crate::single_threaded::SingleThreaded::new($init)
                };
                f(&S)
            }
        )*
    };
}
#[cfg(feature = "platform-mock")]
macro_rules! platform_local {
    (
        $(
            $(#[$meta:meta])*
            $vis:vis fn $name:ident() -> &$t:ty = $init:expr;
        )*
    ) => {
        $(
            $(#[$meta])*
            $vis fn $name<R>(f: impl FnOnce(&$t) -> R) -> R {
                extern crate std;
                std::thread_local! {
                    static S: $t = const { $init };
                }
                S.with(f)
            }
        )*
    };
}
pub(crate) use platform_local;
