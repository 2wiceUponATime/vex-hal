// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! CTE Workcell Signal Tower

use crate::platform::V5_DeviceT;

mock! {
    LightTower:
    pub fn vexDeviceLightTowerBlinkSet(
        device: V5_DeviceT,
        select: u8,
        mask: u8,
        onTime: i32,
        offTime: i32,
    );
    pub fn vexDeviceLightTowerColorSet(device: V5_DeviceT, color_id: u32, value: u32);
    pub fn vexDeviceLightTowerRgbGet(device: V5_DeviceT) -> u32;
    pub fn vexDeviceLightTowerRgbSet(device: V5_DeviceT, rgb_value: u32, xyw_value: u32);
    pub fn vexDeviceLightTowerStatusGet(device: V5_DeviceT) -> u32;
    pub fn vexDeviceLightTowerDebugGet(device: V5_DeviceT, id: i32) -> u32;
    pub fn vexDeviceLightTowerXywGet(device: V5_DeviceT) -> u32;
}
