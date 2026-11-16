// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! EXP Brain Sound

use core::ffi::c_char;

use crate::platform::jump_table;

jump_table! {
    0x62c => pub fn vexSoundPlay(slot: i32, volume: u32);
    0x630 => pub fn vexSoundPlayTime(note: i32, volume: u32, duration: u32);
    0x634 => pub fn vexSoundPlayFile(filename: *mut c_char, volume: u32, source: u32);
    0x638 => pub fn vexSoundPlayNote(octave: i32, note: i32, volume: u32, duration: u32);
    0x63c => pub fn vexSoundIsActive() -> bool;
    0xed8 => pub fn vexSoundDuration() -> u32;
    0xedc => pub fn vexSoundRemaining() -> u32;
    0xee0 => pub fn vexSoundChangeVolume(volume: u32);
}
