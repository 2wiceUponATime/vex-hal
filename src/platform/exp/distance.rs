// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! V5 Distance Sensor

use core::ffi::c_double;

use crate::platform::{V5_DeviceT, jump_table};

jump_table! {
    0x500 => pub fn vexDeviceDistanceDistanceGet(device: V5_DeviceT) -> u32;
    0x504 => pub fn vexDeviceDistanceConfidenceGet(device: V5_DeviceT) -> u32;
    0x508 => pub fn vexDeviceDistanceStatusGet(device: V5_DeviceT) -> u32;
    0x518 => pub fn vexDeviceDistanceObjectSizeGet(device: V5_DeviceT) -> i32;
    0x051c => pub fn vexDeviceDistanceObjectVelocityGet(device: V5_DeviceT) -> c_double;
}
