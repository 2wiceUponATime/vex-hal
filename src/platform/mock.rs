// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

extern crate std;

macro_rules! mock {
    {
        $category:ident $(<$($gen:lifetime),+ $(,)?>)?:
        $(
            $(#[$attr:meta])*
            $vis:vis fn $name:ident
                $(<$($fn_gen:lifetime),+ $(,)?>)?
                ($($arg:ident: $ty:ty),* $(,)?) $(-> $ret:ty)?;
        )*
    } => {::pastey::paste! {
        #[derive(Debug)]
        #[allow(non_snake_case)]
        pub enum [<$category Call>]$(<$($gen),+>)? {
            $([<$name:camel>] {
                $($arg: $ty,)*
            }),*
        }

        impl $(<$($gen),+>)? [<$category Call>]$(<$($gen),+>)? {
            pub fn default_ret(&self) -> [<$category Ret>] {
                match self {
                    $(Self::[<$name:camel>] { .. } => {
                        $crate::platform::mock::mock!(
                            @ret [<$category Ret>],
                            Default::default(),
                            $([<$name:camel>]($ret))?
                        )
                    })*
                }
            }
        }

        pub enum [<$category Ret>] {
            $($(
                [<$name:camel>]($ret),
            )?)*
            Unit,
        }

        impl [<$category Ret>] {
            pub fn is_unit(&self) -> bool {
                matches!(self, Self::Unit)
            }

            $($(
                #[allow(non_snake_case)]
                pub fn [<into_ $name>](self) -> Option<$ret> {
                    #[allow(unreachable_patterns)]
                    match self {
                        Self::[<$name:camel>](value) => Some(value),
                        _ => None,
                    }
                }
            )?)*
        }

        pub trait [<$category Fns>] {
            $(
                #[allow(non_snake_case, unused_variables, clippy::too_many_arguments)]
                fn $name $(<$($fn_gen),+>)?(&self, $($arg: $ty),*) $(-> $ret)? {
                    Default::default()
                }
            )*

            fn [<handle_ $category:snake>](&self, call: [<$category Call>]) -> [<$category Ret>] {
                match call {
                    $([<$category Call>]::[<$name:camel>] {
                        $($arg,)*
                    } => {
                        #[allow(unused_variables)]
                        let result = self.$name($($arg),*);
                        $crate::platform::mock::mock!(
                            @ret [<$category Ret>],
                            result,
                            $([<$name:camel>]($ret))?
                        )
                    })*
                }
            }
        }

        $(
            $(#[$attr])*
            #[allow(non_snake_case, unused_variables, clippy::too_many_arguments)]
            /// # Safety
            /// Calls to jumptable functions are unsafe because jumptable functions are owned by
            /// VEXos and we cannot guarantee their safety.
            $vis unsafe fn $name $(<$($fn_gen),+>)?($($arg: $ty),*) $(-> $ret)? {
                let ret = $crate::platform::mock::backend()
                    .[<handle_ $category:snake>](
                        [<$category Call>]::[<$name:camel>] {
                            $($arg,)*
                        }
                    );
                $crate::platform::mock::mock!(@unwrap [<$category Ret>], ret $(, $name $ret)?)
            }
        )*
    }};

    (@unwrap $ty:ident, $ret:ident) => {
        if !$ret.is_unit() {
            panic!(concat!("expected ", stringify!($ty), "::Unit"));
        }
    };
    (@unwrap $ty:ident, $ret:ident, $func:ident $t:ty) => {::pastey::paste! {
        $ret
            .[<into_ $func>]()
            .expect(concat!("expected ", stringify!($ty), "::", stringify!([<$func:camel>])))
    }};

    (@ret $ty:ident, $default:expr,) => {
        $ty::Unit
    };
    (@ret $ty:ident, $default:expr, $variant:ident($ret:ty)) => {
        $ty::$variant($default)
    };
}
use mock;

/// Handle to an opaque [`V5_Device`].
#[allow(non_camel_case_types)]
pub type V5_DeviceT = *mut V5_Device;

/// A device plugged into a smart port.
#[allow(non_camel_case_types)]
pub type V5_Device = *mut core::ffi::c_void;

pub mod abs_enc;
pub mod adi;
pub mod ai_vision;
pub mod arm;
pub mod battery;
pub mod competition;
pub mod controller;
pub mod device;
pub mod display;
pub mod distance;
pub mod event;
pub mod file;
pub mod generic_radio;
pub mod generic_serial;
pub mod gps;
pub mod imu;
pub mod led;
pub mod light_tower;
pub mod magnet;
pub mod motor;
pub mod optical;
pub mod pneumatic;
pub mod range;
pub mod serial;
pub mod sound;
pub mod system;
pub mod task;
pub mod timer;
pub mod touch;
pub mod vision;

use core::{cell::Cell, ptr::NonNull};

pub use abs_enc::*;
pub use adi::*;
pub use ai_vision::*;
pub use arm::*;
pub use battery::*;
pub use competition::*;
pub use controller::*;
pub use device::*;
pub use display::*;
pub use distance::*;
pub use event::*;
pub use file::*;
pub use generic_radio::*;
pub use generic_serial::*;
pub use gps::*;
pub use imu::*;
pub use led::*;
pub use light_tower::*;
pub use magnet::*;
pub use motor::*;
pub use optical::*;
pub use pneumatic::*;
pub use range::*;
pub use serial::*;
pub use sound::*;
pub use system::*;
pub use task::*;
pub use timer::*;
pub use touch::*;
pub use vision::*;

pub fn exit() -> ! {
    std::process::exit(0);
}

pub fn yield_now() {
    std::thread::yield_now();
}

static DEFAULT: DefaultBackend = DefaultBackend;

std::thread_local! {
    static BACKEND: Cell<NonNull<dyn MockBackend>> = const {
        Cell::new(NonNull::from_ref(&DEFAULT))
    };
}

pub fn backend() -> &'static dyn MockBackend {
    // SAFETY: the pointer is either `&DEFAULT` or an override that `with_backend` keeps alive
    // while it's installed.
    unsafe { BACKEND.with(|c| &*c.get().as_ptr()) }
}

pub fn with_backend<R>(b: &dyn MockBackend, f: impl FnOnce() -> R) -> R {
    // SAFETY: We erase `b`'s lifetime to `'static` only to store it in the
    // thread-local (whose type cannot be lifetime-parameterized). The scope
    // guard restores the previous value before `with_backend` returns - on both
    // normal and unwinding exit - so the erased pointer is only ever read by
    // `backend()` during `f()`, while `b` is still borrowed and alive.
    let b: &'static dyn MockBackend = unsafe { core::mem::transmute(b) };
    let prev = BACKEND.with(|c| c.replace(NonNull::from_ref(b)));
    let _restore = scopeguard::guard((), |_| BACKEND.with(|c| c.set(prev)));
    f()
}

/// A backend that return [`Default::default()`] for all calls.
pub struct DefaultBackend;

impl MockBackend for DefaultBackend {}

/// A backend that logs all calls to stdout and returns [`Default::default()`].
pub struct PrintBackend;

pub struct Fns<T>(pub T);

macro_rules! mock_backend {
    ($($category:ident),* $(,)?) => {::pastey::paste! {
        $(const _: () = {
            #[allow(unused_imports)]
            use $crate::platform::mock::$category as _;
        };)*

        pub trait MockBackend {
            $(fn [<handle_ $category>](
                &self,
                call: [<$category:camel Call>],
            ) -> [<$category:camel Ret>] {
                call.default_ret()
            })+
        }

        impl MockBackend for PrintBackend {
            $(fn [<handle_ $category>](
                &self,
                call: [<$category:camel Call>],
            ) -> [<$category:camel Ret>] {
                $crate::println!("{call:?}");
                $crate::platform::DefaultBackend.[<handle_ $category>](call)
            })+
        }

        impl<T> MockBackend for Fns<T> where T: $([<$category:camel Fns>] +)* {
            $(fn [<handle_ $category>](
                &self,
                call: [<$category:camel Call>],
            ) -> [<$category:camel Ret>] {
                self.0.[<handle_ $category>](call)
            })+
        }
    }};
}

mock_backend!(
    abs_enc,
    adi,
    ai_vision,
    arm,
    battery,
    competition,
    controller,
    device,
    display,
    distance,
    event,
    file,
    generic_radio,
    generic_serial,
    gps,
    imu,
    led,
    light_tower,
    magnet,
    motor,
    optical,
    pneumatic,
    range,
    serial,
    sound,
    system,
    task,
    timer,
    touch,
    vision,
);
