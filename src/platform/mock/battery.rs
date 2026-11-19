// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! V5 Smart Battery

use core::ffi::c_double;

mock! {
    pub fn vexBatteryVoltageGet() -> i32;
    pub fn vexBatteryCurrentGet() -> i32;
    pub fn vexBatteryTemperatureGet() -> c_double;
    pub fn vexBatteryCapacityGet() -> c_double;
}
