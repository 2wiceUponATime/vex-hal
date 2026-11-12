// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! V5 Distance Sensor

use super::jump_table;

jump_table! {
    #[cfg(any())]
    0x000 => pub fn vexDeviceDistanceDistanceGet(device: V5_DeviceT) -> u32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceDistanceConfidenceGet(device: V5_DeviceT) -> u32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceDistanceStatusGet(device: V5_DeviceT) -> u32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceDistanceObjectSizeGet(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceDistanceObjectVelocityGet(device: V5_DeviceT) -> c_double;
}
