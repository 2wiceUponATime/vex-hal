use super::{V5_DeviceT, jump_table};

jump_table! {
    0x2d0 => pub fn vexDeviceMotorVelocitySet(device: V5_DeviceT, velocity: i32);
    0x2d4 => pub fn vexDeviceMotorVelocityGet(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorActualVelocityGet(device: V5_DeviceT) -> c_double;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorDirectionGet(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorModeSet(device: V5_DeviceT, mode: V5MotorControlMode);
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorModeGet(device: V5_DeviceT) -> V5MotorControlMode;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorPwmSet(device: V5_DeviceT, pwm: i32);
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorPwmGet(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorCurrentLimitSet(device: V5_DeviceT, limit: i32);
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorCurrentLimitGet(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorCurrentGet(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorPowerGet(device: V5_DeviceT) -> c_double;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorTorqueGet(device: V5_DeviceT) -> c_double;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorEfficiencyGet(device: V5_DeviceT) -> c_double;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorTemperatureGet(device: V5_DeviceT) -> c_double;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorOverTempFlagGet(device: V5_DeviceT) -> bool;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorCurrentLimitFlagGet(device: V5_DeviceT) -> bool;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorZeroVelocityFlagGet(device: V5_DeviceT) -> bool;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorZeroPositionFlagGet(device: V5_DeviceT) -> bool;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorReverseFlagSet(device: V5_DeviceT, reverse: bool);
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorReverseFlagGet(device: V5_DeviceT) -> bool;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorEncoderUnitsSet(device: V5_DeviceT, units: V5MotorEncoderUnits);
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorEncoderUnitsGet(device: V5_DeviceT) -> V5MotorEncoderUnits;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorBrakeModeSet(device: V5_DeviceT, mode: V5MotorBrakeMode);
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorBrakeModeGet(device: V5_DeviceT) -> V5MotorBrakeMode;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorPositionSet(device: V5_DeviceT, position: c_double);
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorPositionGet(device: V5_DeviceT) -> c_double;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorPositionRawGet(
        device: V5_DeviceT,
        timestamp: *mut u32,
    ) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorPositionReset(device: V5_DeviceT);
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorTargetGet(device: V5_DeviceT) -> c_double;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorServoTargetSet(device: V5_DeviceT, position: c_double);
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorAbsoluteTargetSet(
        device: V5_DeviceT,
        position: c_double,
        veloctiy: i32,
    );
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorRelativeTargetSet(
        device: V5_DeviceT,
        position: c_double,
        velocity: i32,
    );
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorFaultsGet(device: V5_DeviceT) -> u32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorFlagsGet(device: V5_DeviceT) -> u32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorVoltageSet(device: V5_DeviceT, voltage: i32);
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorVoltageGet(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorGearingSet(device: V5_DeviceT, gearset: V5MotorGearset);
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorGearingGet(device: V5_DeviceT) -> V5MotorGearset;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorVoltageLimitSet(device: V5_DeviceT, limit: i32);
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorVoltageLimitGet(device: V5_DeviceT) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorVelocityUpdate(device: V5_DeviceT, velocity: i32);
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorPositionPidSet(device: V5_DeviceT, pid: *mut V5_DeviceMotorPid);
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorVelocityPidSet(device: V5_DeviceT, pid: *mut V5_DeviceMotorPid);
    #[cfg(any())]
    0x000 => pub fn vexDeviceMotorExternalProfileSet(
        device: V5_DeviceT,
        position: c_double,
        velocity: i32,
    );
}
