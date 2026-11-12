// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! Rangefinder/Lidar Sensor
//!
//! This sensor is not sold by VEX.

use crate::platform::jump_table;

jump_table! {
    /// Rangefinder/Lidar - actual API to be determined
    #[cfg(any())]
    0x000 => pub fn vexDeviceRangeValueGet(device: V5_DeviceT) -> i32;
}
