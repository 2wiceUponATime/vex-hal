// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! V5 AI Vision Sensor

use super::jump_table;

jump_table! {
    #[cfg(any())]
    0x000 => pub fn vexDeviceAiVisionClassNameGet(
        device: V5_DeviceT,
        id: i32,
        pName: *mut u8,
    ) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceAiVisionCodeGet(
        device: V5_DeviceT,
        id: u32,
        pCode: *mut V5_DeviceAiVisionCode,
    ) -> bool;
    #[cfg(any())]
    0x000 => pub fn vexDeviceAiVisionCodeSet(
        device: V5_DeviceT,
        pCode: *mut V5_DeviceAiVisionCode,
    );
    #[cfg(any())]
    0x000 => pub fn vexDeviceAiVisionColorGet(
        device: V5_DeviceT,
        id: u32,
        pColor: *mut V5_DeviceAiVisionColor,
    ) -> bool;
    #[cfg(any())]
    0x000 => pub fn vexDeviceAiVisionColorSet(
        device: V5_DeviceT,
        pColor: *mut V5_DeviceAiVisionColor,
    );
    #[cfg(any())]
    0x000 => pub fn vexDeviceAiVisionModeGet(device: V5_DeviceT) -> u32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceAiVisionModeSet(device: V5_DeviceT, mode: u32);
    #[cfg(any())]
    0x000 => pub fn vexDeviceAiVisionObjectCountGet(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceAiVisionObjectGet(
        device: V5_DeviceT,
        indexObj: u32,
        pObject: *mut V5_DeviceAiVisionObject,
    ) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceAiVisionSensorSet(
        device: V5_DeviceT,
        brightness: c_double,
        contrast: c_double,
    );
    #[cfg(any())]
    0x000 => pub fn vexDeviceAiVisionStatusGet(device: V5_DeviceT) -> u32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceAiVisionTemperatureGet(device: V5_DeviceT) -> c_double;
}
