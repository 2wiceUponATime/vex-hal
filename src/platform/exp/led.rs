// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! V5 LED
//!
//! This device is not sold by VEX and only exists as development hardware.

use crate::platform::jump_table;

jump_table! {
    #[cfg(any())]
    0x000 => pub fn vexDeviceLedSet(device: V5_DeviceT, value: V5_DeviceLedColor);
    #[cfg(any())]
    0x000 => pub fn vexDeviceLedRgbSet(device: V5_DeviceT, color: u32);
    #[cfg(any())]
    0x000 => pub fn vexDeviceLedGet(device: V5_DeviceT) -> V5_DeviceLedColor;
    #[cfg(any())]
    0x000 => pub fn vexDeviceLedRgbGet(device: V5_DeviceT) -> u32;
}
