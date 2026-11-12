// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! V5 Inertial Sensor

use crate::platform::jump_table;

jump_table! {
    /// Calibrates the IMU. This function is non-blocking.
    #[cfg(any())]
    0x000 => pub fn vexDeviceImuReset(device: V5_DeviceT);
    /// Returns the yaw-axis rotation of the IMU as an unbounded angle in degrees.
    #[cfg(any())]
    0x000 => pub fn vexDeviceImuHeadingGet(device: V5_DeviceT) -> c_double;
    /// Returns the yaw-axis rotation of the IMU as an angle in degrees bounded 0-360.
    #[cfg(any())]
    0x000 => pub fn vexDeviceImuDegreesGet(device: V5_DeviceT) -> c_double;
    /// Returns quaternion defined by the IMU's rotation.
    #[cfg(any())]
    0x000 => pub fn vexDeviceImuQuaternionGet(
        device: V5_DeviceT,
        data: *mut V5_DeviceImuQuaternion,
    );
    /// Returns the 3-axis euler angles of the IMU bounded from -180 to 180 degrees.
    #[cfg(any())]
    0x000 => pub fn vexDeviceImuAttitudeGet(device: V5_DeviceT, data: *mut V5_DeviceImuAttitude);
    /// Returns the raw gyroscope readings of the IMU in degrees per second,
    #[cfg(any())]
    0x000 => pub fn vexDeviceImuRawGyroGet(device: V5_DeviceT, data: *mut V5_DeviceImuRaw);
    /// Returns the raw accelerometer readings of the IMU in G.
    #[cfg(any())]
    0x000 => pub fn vexDeviceImuRawAccelGet(device: V5_DeviceT, data: *mut V5_DeviceImuRaw);
    /// Returns the IMU's status bits.
    #[cfg(any())]
    0x000 => pub fn vexDeviceImuStatusGet(device: V5_DeviceT) -> u32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceImuModeSet(device: V5_DeviceT, mode: u32);
    #[cfg(any())]
    0x000 => pub fn vexDeviceImuModeGet(device: V5_DeviceT) -> u32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceImuDataRateSet(device: V5_DeviceT, rate: u32);
}
