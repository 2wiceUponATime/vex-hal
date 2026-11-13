// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! EXP Controller

use core::ffi::c_uchar;

use crate::platform::{c_enum, jump_table};

c_enum! {
    pub struct V5_ControllerId: c_uchar {
        kControllerMaster = 0,
        kControllerPartner = 1,
    }
}

c_enum! {
    pub struct V5_ControllerStatus: c_uchar {
        kV5ControllerOffline = 0,
        kV5ControllerTethered = 1,
        kV5ControllerVexnet = 2,
    }
}

c_enum! {
    pub struct V5_ControllerIndex: c_uchar {
        AnaLeftX = 0,
        AnaLeftY = 1,
        AnaRightX = 2,
        AnaRightY = 3,
        AnaSpare1 = 4,
        AnaSpare2 = 5,
        Button5U = 6,
        Button5D = 7,
        Button6U = 8,
        Button6D = 9,
        Button7U = 10,
        Button7D = 11,
        Button7L = 12,
        Button7R = 13,
        Button8U = 14,
        Button8D = 15,
        Button8L = 16,
        Button8R = 17,
        ButtonSEL = 18,
        BatteryLevel = 19,
        ButtonAll = 20,
        Flags = 21,
        BatteryCapacity = 22,
    }
}

#[allow(non_upper_case_globals)]
impl V5_ControllerIndex {
    pub const Axis1: Self = Self::AnaRightX;
    pub const Axis2: Self = Self::AnaRightY;
    pub const Axis3: Self = Self::AnaLeftY;
    pub const Axis4: Self = Self::AnaLeftX;
    pub const ButtonL1: Self = Self::Button5U;
    pub const ButtonL2: Self = Self::Button5D;
    pub const ButtonR1: Self = Self::Button6U;
    pub const ButtonR2: Self = Self::Button6D;
    pub const ButtonUp: Self = Self::Button7U;
    pub const ButtonDown: Self = Self::Button7D;
    pub const ButtonLeft: Self = Self::Button7L;
    pub const ButtonRight: Self = Self::Button7R;
    pub const ButtonX: Self = Self::Button8U;
    pub const ButtonB: Self = Self::Button8D;
    pub const ButtonY: Self = Self::Button8L;
    pub const ButtonA: Self = Self::Button8R;
}

jump_table! {
    /// Get the value of a controller's data channel.
    0x1a4 => pub fn vexControllerGet(id: V5_ControllerId, index: V5_ControllerIndex) -> i32;
    /// Returns `1` if the controller on the given ID is connected, or `0` if not.
    0x1a8 => pub fn vexControllerConnectionStatusGet(id: V5_ControllerId) -> V5_ControllerStatus;
    0x1ac => pub fn vexControllerTextSet(id: u32, line: u32, col: u32, buf: *const u8) -> u32;
}
