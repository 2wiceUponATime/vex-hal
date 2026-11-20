// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! EXP Smart Motor

use core::{
    ffi::{c_double, c_uchar},
    fmt::Debug,
};

use crate::platform::{V5_DeviceT, c_enum};

c_enum! {
    pub struct V5MotorBrakeMode: c_uchar {
        kV5MotorBrakeModeCoast = 0,
        kV5MotorBrakeModeBrake = 1,
        kV5MotorBrakeModeHold = 2,
    }
}

c_enum! {
    pub struct V5MotorControlMode: c_uchar {
        kMotorControlModeOFF = 0,
        kMotorControlModeBRAKE = 1,
        kMotorControlModeHOLD = 2,
        kMotorControlModeSERVO = 3,
        kMotorControlModePROFILE = 4,
        kMotorControlModeVELOCITY = 5,
        kMotorControlModeUNDEFINED = 6,
    }
}

c_enum! {
    pub struct V5MotorEncoderUnits: c_uchar {
        kMotorEncoderDegrees = 0,
        kMotorEncoderRotations = 1,
        kMotorEncoderCounts = 2,
    }
}

c_enum! {
    pub struct V5MotorGearset: c_uchar {
        kMotorGearSet_36 = 0,
        kMotorGearSet_18 = 1,
        kMotorGearSet_06 = 2,
    }
}

#[repr(C, packed)]
#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub struct V5_DeviceMotorPid {
    pub kf: u8,
    pub kp: u8,
    pub ki: u8,
    pub kd: u8,
    pub filter: u8,
    pub pad1: u8,
    pub limit: u16,
    pub threshold: u8,
    pub loopspeed: u8,
    pub pad2: [u8; 2],
}

mock! {
    Motor:
    pub fn vexDeviceMotorVelocitySet(device: V5_DeviceT, velocity: i32);
    pub fn vexDeviceMotorVelocityGet(device: V5_DeviceT) -> i32;
    pub fn vexDeviceMotorActualVelocityGet(device: V5_DeviceT) -> c_double;
    pub fn vexDeviceMotorDirectionGet(device: V5_DeviceT) -> i32;
    pub fn vexDeviceMotorModeSet(device: V5_DeviceT, mode: V5MotorControlMode);
    pub fn vexDeviceMotorModeGet(device: V5_DeviceT) -> V5MotorControlMode;
    pub fn vexDeviceMotorPwmSet(device: V5_DeviceT, pwm: i32);
    pub fn vexDeviceMotorPwmGet(device: V5_DeviceT) -> i32;
    pub fn vexDeviceMotorCurrentLimitSet(device: V5_DeviceT, limit: i32);
    pub fn vexDeviceMotorCurrentLimitGet(device: V5_DeviceT) -> i32;
    pub fn vexDeviceMotorCurrentGet(device: V5_DeviceT) -> i32;
    pub fn vexDeviceMotorPowerGet(device: V5_DeviceT) -> c_double;
    pub fn vexDeviceMotorTorqueGet(device: V5_DeviceT) -> c_double;
    pub fn vexDeviceMotorEfficiencyGet(device: V5_DeviceT) -> c_double;
    pub fn vexDeviceMotorTemperatureGet(device: V5_DeviceT) -> c_double;
    pub fn vexDeviceMotorOverTempFlagGet(device: V5_DeviceT) -> bool;
    pub fn vexDeviceMotorCurrentLimitFlagGet(device: V5_DeviceT) -> bool;
    pub fn vexDeviceMotorZeroVelocityFlagGet(device: V5_DeviceT) -> bool;
    pub fn vexDeviceMotorZeroPositionFlagGet(device: V5_DeviceT) -> bool;
    pub fn vexDeviceMotorReverseFlagSet(device: V5_DeviceT, reverse: bool);
    pub fn vexDeviceMotorReverseFlagGet(device: V5_DeviceT) -> bool;
    pub fn vexDeviceMotorEncoderUnitsSet(device: V5_DeviceT, units: V5MotorEncoderUnits);
    pub fn vexDeviceMotorEncoderUnitsGet(device: V5_DeviceT) -> V5MotorEncoderUnits;
    pub fn vexDeviceMotorBrakeModeSet(device: V5_DeviceT, mode: V5MotorBrakeMode);
    pub fn vexDeviceMotorBrakeModeGet(device: V5_DeviceT) -> V5MotorBrakeMode;
    pub fn vexDeviceMotorPositionSet(device: V5_DeviceT, position: c_double);
    pub fn vexDeviceMotorPositionGet(device: V5_DeviceT) -> c_double;
    pub fn vexDeviceMotorPositionRawGet(device: V5_DeviceT, timestamp: *mut u32) -> i32;
    pub fn vexDeviceMotorPositionReset(device: V5_DeviceT);
    pub fn vexDeviceMotorTargetGet(device: V5_DeviceT) -> c_double;
    pub fn vexDeviceMotorServoTargetSet(device: V5_DeviceT, position: c_double);
    pub fn vexDeviceMotorAbsoluteTargetSet(
        device: V5_DeviceT,
        position: c_double,
        veloctiy: i32,
    );
    pub fn vexDeviceMotorRelativeTargetSet(
        device: V5_DeviceT,
        position: c_double,
        velocity: i32,
    );
    pub fn vexDeviceMotorFaultsGet(device: V5_DeviceT) -> u32;
    pub fn vexDeviceMotorFlagsGet(device: V5_DeviceT) -> u32;
    pub fn vexDeviceMotorVoltageSet(device: V5_DeviceT, voltage: i32);
    pub fn vexDeviceMotorVoltageGet(device: V5_DeviceT) -> i32;
    pub fn vexDeviceMotorGearingSet(device: V5_DeviceT, gearset: V5MotorGearset);
    pub fn vexDeviceMotorGearingGet(device: V5_DeviceT) -> V5MotorGearset;
    pub fn vexDeviceMotorVoltageLimitSet(device: V5_DeviceT, limit: i32);
    pub fn vexDeviceMotorVoltageLimitGet(device: V5_DeviceT) -> i32;
    pub fn vexDeviceMotorVelocityUpdate(device: V5_DeviceT, velocity: i32);
    pub fn vexDeviceMotorPositionPidSet(device: V5_DeviceT, pid: *mut V5_DeviceMotorPid);
    pub fn vexDeviceMotorVelocityPidSet(device: V5_DeviceT, pid: *mut V5_DeviceMotorPid);
    pub fn vexDeviceMotorExternalProfileSet(
        device: V5_DeviceT,
        position: c_double,
        velocity: i32,
    );
    pub fn vexDeviceMotorTypeGet(device: V5_DeviceT);
}
