// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! Competition Control

mock! {
    pub fn vexCompetitionStatus() -> u32;
    pub fn vexCompetitionControl(data: u32);
}
