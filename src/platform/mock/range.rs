// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! Rangefinder/Lidar Sensor
//!
//! This sensor is not sold by VEX.

use crate::platform::V5_DeviceT;

mock! {
    Range:
    /// Rangefinder/Lidar - actual API to be determined
    pub fn vexDeviceRangeValueGet(device: V5_DeviceT) -> i32;
}
