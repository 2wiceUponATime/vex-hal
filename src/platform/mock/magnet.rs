// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! V5 Workcell Electromagnet

use core::ffi::{c_double, c_uchar};

use crate::platform::{V5_DeviceT, c_enum};

c_enum! {
    pub struct V5_DeviceMagnetDuration: c_uchar {
        kMagnetDurationShort = 0,
        kMagnetDurationMedium = 1,
        kMagnetDurationLong = 2,
        kMagnetDurationExtraLong = 3,
    }
}

mock! {
    pub fn vexDeviceMagnetPowerSet(device: V5_DeviceT, value: i32, time: i32);
    pub fn vexDeviceMagnetPowerGet(device: V5_DeviceT) -> i32;
    pub fn vexDeviceMagnetPickup(device: V5_DeviceT, duration: V5_DeviceMagnetDuration);
    pub fn vexDeviceMagnetDrop(device: V5_DeviceT, duration: V5_DeviceMagnetDuration);
    pub fn vexDeviceMagnetTemperatureGet(device: V5_DeviceT) -> c_double;
    pub fn vexDeviceMagnetCurrentGet(device: V5_DeviceT) -> c_double;
    pub fn vexDeviceMagnetStatusGet(device: V5_DeviceT) -> u32;
}
