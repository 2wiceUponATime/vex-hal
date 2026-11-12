// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! V5 Smart Radio

use crate::platform::jump_table;

jump_table! {
    #[cfg(any())]
    0x000 => pub fn vexDeviceGenericRadioConnection(
        device: V5_DeviceT,
        link_id: *mut c_char,
        r#type: c_int,
        ov: bool,
    );
    #[cfg(any())]
    0x000 => pub fn vexDeviceGenericRadioWriteFree(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceGenericRadioTransmit(
        device: V5_DeviceT,
        data: *const u8,
        size: u16,
    ) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceGenericRadioReceiveAvail(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceGenericRadioReceive(
        device: V5_DeviceT,
        data: *mut u8,
        size: u16,
    ) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceGenericRadioLinkStatus(device: V5_DeviceT) -> bool;
}
