// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! ADI Devices

use core::ffi::{c_double, c_uchar};

use crate::platform::{V5_DeviceT, c_enum};

c_enum! {
    pub struct V5_AdiPortConfiguration: c_uchar {
        kAdiPortTypeAnalogIn = 0,
        kAdiPortTypeAnalogOut = 1,
        kAdiPortTypeDigitalIn = 2,
        kAdiPortTypeDigitalOut = 3,
        kAdiPortTypeSmartButton = 4,
        kAdiPortTypeSmartPot = 5,
        kAdiPortTypeLegacyButton = 6,
        kAdiPortTypeLegacyPotentiometer = 7,
        kAdiPortTypeLegacyLineSensor = 8,
        kAdiPortTypeLegacyLightSensor = 9,
        kAdiPortTypeLegacyGyro = 10,
        kAdiPortTypeLegacyAccelerometer = 11,
        kAdiPortTypeLegacyServo = 12,
        kAdiPortTypeLegacyPwm = 13,
        kAdiPortTypeQuadEncoder = 14,
        kAdiPortTypeSonar = 15,
        kAdiPortTypeLegacyPwmSlew = 16,
        kAdiPortTypeUndefined = 255,
    }
}

c_enum! {
    pub struct V5_DeviceBumperState: c_uchar {
        kBumperReleased = 0,
        kBumperPressed = 1,
    }
}

mock! {
    Adi:
    pub fn vexDeviceAdiPortConfigSet(
        device: V5_DeviceT,
        port: u32,
        config: V5_AdiPortConfiguration,
    );
    pub fn vexDeviceAdiPortConfigGet(
        device: V5_DeviceT,
        port: u32,
    ) -> V5_AdiPortConfiguration;
    pub fn vexDeviceAdiValueSet(device: V5_DeviceT, port: u32, value: i32);
    pub fn vexDeviceAdiValueGet(device: V5_DeviceT, port: u32) -> i32;
    /// <Derived from <https://github.com/purduesigbots/pros/blob/89a7417352fbbc86420325afe410861e2210743c/src/devices/vdml_ext_adi.c#L40>
    pub fn vexDeviceAdiAddrLedSet(
        device: V5_DeviceT,
        port: u32,
        pData: *mut u32,
        nOffset: u32,
        nLength: u32,
        options: u32,
    );
    pub fn vexDeviceBumperGet(device: V5_DeviceT) -> V5_DeviceBumperState;
    pub fn vexDeviceGyroReset(device: V5_DeviceT);
    pub fn vexDeviceGyroHeadingGet(device: V5_DeviceT) -> c_double;
    pub fn vexDeviceGyroDegreesGet(device: V5_DeviceT) -> c_double;
    pub fn vexDeviceSonarValueGet(device: V5_DeviceT) -> i32;
}
