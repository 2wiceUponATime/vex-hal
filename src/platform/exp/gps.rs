// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! V5 GPS

use crate::platform::jump_table;

jump_table! {
    #[cfg(any())]
    0x000 => pub fn vexDeviceGpsReset(device: V5_DeviceT);
    #[cfg(any())]
    0x000 => pub fn vexDeviceGpsHeadingGet(device: V5_DeviceT) -> c_double;
    #[cfg(any())]
    0x000 => pub fn vexDeviceGpsDegreesGet(device: V5_DeviceT) -> c_double;
    #[cfg(any())]
    0x000 => pub fn vexDeviceGpsQuaternionGet(
        device: V5_DeviceT,
        data: *mut V5_DeviceGpsQuaternion,
    );
    #[cfg(any())]
    0x000 => pub fn vexDeviceGpsAttitudeGet(
        device: V5_DeviceT,
        data: *mut V5_DeviceGpsAttitude,
        bRaw: bool,
    );
    #[cfg(any())]
    0x000 => pub fn vexDeviceGpsRawGyroGet(device: V5_DeviceT, data: *mut V5_DeviceGpsRaw);
    #[cfg(any())]
    0x000 => pub fn vexDeviceGpsRawAccelGet(device: V5_DeviceT, data: *mut V5_DeviceGpsRaw);
    #[cfg(any())]
    0x000 => pub fn vexDeviceGpsStatusGet(device: V5_DeviceT) -> u32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceGpsModeSet(device: V5_DeviceT, mode: u32);
    #[cfg(any())]
    0x000 => pub fn vexDeviceGpsModeGet(device: V5_DeviceT) -> u32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceGpsDataRateSet(device: V5_DeviceT, rate: u32);
    #[cfg(any())]
    0x000 => pub fn vexDeviceGpsOriginSet(device: V5_DeviceT, ox: c_double, oy: c_double);
    #[cfg(any())]
    0x000 => pub fn vexDeviceGpsOriginGet(
        device: V5_DeviceT,
        ox: *mut c_double,
        oy: *mut c_double,
    );
    #[cfg(any())]
    0x000 => pub fn vexDeviceGpsRotationSet(device: V5_DeviceT, value: c_double);
    #[cfg(any())]
    0x000 => pub fn vexDeviceGpsRotationGet(device: V5_DeviceT) -> c_double;
    #[cfg(any())]
    0x000 => pub fn vexDeviceGpsInitialPositionSet(
        device: V5_DeviceT,
        initial_x: c_double,
        initial_y: c_double,
        initial_rotation: c_double,
    );
    #[cfg(any())]
    0x000 => pub fn vexDeviceGpsErrorGet(device: V5_DeviceT) -> c_double;
}
