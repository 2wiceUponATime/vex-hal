// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! EXP Smart Devices

use core::ffi::c_uchar;

use crate::platform::{V5_DeviceT, c_enum};

c_enum! {
    pub struct V5_DeviceType: c_uchar {
        /// No device connected
        kDeviceTypeNoSensor = 0,
        /// V5 Smart Motor
        kDeviceTypeMotorSensor = 2,
        /// Unknown use (possibly unreleased hardware)
        kDeviceTypeLedSensor = 3,
        /// Rotation Sensor
        kDeviceTypeAbsEncSensor = 4,
        /// V5 Motor CR (unknown use)
        kDeviceTypeCrMotorSensor = 5,
        /// Inertial Sensor
        kDeviceTypeImuSensor = 6,
        /// Distance Sensor
        kDeviceTypeDistanceSensor = 7,
        /// Radio
        kDeviceTypeRadioSensor = 8,
        /// Master Controller
        kDeviceTypeTetherSensor = 9,
        /// Brain
        kDeviceTypeBrainSensor = 10,
        /// Vision Sensor
        kDeviceTypeVisionSensor = 11,
        /// ADI
        kDeviceTypeAdiSensor = 12,
        /// Partner Controller
        kDeviceTypeRes1Sensor = 13,
        /// Battery
        kDeviceTypeRes2Sensor = 14,
        /// Solenoid (unknown use)
        kDeviceTypeRes3Sensor = 15,
        /// Optical Sensor
        kDeviceTypeOpticalSensor = 16,
        /// Electromagnet
        kDeviceTypeMagnetSensor = 17,
        /// GPS
        kDeviceTypeGpsSensor = 20,
        /// AI Stereo Camera
        kDeviceTypeAicameraSensor = 26,
        /// CTE Workcell Light Tower
        kDeviceTypeLightTowerSensor = 27,
        /// CTE Workcell Arm
        kDeviceTypeArmDevice = 28,
        /// AI Vision Sensor
        kDeviceTypeAiVisionSensor = 29,
        /// CTE Workcell Pneumatics
        kDeviceTypePneumaticSensor = 30,
        // All of these are probably just unreleased or beta hardware...
        kDeviceTypeBumperSensor = 0x40,
        kDeviceTypeGyroSensor = 0x46,
        kDeviceTypeSonarSensor = 0x47,
        kDeviceTypeGenericSensor = 128,
        /// Generic Serial
        kDeviceTypeGenericSerial = 129,
        /// Unknown use
        kDeviceTypeUndefinedSensor = 255,
    }
}

mock! {
    /// Get the number of device ports currently present on this system.
    ///
    /// As of VEXos 1.0.8, this number is the constant `12` - 2 more than the number of physical
    /// ports on the brain.
    pub fn vexDevicesGetNumber() -> u32;
    /// Get the number of devices of a specific type plugged into the brain.
    pub fn vexDevicesGetNumberByType(device_type: V5_DeviceType) -> u32;
    /// Get a buffer of all devices on the brain.
    pub fn vexDevicesGet() -> V5_DeviceT;
    /// Get a handle to a device plugged into the specified port index.
    pub fn vexDeviceGetByIndex(index: u32) -> V5_DeviceT;
    /// Get a list of device types plugged into the brain.
    ///
    /// Returns -1 if a null pointer is passed, otherwise the number of devices
    /// that were written to the buffer.
    ///
    /// The length of the buffer should be at most [`V5_MAX_DEVICE_PORTS`](crate::platform::V5_MAX_DEVICE_PORTS).
    pub fn vexDeviceGetStatus(devices: *mut V5_DeviceType) -> i32;
    /// Get the timestamp recorded by a device's internal clock.
    pub fn vexDeviceGetTimestamp(device: V5_DeviceT) -> i32;
    pub fn vexDeviceButtonStateGet() -> u32;
    pub fn vexDeviceTypeGetByIndex(index: u32) -> V5_DeviceType;
    pub fn vexDeviceTypeSetByIndex(index: u32, r#type: V5_DeviceType);
    pub fn vexDeviceFlagsGetByIndex(index: u32) -> u32;
    /// Unimplemented as of VEXos 1.0.8.
    ///
    /// Get the reading of a generic sensor.
    ///
    /// Returns `0` if the device is not considered a generic sensor by VEXos. Note that this
    /// function is not a way to read from all kinds of sensor with a single call: for example,
    /// distance sensors are not Generic sensors any more than they are motors, so they will
    /// return zero.
    pub fn vexDeviceGenericValueGet(device: V5_DeviceT) -> i32;
}
