// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! Filesystem Access

use super::jump_table;

jump_table! {
    #[cfg(any())]
    0x000 => pub fn vexFileMountSD() -> FRESULT;
    #[cfg(any())]
    0x000 => pub fn vexFileDirectoryGet(path: *const c_char, buffer: *mut c_char, len: u32) -> FRESULT;
    #[cfg(any())]
    0x000 => pub fn vexFileOpen(filename: *const c_char, mode: *const c_char) -> *mut FIL;
    #[cfg(any())]
    0x000 => pub fn vexFileOpenWrite(filename: *const c_char) -> *mut FIL;
    #[cfg(any())]
    0x000 => pub fn vexFileOpenCreate(filename: *const c_char) -> *mut FIL;
    #[cfg(any())]
    0x000 => pub fn vexFileClose(fdp: *mut FIL);
    #[cfg(any())]
    0x000 => pub fn vexFileWrite(buf: *mut c_char, size: u32, nItems: u32, fdp: *mut FIL) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexFileSize(fdp: *mut FIL) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexFileSeek(fdp: *mut FIL, offset: u32, whence: i32) -> FRESULT;
    #[cfg(any())]
    0x000 => pub fn vexFileRead(buf: *mut c_char, size: u32, nItems: u32, fdp: *mut FIL) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexFileDriveStatus(drive: u32) -> bool;
    #[cfg(any())]
    0x000 => pub fn vexFileTell(fdp: *mut FIL) -> i32;
    #[cfg(any())]
    0x000 => pub fn vexFileSync(fdp: *mut FIL);
    #[cfg(any())]
    0x000 => pub fn vexFileStatus(filename: *const c_char) -> u32;
}
