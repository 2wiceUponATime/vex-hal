// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! V5 Controller

use super::jump_table;

jump_table! {
    /// Get the value of a controller's data channel.
    #[cfg(any())]
    0x000 => pub fn vexControllerGet(id: V5_ControllerId, index: V5_ControllerIndex) -> i32;
    /// Returns `1` if the controller on the given ID is connected, or `0` if not.
    #[cfg(any())]
    0x000 => pub fn vexControllerConnectionStatusGet(id: V5_ControllerId) -> V5_ControllerStatus;
    #[cfg(any())]
    0x000 => pub fn vexControllerTextSet(id: u32, line: u32, col: u32, buf: *const u8) -> u32;
}
