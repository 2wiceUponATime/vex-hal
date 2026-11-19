// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! Filesystem Access

use core::ffi::{c_char, c_uchar};

use crate::platform::c_enum;

c_enum! {
    pub struct FRESULT: c_uchar {
        FR_OK = 0,
        FR_DISK_ERR = 1,
        FR_INT_ERR = 2,
        FR_NOT_READY = 3,
        FR_NO_FILE = 4,
        FR_NO_PATH = 5,
        FR_INVALID_NAME = 6,
        FR_DENIED = 7,
        FR_EXIST = 8,
        FR_INVALID_OBJECT = 9,
        FR_WRITE_PROTECTED = 10,
        FR_INVALID_DRIVE = 11,
        FR_NOT_ENABLED = 12,
        FR_NO_FILESYSTEM = 13,
        FR_MKFS_ABORTED = 14,
        FR_TIMEOUT = 15,
        FR_LOCKED = 16,
        FR_NOT_ENOUGH_CORE = 17,
        FR_TOO_MANY_OPEN_FILES = 18,
        FR_INVALID_PARAMETER = 19,
    }
}

pub type FIL = core::ffi::c_void;

mock! {
    pub fn vexFileMountSD() -> FRESULT;
    pub fn vexFileDirectoryGet(
        path: *const c_char,
        buffer: *mut c_char,
        len: u32,
    ) -> FRESULT;
    pub fn vexFileOpen(filename: *const c_char, mode: *const c_char) -> *mut FIL;
    pub fn vexFileOpenWrite(filename: *const c_char) -> *mut FIL;
    pub fn vexFileOpenCreate(filename: *const c_char) -> *mut FIL;
    pub fn vexFileClose(fdp: *mut FIL);
    // The SDK doesn't use `const char *`, but it can be assumed that `buf` is not mutated.
    pub fn vexFileWrite(buf: *const c_char, size: u32, nItems: u32, fdp: *mut FIL) -> i32;
    pub fn vexFileSize(fdp: *mut FIL) -> i32;
    pub fn vexFileSeek(fdp: *mut FIL, offset: u32, whence: i32) -> FRESULT;
    pub fn vexFileRead(buf: *mut c_char, size: u32, nItems: u32, fdp: *mut FIL) -> i32;
    pub fn vexFileDriveStatus(drive: u32) -> bool;
    pub fn vexFileTell(fdp: *mut FIL) -> i32;
    pub fn vexFileSync(fdp: *mut FIL);
    pub fn vexFileStatus(filename: *const c_char) -> u32;
}
