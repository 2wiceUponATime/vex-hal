// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! V5 Inertial Sensor

use core::ffi::{c_double, c_uchar};

use crate::platform::{V5_DeviceT, c_enum};

#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct V5_DeviceImuRaw {
    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
    pub w: c_double,
}

#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct V5_DeviceImuQuaternion {
    pub a: c_double,
    pub b: c_double,
    pub c: c_double,
    pub d: c_double,
}

c_enum! {
    pub struct V5ImuOrientationMode: c_uchar {
        kImuOrientationZUp = 0x00,
        kImuOrientationZDown = 0x10,
        kImuOrientationXUp = 0x20,
        kImuOrientationXDown = 0x30,
        kImuOrientationYUp = 0x40,
        kImuOrientationYDown = 0x50,
        kImuOrientationAuto = 0x80,
    }
}

#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub struct V5_DeviceImuAttitude {
    pub pitch: c_double,
    pub roll: c_double,
    pub yaw: c_double,
}

mock! {
    pub trait MockImu;
    /// Calibrates the IMU. This function is non-blocking.
    pub fn vexDeviceImuReset(device: V5_DeviceT);
    /// Returns the yaw-axis rotation of the IMU as an unbounded angle in degrees.
    pub fn vexDeviceImuHeadingGet(device: V5_DeviceT) -> c_double;
    /// Returns the yaw-axis rotation of the IMU as an angle in degrees bounded 0-360.
    pub fn vexDeviceImuDegreesGet(device: V5_DeviceT) -> c_double;
    /// Returns quaternion defined by the IMU's rotation.
    pub fn vexDeviceImuQuaternionGet(
        device: V5_DeviceT,
        data: *mut V5_DeviceImuQuaternion,
    );
    /// Returns the 3-axis euler angles of the IMU bounded from -180 to 180 degrees.
    pub fn vexDeviceImuAttitudeGet(device: V5_DeviceT, data: *mut V5_DeviceImuAttitude);
    /// Returns the raw gyroscope readings of the IMU in degrees per second,
    pub fn vexDeviceImuRawGyroGet(device: V5_DeviceT, data: *mut V5_DeviceImuRaw);
    /// Returns the raw accelerometer readings of the IMU in G.
    pub fn vexDeviceImuRawAccelGet(device: V5_DeviceT, data: *mut V5_DeviceImuRaw);
    /// Returns the IMU's status bits.
    pub fn vexDeviceImuStatusGet(device: V5_DeviceT) -> u32;
    pub fn vexDeviceImuModeSet(device: V5_DeviceT, mode: u32);
    pub fn vexDeviceImuModeGet(device: V5_DeviceT) -> u32;
    pub fn vexDeviceImuDataRateSet(device: V5_DeviceT, rate: u32);
}
