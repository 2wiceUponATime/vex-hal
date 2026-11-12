// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! V5 Vision Sensor

use super::jump_table;

jump_table! {
    #[cfg(any())]
    0x000 => pub fn vexDeviceVisionModeSet(device: V5_DeviceT, mode: V5VisionMode);
    #[cfg(any())]
    0x000 => pub fn vexDeviceVisionModeGet(device: V5_DeviceT) -> V5VisionMode;
    #[cfg(any())]
    0x000 => pub fn vexDeviceVisionObjectCountGet(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceVisionObjectGet(
        device: V5_DeviceT,
        index: u32,
        object: *mut V5_DeviceVisionObject,
    ) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceVisionSignatureSet(
        device: V5_DeviceT,
        signature: *mut V5_DeviceVisionSignature,
    );
    #[cfg(any())]
    0x000 => pub fn vexDeviceVisionSignatureGet(
        device: V5_DeviceT,
        id: u32,
        signature: *mut V5_DeviceVisionSignature,
    ) -> bool;
    #[cfg(any())]
    0x000 => pub fn vexDeviceVisionBrightnessSet(device: V5_DeviceT, percent: u8);
    #[cfg(any())]
    0x000 => pub fn vexDeviceVisionBrightnessGet(device: V5_DeviceT) -> u8;
    #[cfg(any())]
    0x000 => pub fn vexDeviceVisionWhiteBalanceModeSet(device: V5_DeviceT, mode: V5VisionWBMode);
    #[cfg(any())]
    0x000 => pub fn vexDeviceVisionWhiteBalanceModeGet(device: V5_DeviceT) -> V5VisionWBMode;
    #[cfg(any())]
    0x000 => pub fn vexDeviceVisionWhiteBalanceSet(device: V5_DeviceT, color: V5_DeviceVisionRgb);
    #[cfg(any())]
    0x000 => pub fn vexDeviceVisionWhiteBalanceGet(device: V5_DeviceT) -> V5_DeviceVisionRgb;
    #[cfg(any())]
    0x000 => pub fn vexDeviceVisionLedModeSet(device: V5_DeviceT, mode: V5VisionLedMode);
    #[cfg(any())]
    0x000 => pub fn vexDeviceVisionLedModeGet(device: V5_DeviceT) -> V5VisionLedMode;
    #[cfg(any())]
    0x000 => pub fn vexDeviceVisionLedBrigntnessSet(device: V5_DeviceT, percent: u8);
    #[cfg(any())]
    0x000 => pub fn vexDeviceVisionLedBrigntnessGet(device: V5_DeviceT) -> u8;
    #[cfg(any())]
    0x000 => pub fn vexDeviceVisionLedColorSet(device: V5_DeviceT, color: V5_DeviceVisionRgb);
    #[cfg(any())]
    0x000 => pub fn vexDeviceVisionLedColorGet(device: V5_DeviceT) -> V5_DeviceVisionRgb;
    #[cfg(any())]
    0x000 => pub fn vexDeviceVisionWifiModeSet(device: V5_DeviceT, mode: V5VisionWifiMode);
    #[cfg(any())]
    0x000 => pub fn vexDeviceVisionWifiModeGet(device: V5_DeviceT) -> V5VisionWifiMode;
}
