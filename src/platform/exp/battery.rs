// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! V5 Smart Battery

use crate::platform::jump_table;

jump_table! {
    #[cfg(any())]
    0x000 => pub fn vexBatteryVoltageGet() -> i32;
    #[cfg(any())]
    0x000 => pub fn vexBatteryCurrentGet() -> i32;
    #[cfg(any())]
    0x000 => pub fn vexBatteryTemperatureGet() -> c_double;
    #[cfg(any())]
    0x000 => pub fn vexBatteryCapacityGet() -> c_double;
}
