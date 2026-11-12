// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! ADI Devices

use super::jump_table;

jump_table! {
    #[cfg(any())]
    0x000 => pub fn vexDeviceAdiPortConfigSet(
        device: V5_DeviceT,
        port: u32,
        config: V5_AdiPortConfiguration,
    );
    #[cfg(any())]
    0x000 => pub fn vexDeviceAdiPortConfigGet(
        device: V5_DeviceT,
        port: u32,
    ) -> V5_AdiPortConfiguration;
    #[cfg(any())]
    0x000 => pub fn vexDeviceAdiValueSet(device: V5_DeviceT, port: u32, value: i32);
    #[cfg(any())]
    0x000 => pub fn vexDeviceAdiValueGet(device: V5_DeviceT, port: u32) -> i32;
    /// <Derived from <https://github.com/purduesigbots/pros/blob/89a7417352fbbc86420325afe410861e2210743c/src/devices/vdml_ext_adi.c#L40>
    #[cfg(any())]
    0x000 => pub fn vexDeviceAdiAddrLedSet(
        device: V5_DeviceT,
        port: u32,
        pData: *mut u32,
        nOffset: u32,
        nLength: u32,
        options: u32,
    );
    #[cfg(any())]
    0x000 => pub fn vexDeviceBumperGet(device: V5_DeviceT) -> V5_DeviceBumperState;
    #[cfg(any())]
    0x000 => pub fn vexDeviceGyroReset(device: V5_DeviceT);
    #[cfg(any())]
    0x000 => pub fn vexDeviceGyroHeadingGet(device: V5_DeviceT) -> c_double;
    #[cfg(any())]
    0x000 => pub fn vexDeviceGyroDegreesGet(device: V5_DeviceT) -> c_double;
    #[cfg(any())]
    0x000 => pub fn vexDeviceSonarValueGet(device: V5_DeviceT) -> i32;
}
