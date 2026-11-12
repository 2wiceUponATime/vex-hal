// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! CTE Workcell Pneumatics Control

use super::jump_table;

jump_table! {
    #[cfg(any())]
    0x000 => pub fn vexDevicePneumaticActuationStatusGet(
        device: V5_DeviceT,
        ac1: *mut u16,
        ac2: *mut u16,
        ac3: *mut u16,
        ac4: *mut u16,
    ) -> u32;
    #[cfg(any())]
    0x000 => pub fn vexDevicePneumaticCompressorSet(device: V5_DeviceT, bState: bool);
    #[cfg(any())]
    0x000 => pub fn vexDevicePneumaticCtrlSet(
        device: V5_DeviceT,
        pCtrl: *mut V5_DevicePneumaticCtrl,
    );
    #[cfg(any())]
    0x000 => pub fn vexDevicePneumaticCylinderPwmSet(
        device: V5_DeviceT,
        id: u32,
        bState: bool,
        pwm: u8,
    );
    #[cfg(any())]
    0x000 => pub fn vexDevicePneumaticCylinderSet(device: V5_DeviceT, id: u32, bState: bool);
    #[cfg(any())]
    0x000 => pub fn vexDevicePneumaticPwmGet(device: V5_DeviceT) -> u32;
    #[cfg(any())]
    0x000 => pub fn vexDevicePneumaticPwmSet(device: V5_DeviceT, pwm: u8);
    #[cfg(any())]
    0x000 => pub fn vexDevicePneumaticStatusGet(device: V5_DeviceT) -> u32;
}
