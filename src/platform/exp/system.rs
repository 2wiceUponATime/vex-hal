// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

use super::jump_table;

jump_table! {
    /// Exits the current user program, returning to the main program screen.
    0x130 => pub fn vexSystemExitRequest();
}
