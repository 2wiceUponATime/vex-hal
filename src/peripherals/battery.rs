// SPDX-FileCopyrightText: 2023 Gavin Niederman
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! Robot battery information.
//!
//! This module provides APIs for retrieving information about the robot's battery and state of
//! charge.

use crate::platform::{
    vexBatteryCapacityGet, vexBatteryCurrentGet, vexBatteryTemperatureGet, vexBatteryVoltageGet,
};

/// Returns the charge capacity of the robot's battery in the range of [0.0, 1.0].
///
/// A value of `0.0` indicates a completely empty battery, while a value of `1.0` indicates a
/// fully-charged battery.
#[must_use]
pub fn capacity() -> f64 {
    (unsafe { vexBatteryCapacityGet() } as f64) / 100.0
}

/// Returns the internal temperature of the robot's battery in degrees Celsius.
#[must_use]
pub fn temperature() -> u64 {
    (unsafe { vexBatteryTemperatureGet() } as u64)
}

/// Returns the electric current of the robot's battery in amps.
///
/// Maximum current draw on the V5 battery is 20 Amps.
#[must_use]
pub fn current() -> f64 {
    f64::from(unsafe { vexBatteryCurrentGet() }) / 1000.0
}

/// Returns the robot's battery voltage in volts.
///
/// Nominal battery voltage on the V5 brain is 12.8V.
#[must_use]
pub fn voltage() -> f64 {
    f64::from(unsafe { vexBatteryVoltageGet() }) / 1000.0
}
