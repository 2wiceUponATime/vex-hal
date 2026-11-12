// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! V5 Optical Sensor

use super::jump_table;

jump_table! {
    #[cfg(any())]
    0x000 => pub fn vexDeviceOpticalHueGet(device: V5_DeviceT) -> c_double;
    #[cfg(any())]
    0x000 => pub fn vexDeviceOpticalSatGet(device: V5_DeviceT) -> c_double;
    #[cfg(any())]
    0x000 => pub fn vexDeviceOpticalBrightnessGet(device: V5_DeviceT) -> c_double;
    #[cfg(any())]
    0x000 => pub fn vexDeviceOpticalProximityGet(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceOpticalRgbGet(device: V5_DeviceT, data: *mut V5_DeviceOpticalRgb);
    #[cfg(any())]
    0x000 => pub fn vexDeviceOpticalLedPwmSet(device: V5_DeviceT, value: i32);
    #[cfg(any())]
    0x000 => pub fn vexDeviceOpticalLedPwmGet(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceOpticalStatusGet(device: V5_DeviceT) -> u32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceOpticalRawGet(device: V5_DeviceT, data: *mut V5_DeviceOpticalRaw);
    #[cfg(any())]
    0x000 => pub fn vexDeviceOpticalModeSet(device: V5_DeviceT, mode: u32);
    #[cfg(any())]
    0x000 => pub fn vexDeviceOpticalModeGet(device: V5_DeviceT) -> u32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceOpticalGestureGet(
        device: V5_DeviceT,
        pData: *mut V5_DeviceOpticalGesture,
    ) -> u32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceOpticalGestureEnable(device: V5_DeviceT);
    #[cfg(any())]
    0x000 => pub fn vexDeviceOpticalGestureDisable(device: V5_DeviceT);
    #[cfg(any())]
    0x000 => pub fn vexDeviceOpticalProximityThreshold(device: V5_DeviceT, value: i32);
    #[cfg(any())]
    0x000 => pub fn vexDeviceOpticalIntegrationTimeSet(device: V5_DeviceT, timeMs: c_double);
    #[cfg(any())]
    0x000 => pub fn vexDeviceOpticalIntegrationTimeGet(device: V5_DeviceT) -> c_double;
}
