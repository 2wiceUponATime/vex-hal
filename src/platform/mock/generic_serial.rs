// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! Smart Port Generic Serial Communication

use crate::platform::V5_DeviceT;

mock! {
    GenericSerial:
    pub fn vexDeviceGenericSerialEnable(device: V5_DeviceT, options: i32);
    pub fn vexDeviceGenericSerialBaudrate(device: V5_DeviceT, baudrate: i32);
    pub fn vexDeviceGenericSerialWriteChar(device: V5_DeviceT, c: u8) -> i32;
    pub fn vexDeviceGenericSerialWriteFree(device: V5_DeviceT) -> i32;
    pub fn vexDeviceGenericSerialTransmit(
        device: V5_DeviceT,
        buffer: *const u8,
        length: i32,
    ) -> i32;
    pub fn vexDeviceGenericSerialReadChar(device: V5_DeviceT) -> i32;
    pub fn vexDeviceGenericSerialPeekChar(device: V5_DeviceT) -> i32;
    pub fn vexDeviceGenericSerialReceiveAvail(device: V5_DeviceT) -> i32;
    pub fn vexDeviceGenericSerialReceive(
        device: V5_DeviceT,
        buffer: *mut u8,
        length: i32,
    ) -> i32;
    pub fn vexDeviceGenericSerialFlush(device: V5_DeviceT);
}
