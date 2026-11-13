// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! V5 Smart Battery

use core::ffi::c_double;

use crate::platform::jump_table;

jump_table! {
    0xa00 => pub fn vexBatteryVoltageGet() -> i32;
    0xa04 => pub fn vexBatteryCurrentGet() -> i32;
    0xa08 => pub fn vexBatteryTemperatureGet() -> c_double;
    0xa0c => pub fn vexBatteryCapacityGet() -> c_double;
}
