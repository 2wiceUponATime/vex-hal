// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! Brain Screen Touchscreen

use core::ffi::c_uchar;

use crate::platform::c_enum;

c_enum! {
    pub struct V5_TouchEvent: c_uchar {
        kTouchEventRelease = 0,
        kTouchEventPress = 1,
        kTouchEventPressAuto = 2,
    }
}

#[repr(C)]
#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
#[allow(non_snake_case)]
pub struct V5_TouchStatus {
    pub lastEvent: V5_TouchEvent,
    pub lastXpos: i16,
    pub lastYpos: i16,
    pub pressCount: i32,
    pub releaseCount: i32,
}

mock! {
    pub trait MockTouch;
    pub fn vexTouchUserCallbackSet(
        callback: unsafe extern "C" fn(V5_TouchEvent, i32, i32),
    );
    pub fn vexTouchDataGet(status: *mut V5_TouchStatus);
}
