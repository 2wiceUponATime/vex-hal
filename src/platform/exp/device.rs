// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! V5 Smart Devices

use super::jump_table;

jump_table! {
    /// Get the number of device ports currently present on this system.
    ///
    /// As of VEXos 1.1.4, this number is the constant `23`.
    #[cfg(any())]
    0x000 => pub fn vexDevicesGetNumber() -> u32;
    /// Get the number of devices of a specific type plugged into the brain.
    #[cfg(any())]
    0x000 => pub fn vexDevicesGetNumberByType(device_type: V5_DeviceType) -> u32;
    /// Get a buffer of all devices on the brain.
    #[cfg(any())]
    0x000 => pub fn vexDevicesGet() -> V5_DeviceT;
    /// Get a handle to a device plugged into the specified port index.
    #[cfg(any())]
    0x000 => pub fn vexDeviceGetByIndex(index: u32) -> V5_DeviceT;
    /// Get a list of device types plugged into the brain.
    ///
    /// Returns -1 if a null pointer is passed, otherwise the number of devices
    /// that were written to the buffer.
    ///
    /// The length of the buffer should be at most [`V5_MAX_DEVICE_PORTS`].
    #[cfg(any())]
    0x000 => pub fn vexDeviceGetStatus(devices: *mut V5_DeviceType) -> i32;
    /// Get the timestamp recorded by a device's internal clock.
    #[cfg(any())]
    0x000 => pub fn vexDeviceGetTimestamp(device: V5_DeviceT) -> u32;
    /// Get the reading of a generic sensor.
    ///
    /// Returns `0` if the device is not considered a generic sensor by VEXos. Note that this
    /// function is not a way to read from all kinds of sensor with a single call: for example,
    /// distance sensors are not Generic sensors any more than they are motors, so they will
    /// return zero.
    #[cfg(any())]
    0x000 => pub fn vexDeviceGenericValueGet(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceButtonStateGet() -> c_int;
}
