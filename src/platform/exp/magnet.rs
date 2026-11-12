// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! V5 Workcell Electromagnet

use super::jump_table;

jump_table! {
    #[cfg(any())]
    0x000 => pub fn vexDeviceMagnetPowerSet(device: V5_DeviceT, value: i32, time: i32);
    #[cfg(any())]
    0x000 => pub fn vexDeviceMagnetPowerGet(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMagnetPickup(device: V5_DeviceT, duration: V5_DeviceMagnetDuration);
    #[cfg(any())]
    0x000 => pub fn vexDeviceMagnetDrop(device: V5_DeviceT, duration: V5_DeviceMagnetDuration);
    #[cfg(any())]
    0x000 => pub fn vexDeviceMagnetTemperatureGet(device: V5_DeviceT) -> c_double;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMagnetCurrentGet(device: V5_DeviceT) -> c_double;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMagnetStatusGet(device: V5_DeviceT) -> u32;
}
