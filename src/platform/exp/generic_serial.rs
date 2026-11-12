// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! Smart Port Generic Serial Communication

use crate::platform::jump_table;

jump_table! {
    #[cfg(any())]
    0x000 => pub fn vexDeviceGenericSerialEnable(device: V5_DeviceT, options: i32);
    #[cfg(any())]
    0x000 => pub fn vexDeviceGenericSerialBaudrate(device: V5_DeviceT, baudrate: i32);
    #[cfg(any())]
    0x000 => pub fn vexDeviceGenericSerialWriteChar(device: V5_DeviceT, c: u8) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceGenericSerialWriteFree(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceGenericSerialTransmit(
        device: V5_DeviceT,
        buffer: *const u8,
        length: i32,
    ) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceGenericSerialReadChar(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceGenericSerialPeekChar(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceGenericSerialReceiveAvail(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceGenericSerialReceive(
        device: V5_DeviceT,
        buffer: *mut u8,
        length: i32,
    ) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceGenericSerialFlush(device: V5_DeviceT);
}
