// SPDX-FileCopyrightText: 2023 Gavin Niederman
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

use core::time::Duration;

use bitflags::bitflags;
use snafu::{Snafu, ensure};

use crate::{
    math::{Angle, Direction},
    peripherals::{PortError, SmartDevice, SmartDeviceType, SmartPort},
    platform::*,
};

/// A motor plugged into a Smart Port.
#[derive(Debug, PartialEq)]
pub struct Motor {
    port: SmartPort,
    target: MotorControl,
    device: V5_DeviceT,
    motor_type: MotorType,
}

// SAFETY: Required because we store a raw pointer to the device handle to avoid it getting from the
// SDK each device function. Simply sharing a raw pointer across threads is not inherently unsafe.
unsafe impl Send for Motor {}
unsafe impl Sync for Motor {}

/// A possible target action for a [`Motor`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MotorControl {
    /// The motor brakes using a specified [`BrakeMode`].
    Brake(BrakeMode),

    /// The motor outputs a raw voltage (specified in volts).
    ///
    /// # Fields
    ///
    /// - `0`: The desired output voltage of the motor
    Voltage(f64),

    /// The motor attempts to hold a velocity using its internal PID control.
    ///
    /// # Fields
    ///
    /// - `0`: The desired speed of the motor during the movement operation
    Velocity(i32),

    /// The motor attempts to reach a position using its internal PID control.
    ///
    /// # Fields
    ///
    /// - `0`: The desired position of the motor after the movement operation
    /// - `1`: The desired speed of the velocity in RPM during the movement operation
    Position(Angle, i32),
}

/// Represents the type of a Smart motor.
/// Either a 11W (V5) or 5.5W (EXP) motor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MotorType {
    /// A 5.5W Smart Motor
    Exp,
    /// An 11W Smart Motor
    V5,
}

impl MotorType {
    /// Returns `true` if the motor is a 5.5W (EXP) Smart Motor.
    #[must_use]
    pub const fn is_exp(&self) -> bool {
        matches!(self, Self::Exp)
    }

    /// Returns `true` if the motor is an 11W (V5) Smart Motor.
    #[must_use]
    pub const fn is_v5(&self) -> bool {
        matches!(self, Self::V5)
    }

    /// Returns the maximum voltage for a motor of this type.
    #[must_use]
    pub const fn max_voltage(&self) -> f64 {
        match self {
            MotorType::Exp => Motor::EXP_MAX_VOLTAGE,
            MotorType::V5 => Motor::V5_MAX_VOLTAGE,
        }
    }
}

impl Motor {
    /// The maximum voltage value that can be sent to a V5 [`Motor`].
    pub const V5_MAX_VOLTAGE: f64 = 12.0;
    /// The maximum voltage value that can be sent to a EXP [`Motor`].
    pub const EXP_MAX_VOLTAGE: f64 = 8.0;

    /// The interval at which the Brain will send new packets to a [`Motor`].
    pub const WRITE_INTERVAL: Duration = Duration::from_millis(5);

    /// Create a new V5 or EXP motor.
    #[must_use]
    fn new_with_type(
        port: SmartPort,
        gearset: Gearset,
        direction: Direction,
        motor_type: MotorType,
    ) -> Self {
        let device = unsafe { port.device_handle() }; // SAFETY: This function is only called once on this port.

        // NOTE: SDK properly stores device state when unplugged, meaning that we can safely set
        // these without consequence even if the device is not available. This is an edge case for
        // the SDK though, and seems to just be a thing for motors and rotation sensors.
        unsafe {
            vexDeviceMotorEncoderUnitsSet(device, V5MotorEncoderUnits::kMotorEncoderCounts);

            vexDeviceMotorReverseFlagSet(device, direction.is_reverse());
            vexDeviceMotorGearingSet(device, gearset.into());
        }

        Self {
            port,
            target: MotorControl::Voltage(0.0),
            device,
            motor_type,
        }
    }

    /// Creates a new 11W (V5) Smart Motor.
    ///
    /// See [`Motor::new_exp`] to create a 5.5W (EXP) Smart Motor.
    #[must_use]
    pub fn new(port: SmartPort, gearset: Gearset, direction: Direction) -> Self {
        Self::new_with_type(port, gearset, direction, MotorType::V5)
    }

    /// Creates a new 5.5W (EXP) Smart Motor.
    ///
    /// See [`Motor::new`] to create a 11W (V5) Smart Motor.
    #[must_use]
    pub fn new_exp(port: SmartPort, direction: Direction) -> Self {
        Self::new_with_type(port, Gearset::Green, direction, MotorType::Exp)
    }

    /// Sets the motor's output voltage.
    ///
    /// This voltage value spans from -12 (fully spinning reverse) to +12 (fully spinning forwards)
    /// volts, and controls the raw output of the motor.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn set_voltage(&mut self, volts: f64) -> Result<(), PortError> {
        self.set_target(MotorControl::Voltage(volts))
    }

    /// Spins the motor at a target velocity.
    ///
    /// This velocity corresponds to different actual speeds in RPM depending on the gearset used
    /// for the motor. Velocity is held with an internal PID controller to ensure consistent
    /// speed, as opposed to setting the motor's voltage.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn set_velocity(&mut self, rpm: i32) -> Result<(), PortError> {
        self.set_target(MotorControl::Velocity(rpm))
    }

    /// Stops this motor with the given [`BrakeMode`].
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn brake(&mut self, mode: BrakeMode) -> Result<(), PortError> {
        self.set_target(MotorControl::Brake(mode))
    }

    /// Sets an absolute position target for the motor to attempt to reach.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn set_position_target(&mut self, position: Angle, velocity: i32) -> Result<(), PortError> {
        self.set_target(MotorControl::Position(position, velocity))
    }

    /// Changes the output velocity for a profiled movement (motor_move_absolute or
    /// motor_move_relative).
    ///
    /// This will have no effect if the motor is not following a profiled movement.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn set_profiled_velocity(&mut self, velocity: i32) -> Result<(), PortError> {
        self.validate_port()?;

        unsafe {
            vexDeviceMotorVelocityUpdate(self.device, velocity);
        }

        if let MotorControl::Position(position, _) = self.target {
            self.target = MotorControl::Position(position, velocity);
        }

        Ok(())
    }

    /// Sets the target that the motor should attempt to reach.
    ///
    /// This could be a voltage, velocity, position, or even brake mode.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn set_target(&mut self, target: MotorControl) -> Result<(), PortError> {
        let gearset = self.gearset()?;
        self.target = target;

        match target {
            MotorControl::Brake(mode) => unsafe {
                vexDeviceMotorBrakeModeSet(self.device, mode.into());
                // Force motor into braking by putting it into velocity control with a 0rpm
                // setpoint.
                vexDeviceMotorVelocitySet(self.device, 0);
            },
            MotorControl::Velocity(rpm) => unsafe {
                vexDeviceMotorBrakeModeSet(self.device, V5MotorBrakeMode::kV5MotorBrakeModeCoast);
                vexDeviceMotorVelocitySet(self.device, rpm);
            },
            MotorControl::Voltage(volts) => unsafe {
                vexDeviceMotorBrakeModeSet(self.device, V5MotorBrakeMode::kV5MotorBrakeModeCoast);
                vexDeviceMotorVoltageSet(self.device, (volts * 1000.0) as i32);
            },
            MotorControl::Position(position, velocity) => unsafe {
                vexDeviceMotorBrakeModeSet(self.device, V5MotorBrakeMode::kV5MotorBrakeModeCoast);
                // position will not reach large enough values to cause loss of precision during
                // normal operation
                #[allow(clippy::cast_precision_loss)]
                vexDeviceMotorAbsoluteTargetSet(
                    self.device,
                    position.as_ticks(gearset.ticks_per_revolution()),
                    velocity,
                );
            },
        }

        Ok(())
    }

    /// Returns the current [`MotorControl`] target that the motor is attempting to use.
    ///
    /// This value is set with [`Motor::set_target`].
    #[must_use]
    pub const fn target(&self) -> MotorControl {
        self.target
    }

    /// Returns the gearset of the motor
    ///
    /// For 5.5W motors, this will always be returned as [`Gearset::Green`].
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn gearset(&self) -> Result<Gearset, PortError> {
        if self.motor_type.is_exp() {
            return Ok(Gearset::Green);
        }
        self.validate_port()?;
        Ok(unsafe { vexDeviceMotorGearingGet(self.device) }.into())
    }

    /// Sets the gearset of an 11W motor.
    ///
    /// # Errors
    ///
    /// - A [`SetGearsetError::Port`] error is returned if there was not a motor connected to the
    ///   port.
    /// - A [`SetGearsetError::SetGearsetExp`] is returned if the motor is a 5.5W EXP Smart Motor,
    ///   which has no swappable gearset.
    pub fn set_gearset(&mut self, gearset: Gearset) -> Result<(), SetGearsetError> {
        ensure!(self.motor_type.is_v5(), SetGearsetExpSnafu);
        self.validate_port()?;
        unsafe {
            vexDeviceMotorGearingSet(self.device, gearset.into());
        }
        Ok(())
    }

    /// Returns the [`Direction`] of this motor.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn direction(&self) -> Result<Direction, PortError> {
        self.validate_port()?;

        Ok(match unsafe { vexDeviceMotorReverseFlagGet(self.device) } {
            false => Direction::Forward,
            true => Direction::Reverse,
        })
    }

    /// Sets the motor to operate in a given [`Direction`].
    ///
    /// This determines which way the motor considers to be “forwards”. You can use the marking on
    /// the back of the motor as a reference:
    ///
    /// - When [`Direction::Forward`] is specified, positive velocity/voltage values will cause the
    ///   motor to rotate **with the arrow on the back**. Position will increase as the motor
    ///   rotates **with the arrow**.
    /// - When [`Direction::Reverse`] is specified, positive velocity/voltage values will cause the
    ///   motor to rotate **against the arrow on the back**. Position will increase as the motor
    ///   rotates **against the arrow**.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn set_direction(&mut self, direction: Direction) -> Result<(), PortError> {
        self.validate_port()?;

        unsafe {
            vexDeviceMotorReverseFlagSet(self.device, direction.is_reverse());
        }

        Ok(())
    }

    /// Returns the angular position of the motor as measured by the IME (integrated motor encoder).
    ///
    /// This is returned as an instance of the [`Angle`] struct, which may be converted to degrees,
    /// radians, revolutions, etc...
    ///
    /// # Gearing affects position!
    ///
    /// Position measurements are dependent on the Motor's [`Gearset`], and may be reported
    /// incorrectly if the motor is configured with the incorrect gearset variant. Make sure
    /// that the motor is configured with the same gearset as its physical cartridge color.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn position(&self) -> Result<Angle, PortError> {
        let gearset = self.gearset()?;
        Ok(Angle::from_ticks(
            unsafe { vexDeviceMotorPositionGet(self.device) },
            gearset.ticks_per_revolution(),
        ))
    }

    /// Sets the current encoder position to the given position without moving the motor.
    ///
    /// Analogous to taring or resetting the encoder so that the new position is equal to the given
    /// position.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn set_position(&mut self, position: Angle) -> Result<(), PortError> {
        let gearset = self.gearset()?;

        unsafe {
            vexDeviceMotorPositionSet(
                self.device,
                // NOTE: No precision loss since ticks are not fractional.
                position.as_ticks(gearset.ticks_per_revolution()),
            );
        }

        Ok(())
    }

    /// Sets the current encoder position to zero without moving the motor.
    ///
    /// Analogous to taring or resetting the encoder to the current position.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn reset_position(&mut self) -> Result<(), PortError> {
        self.validate_port()?;
        unsafe { vexDeviceMotorPositionReset(self.device) }
        Ok(())
    }

    /// Returns the most recently recorded raw encoder tick data from the motor's IME.
    ///
    /// The motor's integrated encoder has a TPR of 4096. Gearset is not taken into consideration
    /// when dealing with the raw value, meaning this measurement will be taken relative to the
    /// motor's internal position *before* being geared down from 3600RPM.
    ///
    /// Methods such as [`Motor::reset_position`] and [`Motor::set_position`] do not
    /// change the value of this raw measurement.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn raw_position(&self) -> Result<i32, PortError> {
        self.validate_port()?;

        Ok(unsafe { vexDeviceMotorPositionRawGet(self.device, core::ptr::null_mut()) })
    }

    /// Returns the motor's estimate of its angular velocity in rotations per minute (RPM).
    ///
    /// # Accuracy
    ///
    /// In some cases, this reported value may be noisy or inaccurate, especially for systems where
    /// accurate velocity control at high speeds is required (such as flywheels). If the
    /// accuracy of this value proves inadequate, you may opt to perform your own velocity
    /// calculations by differentiating [`Motor::position`] over the reported internal timestamp
    /// of the motor using [`Motor::timestamp`].
    ///
    /// > For more information about Smart motor velocity estimation, see [this article](https://sylvie.fyi/sylib/docs/db/d8e/md_module_writeups__velocity__estimation.html).
    ///
    /// # Note
    ///
    /// To get the current **target** velocity instead of the estimated velocity, use
    /// [`Motor::target`].
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn velocity(&self) -> Result<f64, PortError> {
        self.validate_port()?;
        Ok(unsafe { vexDeviceMotorActualVelocityGet(self.device) })
    }

    /// Returns the power drawn by the motor in Watts.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn power(&self) -> Result<f64, PortError> {
        self.validate_port()?;
        Ok(unsafe { vexDeviceMotorPowerGet(self.device) })
    }

    /// Returns the torque output of the motor in Nm.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn torque(&self) -> Result<f64, PortError> {
        self.validate_port()?;
        Ok(unsafe { vexDeviceMotorTorqueGet(self.device) })
    }

    /// Returns the voltage the motor is drawing in volts.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn voltage(&self) -> Result<f64, PortError> {
        self.validate_port()?;
        Ok(f64::from(unsafe { vexDeviceMotorVoltageGet(self.device) }) / 1000.0)
    }

    /// Returns the electrical current draw of the motor in amps.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn current(&self) -> Result<f64, PortError> {
        self.validate_port()?;
        Ok(f64::from(unsafe { vexDeviceMotorCurrentGet(self.device) }) / 1000.0)
    }

    /// Returns the efficiency of the motor from a range of [0.0, 1.0].
    ///
    /// An efficiency of 1.0 means that the motor is moving electrically while drawing no electrical
    /// power, and an efficiency of 0.0 means that the motor is drawing power but not moving.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn efficiency(&self) -> Result<f64, PortError> {
        self.validate_port()?;

        Ok(unsafe { vexDeviceMotorEfficiencyGet(self.device) } / 100.0)
    }

    /// Returns the current limit for the motor in amps.
    ///
    /// This limit can be configured with the [`Motor::set_current_limit`] method.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn current_limit(&self) -> Result<f64, PortError> {
        self.validate_port()?;
        Ok(f64::from(unsafe { vexDeviceMotorCurrentLimitGet(self.device) }) / 1000.0)
    }

    /// Sets the current limit for the motor in amps.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn set_current_limit(&mut self, limit: f64) -> Result<(), PortError> {
        self.validate_port()?;
        unsafe { vexDeviceMotorCurrentLimitSet(self.device, (limit * 1000.0) as i32) }
        Ok(())
    }

    /// Returns the voltage limit for the motor if one has been explicitly set.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn voltage_limit(&self) -> Result<f64, PortError> {
        self.validate_port()?;
        Ok(f64::from(unsafe { vexDeviceMotorVoltageLimitGet(self.device) }) / 1000.0)
    }

    /// Sets the voltage limit for the motor in volts.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn set_voltage_limit(&mut self, limit: f64) -> Result<(), PortError> {
        self.validate_port()?;

        unsafe {
            vexDeviceMotorVoltageLimitSet(self.device, (limit * 1000.0) as i32);
        }

        Ok(())
    }

    /// Returns the type of the motor.
    ///
    /// This does not check the hardware, it simply returns the type that the motor was created
    /// with.
    #[must_use]
    pub const fn motor_type(&self) -> MotorType {
        self.motor_type
    }

    /// Returns `true` if the motor is a 5.5W (EXP) Smart Motor.
    #[must_use]
    pub const fn is_exp(&self) -> bool {
        self.motor_type.is_exp()
    }

    /// Returns `true` if the motor is an 11W (V5) Smart Motor.
    #[must_use]
    pub const fn is_v5(&self) -> bool {
        self.motor_type.is_v5()
    }

    /// Returns the maximum voltage for the motor based off of its [motor type](Motor::motor_type).
    #[must_use]
    pub const fn max_voltage(&self) -> f64 {
        self.motor_type.max_voltage()
    }

    /// Returns the internal temperature recorded by the motor in degrees Celsius.
    ///
    /// The returned temperature has a resolution of 5°C.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn temperature(&self) -> Result<f64, PortError> {
        self.validate_port()?;
        Ok(unsafe { vexDeviceMotorTemperatureGet(self.device) })
    }

    /// Returns the status flags of a motor.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn status(&self) -> Result<MotorStatus, PortError> {
        self.validate_port()?;

        Ok(MotorStatus::from_bits_retain(unsafe {
            vexDeviceMotorFlagsGet(self.device)
        }))
    }

    /// Returns the fault flags of the motor.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn faults(&self) -> Result<MotorFaults, PortError> {
        self.validate_port()?;

        Ok(MotorFaults::from_bits_retain(unsafe {
            vexDeviceMotorFaultsGet(self.device)
        }))
    }

    /// Returns `true` if the motor's over temperature flag is set.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn is_over_temperature(&self) -> Result<bool, PortError> {
        Ok(self.faults()?.contains(MotorFaults::OVER_TEMPERATURE))
    }

    /// Returns `true` if the motor's over-current flag is set.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn is_over_current(&self) -> Result<bool, PortError> {
        Ok(self.faults()?.contains(MotorFaults::OVER_CURRENT))
    }

    /// Returns `true` if a H-bridge (motor driver) fault has occurred.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn is_driver_fault(&self) -> Result<bool, PortError> {
        Ok(self.faults()?.contains(MotorFaults::DRIVER_FAULT))
    }

    /// Returns `true` if the motor's H-bridge has an over-current fault.
    ///
    /// # Errors
    ///
    /// - A [`PortError::Disconnected`] error is returned if no device was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if the wrong type of device was
    ///   connected to the port.
    pub fn is_driver_over_current(&self) -> Result<bool, PortError> {
        Ok(self.faults()?.contains(MotorFaults::DRIVER_OVER_CURRENT))
    }
}

impl SmartDevice for Motor {
    fn port_number(&self) -> u8 {
        self.port.number()
    }

    fn device_type(&self) -> SmartDeviceType {
        SmartDeviceType::Motor
    }
}

impl From<Motor> for SmartPort {
    fn from(device: Motor) -> Self {
        device.port
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BrakeMode {
    /// Motor never brakes.
    Coast,

    /// Motor uses regenerative braking to slow down faster.
    Brake,

    /// Motor exerts force holding itself in the same position.
    Hold,
}

impl From<V5MotorBrakeMode> for BrakeMode {
    fn from(value: V5MotorBrakeMode) -> Self {
        match value {
            V5MotorBrakeMode::kV5MotorBrakeModeBrake => Self::Brake,
            V5MotorBrakeMode::kV5MotorBrakeModeCoast => Self::Coast,
            V5MotorBrakeMode::kV5MotorBrakeModeHold => Self::Hold,
            _ => unreachable!(),
        }
    }
}

impl From<BrakeMode> for V5MotorBrakeMode {
    fn from(value: BrakeMode) -> Self {
        match value {
            BrakeMode::Brake => Self::kV5MotorBrakeModeBrake,
            BrakeMode::Coast => Self::kV5MotorBrakeModeCoast,
            BrakeMode::Hold => Self::kV5MotorBrakeModeHold,
        }
    }
}

bitflags! {
    /// The fault flags returned by a [`Motor`].
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct MotorFaults: u32 {
        /// The motor's temperature is above its limit.
        const OVER_TEMPERATURE = 0x01;

        /// The motor is over current.
        const OVER_CURRENT = 0x04;

        /// The motor's H-bridge has encountered a fault.
        const DRIVER_FAULT = 0x02;

        /// The motor's H-bridge is over current.
        const DRIVER_OVER_CURRENT = 0x08;
    }
}

bitflags! {
    /// The status bits returned by a [`Motor`].
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
    pub struct MotorStatus: u32 {
        /// Failed to communicate with the motor
        const BUSY = 0x01;
    }
}

/// Internal gearset used by VEX Smart motors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gearset {
    /// 36:1 gear ratio
    Red,
    /// 18:1 gear ratio
    Green,
    /// 6:1 gear ratio
    Blue,
}

impl Gearset {
    /// 36:1 gear ratio (alias to `Self::Red`)
    pub const RATIO_36: Gearset = Self::Red;
    /// 18:1 gear ratio (alias to `Self::Green`)
    pub const RATIO_18: Gearset = Self::Green;
    /// 6:1 gear ratio (alias to `Self::Blue`)
    pub const RATIO_6: Gearset = Self::Blue;

    /// 100 rpm gearset (alias to `Self::Red`)
    pub const RPM_100: Gearset = Self::Red;
    /// 200 rpm (alias to `Self::Green`)
    pub const RPM_200: Gearset = Self::Green;
    /// 600 rpm (alias to `Self::Blue`)
    pub const RPM_600: Gearset = Self::Blue;

    /// Rated max speed for a Smart motor with a [`Red`](Gearset::Red) gearset.
    pub const MAX_RED_RPM: f64 = 100.0;
    /// Rated speed for a Smart motor with a [`Green`](Gearset::Green) gearset.
    pub const MAX_GREEN_RPM: f64 = 200.0;
    /// Rated speed for a Smart motor with a [`Blue`](Gearset::Blue) gearset.
    pub const MAX_BLUE_RPM: f64 = 600.0;

    /// Number of encoder ticks per revolution for the [`Red`](Gearset::Red) gearset.
    pub const RED_TICKS_PER_REVOLUTION: u32 = 1800;
    /// Number of encoder ticks per revolution for the [`Green`](Gearset::Green) gearset.
    pub const GREEN_TICKS_PER_REVOLUTION: u32 = 900;
    /// Number of encoder ticks per revolution for the [`Blue`](Gearset::Blue) gearset.
    pub const BLUE_TICKS_PER_REVOLUTION: u32 = 300;

    /// Returns the rated maximum speed for this motor gearset.
    #[must_use]
    pub const fn max_rpm(&self) -> f64 {
        match self {
            Self::Red => Self::MAX_RED_RPM,
            Self::Green => Self::MAX_GREEN_RPM,
            Self::Blue => Self::MAX_BLUE_RPM,
        }
    }

    /// Returns the number of encoder ticks per revolution for this motor gearset.
    #[must_use]
    pub const fn ticks_per_revolution(&self) -> u32 {
        match self {
            Self::Red => Self::RED_TICKS_PER_REVOLUTION,
            Self::Green => Self::GREEN_TICKS_PER_REVOLUTION,
            Self::Blue => Self::BLUE_TICKS_PER_REVOLUTION,
        }
    }
}

impl From<V5MotorGearset> for Gearset {
    fn from(value: V5MotorGearset) -> Self {
        match value {
            V5MotorGearset::kMotorGearSet_06 => Self::Blue,
            V5MotorGearset::kMotorGearSet_18 => Self::Green,
            V5MotorGearset::kMotorGearSet_36 => Self::Red,
            _ => unreachable!(),
        }
    }
}

impl From<Gearset> for V5MotorGearset {
    fn from(value: Gearset) -> Self {
        match value {
            Gearset::Blue => Self::kMotorGearSet_06,
            Gearset::Green => Self::kMotorGearSet_18,
            Gearset::Red => Self::kMotorGearSet_36,
        }
    }
}

/// Error returned by [`Motor::set_gearset`].
#[derive(Debug, Clone, Copy, Eq, PartialEq, Snafu)]
pub enum SetGearsetError {
    /// Generic port related error.
    #[snafu(transparent)]
    Port {
        /// The source of the error.
        source: PortError,
    },

    /// EXP motors do not have customizable gearsets.
    SetGearsetExp,
}
