// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! V5 Rotation Sensor

use super::jump_table;

jump_table! {
    #[cfg(any())]
    0x000 => pub fn vexDeviceAbsEncReset(device: V5_DeviceT);
    #[cfg(any())]
    0x000 => pub fn vexDeviceAbsEncPositionSet(device: V5_DeviceT, position: i32);
    #[cfg(any())]
    0x000 => pub fn vexDeviceAbsEncPositionGet(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceAbsEncVelocityGet(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceAbsEncAngleGet(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceAbsEncReverseFlagSet(device: V5_DeviceT, value: bool);
    #[cfg(any())]
    0x000 => pub fn vexDeviceAbsEncReverseFlagGet(device: V5_DeviceT) -> bool;
    #[cfg(any())]
    0x000 => pub fn vexDeviceAbsEncStatusGet(device: V5_DeviceT) -> u32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceAbsEncDataRateSet(device: V5_DeviceT, rate: u32);
}
