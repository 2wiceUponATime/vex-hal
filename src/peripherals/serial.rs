// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! Generic Serial Communication
//!
//! This module provides an interface for using V5 Smart Ports as serial communication ports over
//! RS-485. It allows bidirectional communication with any device that speaks serial over the V5's
//! RS-485 interface.
//!
//! # Hardware Description
//!
//! V5 Smart Ports provide half-duplex RS-485 serial communication at up to an allowed 921600 baud
//! for user programs.
//!
//! Writes to the serial port are buffered, but are automatically flushed by VEXos as fast as
//! possible (down to ~10µs or so).

use core::{
    pin::Pin,
    task::{Context, Poll},
};

use snafu::Snafu;

use crate::{
    peripherals::{SmartDevice, SmartDeviceType, SmartPort},
    platform::*,
};

/// A Smart Port configured as a generic RS-485 serial port.
///
/// This struct implements the [`std::io::Read`] and [`std::io::Write`] traits if the `std` feature
/// is enabled. Alternatively, the `embedded_io` enabled implementations of the
/// [`embedded_io::Read`] and [`embedded_io::Write`] for this struct if `vexide_devices` is being
/// used in a `no_std` environment.
///
/// [`embedded_io::Read`]: https://docs.rs/embedded-io/0.6.1/embedded_io/trait.Read.html
/// [`embedded_io::Write`]: https://docs.rs/embedded-io/0.6.1/embedded_io/trait.Write.html
#[derive(Debug, Eq, PartialEq)]
pub struct SerialPort {
    port: SmartPort,
    device: V5_DeviceT,
}

// SAFETY: Required because we store a raw pointer to the device handle to avoid it getting from the
// SDK each device function. Simply sharing a raw pointer across threads is not inherently unsafe.
unsafe impl Send for SerialPort {}
unsafe impl Sync for SerialPort {}

impl SerialPort {
    /// The maximum user-configurable baud rate for generic serial under normal conditions.
    pub const MAX_BAUD_RATE: u32 = 921_600;

    /// The length of the serial FIFO input and output buffers.
    pub const INTERNAL_BUFFER_SIZE: usize = 1024;

    /// Open and configure a generic serial port on a [`SmartPort`].
    ///
    /// This configures a [`SmartPort`] to act as a generic serial controller capable of
    /// sending/receiving data. Providing a baud rate, or the transmission rate of bits is required.
    /// The maximum allowed baud rate is 921600.
    pub fn open(port: SmartPort, baud_rate: u32) -> SerialPortOpenFuture {
        SerialPortOpenFuture {
            state: SerialPortOpenState::Configure { baud_rate },
            serial: Some(Self {
                device: unsafe { port.device_handle() },
                port,
            }),
        }
    }

    /// Configures the baud rate of the serial port.
    ///
    /// Baud rate determines the speed of communication over the data channel. Under normal
    /// conditions, user code is limited to a maximum baudrate of 921600.
    pub fn set_baud_rate(&mut self, baud_rate: u32) {
        unsafe {
            vexDeviceGenericSerialBaudrate(self.device, baud_rate as i32);
        }
    }

    /// Clears the internal input and output FIFO buffers.
    ///
    /// This can be useful to reset state and remove old, potentially unneeded data from the input
    /// FIFO buffer or to cancel sending any data in the output FIFO buffer.
    ///
    /// # This is not the same thing as "flushing".
    ///
    /// This function does not cause the data in the output buffer to be written. It simply clears
    /// the internal buffers. Unlike stdout, generic serial does not use buffered IO (the FIFO
    /// buffers are written as soon as possible).
    pub fn clear_buffers(&mut self) {
        unsafe {
            vexDeviceGenericSerialFlush(self.device);
        }
    }

    /// Read the next byte available in the serial port's input buffer, or `None` if the input
    /// buffer is empty.
    pub fn read_byte(&mut self) -> Option<u8> {
        let byte = unsafe { vexDeviceGenericSerialReadChar(self.device) };

        match byte {
            -1 => None,
            _ => Some(byte as u8),
        }
    }

    /// Read the next byte available in the port's input buffer without removing it. Returns `None`
    /// if the input buffer is empty.
    #[must_use]
    pub fn peek_byte(&self) -> Option<u8> {
        match unsafe { vexDeviceGenericSerialPeekChar(self.device) } {
            -1 => None,
            byte => Some(byte as u8),
        }
    }

    /// Write a single byte to the port's output buffer.
    ///
    /// # Errors
    ///
    /// - A [`SerialError::WriteFailed`] error is returned if the byte could not be written.
    pub fn write_byte(&mut self, byte: u8) -> Result<(), SerialError> {
        match unsafe { vexDeviceGenericSerialWriteChar(self.device, byte) } {
            -1 => WriteFailedSnafu.fail(),
            _ => Ok(()),
        }
    }

    /// Returns the number of bytes available to be read in the port's FIFO input buffer.
    ///
    /// # Errors
    ///
    /// - A [`SerialError::ReadFailed`] error is returned if the serial device's status could not be
    ///   read.
    pub fn unread_bytes(&self) -> Result<usize, SerialError> {
        match unsafe { vexDeviceGenericSerialReceiveAvail(self.device) } {
            -1 => ReadFailedSnafu.fail(),
            available => Ok(available as usize),
        }
    }

    /// Returns the number of bytes free in the port's FIFO output buffer.
    ///
    /// # Errors
    ///
    /// - A [`SerialError::ReadFailed`] error is returned if the serial device's status could not be
    ///   read.
    pub fn write_capacity(&self) -> Result<usize, SerialError> {
        match unsafe { vexDeviceGenericSerialWriteFree(self.device) } {
            -1 => ReadFailedSnafu.fail(),
            available => Ok(available as usize),
        }
    }
}

impl SmartDevice for SerialPort {
    fn port_number(&self) -> u8 {
        self.port.number()
    }

    fn device_type(&self) -> SmartDeviceType {
        SmartDeviceType::GenericSerial
    }
}
impl From<SerialPort> for SmartPort {
    fn from(device: SerialPort) -> Self {
        device.port
    }
}

/// Errors that can occur when interacting with a [`SerialPort`].
#[derive(Debug, Clone, Copy, Eq, PartialEq, Snafu)]
pub enum SerialError {
    /// Internal write error occurred.
    WriteFailed,

    /// Internal read error occurred.
    ReadFailed,
}

#[derive(Debug, Clone, Copy)]
enum SerialPortOpenState {
    Configure { baud_rate: u32 },
    Waiting,
}

/// Future that opens and configures a [`SerialPort`].
///
/// If the port was not previous configured as a generic serial port, this may take a few
/// milliseconds to complete.
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[derive(Debug)]
pub struct SerialPortOpenFuture {
    state: SerialPortOpenState,
    serial: Option<SerialPort>,
}

impl core::future::Future for SerialPortOpenFuture {
    type Output = SerialPort;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        // According to the `Future` docs, "Once a future has finished, clients
        // should not poll it again."
        // Therefore, it is okay to panic here since it is not our problem.
        let serial = this.serial.as_ref().unwrap();

        if let SerialPortOpenState::Configure { baud_rate } = this.state {
            unsafe {
                vexDeviceGenericSerialEnable(serial.device, 0);
                vexDeviceGenericSerialBaudrate(serial.device, baud_rate as i32);
            }

            this.state = SerialPortOpenState::Waiting;
        }

        if serial.validate_port().is_ok() {
            Poll::Ready(this.serial.take().unwrap())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
