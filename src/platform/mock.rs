// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

extern crate std;

macro_rules! mock {
    {
        $trait_vis:vis trait $trait:ident;
        $(
            $(#[$attr:meta])*
            $vis:vis fn $name:ident($($arg:ident: $ty:ty),* $(,)?) $(-> $ret:ty)?;
        )*
    } => {
        $trait_vis trait $trait {
            $(
                #[allow(non_snake_case, unused_variables, clippy::too_many_arguments)]
                fn $name(&self, $($arg: $ty),*) $(-> $ret)? {
                    Default::default()
                }
            )*
        }
        $(
            $(#[$attr])*
            #[allow(non_snake_case, unused_variables, clippy::too_many_arguments)]
            /// # Safety
            /// Calls to jumptable functions are unsafe because jumptable functions are owned by
            /// VEXos and we cannot guarantee their safety.
            $vis unsafe fn $name($($arg: $ty),*) $(-> $ret)? {
                $crate::platform::mock::backend().$name($($arg),*)
            }
        )*
    };
}

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

pub type DeviceType = V5_DeviceType;

pub fn exit() -> ! {
    std::process::exit(0);
}

pub fn yield_now() {
    std::thread::yield_now();
}

static DEFAULT: DefaultBackend = DefaultBackend;

std::thread_local! {
    static BACKEND: Cell<NonNull<dyn MockBackend>> = const { Cell::new(NonNull::from_ref(&DEFAULT)) };
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

macro_rules! mock_backend {
    ($first:ident $(, $trait:ident)* $(,)?) => {
        pub trait MockBackend: $first $(+ $trait)* {}

        impl $first for DefaultBackend {}
        $(impl $trait for DefaultBackend {})+

        impl<T: $first $(+ $trait)*> MockBackend for T {}
    };
}

mock_backend!(
    MockAbsEnc,
    MockAdi,
    MockAiVision,
    MockArm,
    MockBattery,
    MockCompetition,
    MockController,
    MockDevice,
    MockDisplay,
    MockDistance,
    MockEvent,
    MockFile,
    MockGenericRadio,
    MockGenericSerial,
    MockGps,
    MockImu,
    MockLed,
    MockLightTower,
    MockMagnet,
    MockMotor,
    MockOptical,
    MockPneumatic,
    MockRange,
    MockSerial,
    MockSound,
    MockSystem,
    MockTask,
    MockTimer,
    MockTouch,
    MockVision,
);
