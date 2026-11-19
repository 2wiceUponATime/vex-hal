// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! EXP Brain Sound

use core::ffi::c_char;


mock! {
    pub fn vexSoundPlay(slot: i32, volume: u32);
    pub fn vexSoundPlayTime(note: i32, volume: u32, duration: u32);
    pub fn vexSoundPlayFile(filename: *mut c_char, volume: u32, source: u32);
    pub fn vexSoundPlayNote(octave: i32, note: i32, volume: u32, duration: u32);
    pub fn vexSoundIsActive() -> bool;
    pub fn vexSoundDuration() -> u32;
    pub fn vexSoundRemaining() -> u32;
    pub fn vexSoundChangeVolume(volume: u32);
}
