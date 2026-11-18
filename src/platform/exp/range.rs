// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! Rangefinder/Lidar Sensor
//!
//! This sensor is not sold by VEX.

use crate::platform::{V5_DeviceT, jump_table};

jump_table! {
    /// Unimplemented as of VEXos 1.0.8.
    ///
    /// Rangefinder/Lidar - actual API to be determined
    0x4d8 => pub fn vexDeviceRangeValueGet(device: V5_DeviceT) -> i32;
}
