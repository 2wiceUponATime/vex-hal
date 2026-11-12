// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

use crate::platform::jump_table;

jump_table! {
    /// Writes a single byte to the serial FIFO output buffer.
    ///
    /// # Arguments
    ///
    /// - `channel`: The serial communications channel to write to. Use `1` for stdio.
    /// - `c`: The byte to write.
    ///
    /// # Return
    ///
    /// The number of bytes written, or -1 if an internal error occurred.
    0x898 => pub fn vexSerialWriteChar(channel: u32, c: u8) -> i32;
    /// Writes an arbitrary buffer to the serial FIFO output buffer, returning how
    /// many bytes were written. The output buffer has a maximum size of 2048 bytes,
    /// meaning that a larger buffer may be truncated.
    ///
    /// # Arguments
    ///
    /// - `channel`: The serial communications channel to write to. Use `1` for stdio.
    /// - `data`: A buffer of bytes to write.
    /// - `data_len`: The length of the specified buffer.
    ///
    /// # Return
    ///
    /// The number of bytes written, or -1 if an internal error occurred.
    0x89c => pub fn vexSerialWriteBuffer(channel: u32, data: *const u8, data_len: u32) -> i32;
    /// Reads a single byte from the input buffer.
    ///
    /// # Arguments
    ///
    /// - `channel`: The serial communications channel to read from. Use `1` for stdio.
    ///
    /// # Return
    ///
    /// The next byte in the input buffer, or -1 if no character is available to be read.
    0x8a0 => pub fn vexSerialReadChar(channel: u32) -> i32;
    /// Returns the next available byte to be read in the input buffer without removing
    /// it from the buffer.
    ///
    /// # Arguments
    ///
    /// - `channel`: The serial communications channel to read from. Use `1` for stdio.
    ///
    /// # Return
    ///
    /// The next byte in the input buffer, or -1 if no character is available to be read.
    0x8a4 => pub fn vexSerialPeekChar(channel: u32) -> i32;
    /// Returns the number of free bytes (out of `2048`) remaining in the serial output
    /// buffer.
    ///
    /// # Arguments
    ///
    /// - `channel`: The serial communications channel to read from. Use `1` for stdio.
    ///
    /// # Return
    ///
    /// The number of remaining available bytes.
    0x8ac => pub fn vexSerialWriteFree(channel: u32) -> i32;
}
