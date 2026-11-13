// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! Competition Control

use crate::platform::jump_table;

jump_table! {
    0x9d8 => pub fn vexCompetitionStatus() -> u32;
    0x9dc => pub fn vexCompetitionControl(data: u32);
}
