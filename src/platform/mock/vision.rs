// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! V5 Vision Sensor

use core::ffi::c_uchar;

use crate::platform::{V5_DeviceT, c_enum};

c_enum! {
    pub struct V5VisionMode: c_uchar {
        kVisionModeNormal = 0,
        kVisionModeMixed = 1,
        kVisionModeLineDetect = 2,
        kVisionTypeTest = 3,
    }
}

c_enum! {
    pub struct V5VisionBlockType: c_uchar {
        kVisionTypeNormal = 0,
        kVisionTypeColorCode = 1,
        kVisionTypeLineDetect = 2,
    }
}

#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5_DeviceVisionObject {
    /// block signature
    pub signature: u16,
    /// block type
    pub r#type: V5VisionBlockType,
    /// left side of block
    pub xoffset: u16,
    /// top of block
    pub yoffset: u16,
    /// width of block
    pub width: u16,
    /// height of block
    pub height: u16,
    /// angle of CC block in 0.1 deg units
    pub angle: u16,
}

#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, PartialEq)]
#[allow(non_snake_case)]
pub struct V5_DeviceVisionSignature {
    pub id: u8,
    pub flags: u8,
    pub pad: [u8; 2],
    pub range: f32,
    pub uMin: i32,
    pub uMax: i32,
    pub uMean: i32,
    pub vMin: i32,
    pub vMax: i32,
    pub vMean: i32,
    pub mRgb: u32,
    pub mType: u32,
}

c_enum! {
    pub struct V5VisionWBMode: c_uchar {
        kVisionWBNormal = 0,
        kVisionWBStart = 1,
        kVisionWBManual = 2,
    }
}

#[repr(C, packed)]
#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub struct V5_DeviceVisionRgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub brightness: u8,
}

c_enum! {
    pub struct V5VisionLedMode: c_uchar {
        kVisionLedModeAuto = 0,
        kVisionLedModeManual = 1,
    }
}

c_enum! {
    pub struct V5VisionWifiMode: c_uchar {
        kVisionWifiModeOff = 0,
        kVisionWifiModeOn = 1,
    }
}

mock! {
    pub trait MockVision;
    pub fn vexDeviceVisionModeSet(device: V5_DeviceT, mode: V5VisionMode);
    pub fn vexDeviceVisionModeGet(device: V5_DeviceT) -> V5VisionMode;
    pub fn vexDeviceVisionObjectCountGet(device: V5_DeviceT) -> i32;
    pub fn vexDeviceVisionObjectGet(
        device: V5_DeviceT,
        index: u32,
        object: *mut V5_DeviceVisionObject,
    ) -> i32;
    pub fn vexDeviceVisionSignatureSet(
        device: V5_DeviceT,
        signature: *mut V5_DeviceVisionSignature,
    );
    pub fn vexDeviceVisionSignatureGet(
        device: V5_DeviceT,
        id: u32,
        signature: *mut V5_DeviceVisionSignature,
    ) -> bool;
    pub fn vexDeviceVisionBrightnessSet(device: V5_DeviceT, percent: u8);
    pub fn vexDeviceVisionBrightnessGet(device: V5_DeviceT) -> u8;
    pub fn vexDeviceVisionWhiteBalanceModeSet(device: V5_DeviceT, mode: V5VisionWBMode);
    pub fn vexDeviceVisionWhiteBalanceModeGet(device: V5_DeviceT) -> V5VisionWBMode;
    pub fn vexDeviceVisionWhiteBalanceSet(device: V5_DeviceT, color: V5_DeviceVisionRgb);
    pub fn vexDeviceVisionWhiteBalanceGet(device: V5_DeviceT) -> V5_DeviceVisionRgb;
    pub fn vexDeviceVisionLedModeSet(device: V5_DeviceT, mode: V5VisionLedMode);
    pub fn vexDeviceVisionLedModeGet(device: V5_DeviceT) -> V5VisionLedMode;
    pub fn vexDeviceVisionLedBrigntnessSet(device: V5_DeviceT, percent: u8);
    pub fn vexDeviceVisionLedBrigntnessGet(device: V5_DeviceT) -> u8;
    pub fn vexDeviceVisionLedColorSet(device: V5_DeviceT, color: V5_DeviceVisionRgb);
    pub fn vexDeviceVisionLedColorGet(device: V5_DeviceT) -> V5_DeviceVisionRgb;
    pub fn vexDeviceVisionWifiModeSet(device: V5_DeviceT, mode: V5VisionWifiMode);
    pub fn vexDeviceVisionWifiModeGet(device: V5_DeviceT) -> V5VisionWifiMode;
}
