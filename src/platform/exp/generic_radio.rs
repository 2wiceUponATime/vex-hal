// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! V5 Smart Radio

use core::ffi::{c_char, c_int};

use crate::platform::{V5_DeviceT, jump_table};

jump_table! {
    0xaa4 => pub fn vexDeviceGenericRadioConnection(
        device: V5_DeviceT,
        link_id: *mut c_char,
        r#type: c_int,
        ov: bool,
    );
    // Potential signature: 0xaa8 => pub fn vexDeviceGenericRadioWriteChar(device: V5_DeviceT, c: u8);
    0xaac => pub fn vexDeviceGenericRadioWriteFree(device: V5_DeviceT) -> i32;
    0xab0 => pub fn vexDeviceGenericRadioTransmit(
        device: V5_DeviceT,
        data: *const u8,
        size: u16,
    ) -> i32;
    0xab4 => pub fn vexDeviceGenericRadioReadChar(device: V5_DeviceT) -> i32;
    0xab8 => pub fn vexDeviceGenericRadioPeekChar(device: V5_DeviceT) -> i32;
    0xabc => pub fn vexDeviceGenericRadioReceiveAvail(device: V5_DeviceT) -> i32;
    0xac0 => pub fn vexDeviceGenericRadioReceive(
        device: V5_DeviceT,
        data: *mut u8,
        size: u16,
    ) -> i32;
    0x8c4 => pub fn vexDeviceGenericRadioFlush(device: V5_DeviceT);
    0xac8 => pub fn vexDeviceGenericRadioLinkStatus(device: V5_DeviceT) -> bool;
}
