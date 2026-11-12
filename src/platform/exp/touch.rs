// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! Brain Screen Touchscreen

use super::jump_table;

jump_table! {
    #[cfg(any())]
    0x000 => pub fn vexTouchUserCallbackSet(
        callback: unsafe extern "C" fn(V5_TouchEvent, i32, i32),
    );
    #[cfg(any())]
    0x000 => pub fn vexTouchDataGet(status: *mut V5_TouchStatus);
}
