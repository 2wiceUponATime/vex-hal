// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! Competition Control

use crate::platform::jump_table;

jump_table! {
    #[cfg(any())]
    0x000 => pub fn vexCompetitionStatus() -> u32;
    #[cfg(any())]
    0x000 => pub fn vexCompetitionControl(data: u32);
}
