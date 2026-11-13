// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! V5 LED
//!
//! This device is not sold by VEX and only exists as development hardware.

use core::ffi::c_uint;

use crate::platform::{V5_DeviceT, c_enum, jump_table};

c_enum! {
    #[allow(non_camel_case_types)]
    pub struct V5_DeviceLedColor: c_uint {
        kLedColorBlack = 0,
        kLedColorRed = 0xFF0000,
        kLedColorGreen = 0x00FF00,
        kLedColorBlue = 0x0000FF,
        kLedColorYellow = 0xFFFF00,
        kLedColorCyan = 0x00FFFF,
        kLedColorMagenta = 0xFF00FF,
        kLedColorWhite = 0xFFFFFF,
    }
}

jump_table! {
    0x1e0 => pub fn vexDeviceLedSet(device: V5_DeviceT, value: V5_DeviceLedColor);
    0x1e4 => pub fn vexDeviceLedRgbSet(device: V5_DeviceT, color: u32);
    0x1e8 => pub fn vexDeviceLedGet(device: V5_DeviceT) -> V5_DeviceLedColor;
    0x1ec => pub fn vexDeviceLedRgbGet(device: V5_DeviceT) -> u32;
}
