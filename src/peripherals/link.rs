// SPDX-FileCopyrightText: 2023 Gavin Niederman
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! VEXlink Radio
//!
//! This module provides support for VEXlink, a point-to-point wireless communications protocol
//! between two VEXnet radios.
//!
//! # Hardware Overview
//!
//! There are two types of radios in a VEXlink connection: "manager" and "worker". A "manager" radio
//! can transmit data at up to 1040 bytes/s while a "worker" radio can transmit data at up to 520
//! bytes/s. A connection should only ever have both types of radios.
//!
//! In order to connect to a radio, VEXos hashes a given link name and uses it as an ID to verify
//! the connection. For this reason, you should try to create a unique name for each radio link to
//! avoid accidentally interfering, or being interfered with by, an unrelated VEXlink connection.
//! Ideally, you want a name that will never be used by another team.
//!
//! The lights on the radio can be used as a status indicator:
//! - Blinking red: The radio is waiting for a connection to be established.
//! - Alternating red and green quickly: The radio is connected to another radio and is the
//!   "manager" radio.
//! - Alternating red and green slowly: The radio is connected to another radio and is the "worker"
//!   radio.
//!
//! For further information, see <https://www.vexforum.com/t/vexlink-documentaton/84538>

extern crate alloc;

use core::time::Duration;

use alloc::ffi::CString;
use snafu::Snafu;

use crate::{
    peripherals::{SmartDevice, SmartDeviceType, SmartPort},
    platform::*,
};

/// VEXLink Wireless Radio Link
///
/// VEXLink is a point-to-point wireless communications protocol between two VEXNet radios. For
/// further information, see <https://www.vexforum.com/t/vexlink-documentaton/84538>
#[derive(Debug, Eq, PartialEq)]
pub struct RadioLink {
    port: SmartPort,
    device: V5_DeviceT,
}

// SAFETY: Required because we store a raw pointer to the device handle to avoid it getting from the
// SDK each device function. Simply sharing a raw pointer across threads is not inherently unsafe.
unsafe impl Send for RadioLink {}
unsafe impl Sync for RadioLink {}

impl RadioLink {
    /// The length of the link's FIFO input and output buffers.
    pub const INTERNAL_BUFFER_SIZE: usize = 512;

    /// Opens a radio link from a VEXNet radio plugged into a Smart Port. Once opened, other VEXNet
    /// functionality such as controller tethering on this specific radio will be disabled. Other
    /// radios connected to the Brain can take over this functionality.
    ///
    /// # Panics
    ///
    /// - Panics if a NUL (0x00) character was found anywhere in the specified `id`.
    #[must_use]
    pub fn open(port: SmartPort, id: &str, link_type: LinkType) -> Self {
        let id = CString::new(id)
            .expect("CString::new encountered NUL (U+0000) byte in non-terminating position.");

        unsafe {
            vexDeviceGenericRadioConnection(
                port.device_handle(),
                id.as_ptr().cast_mut(),
                match link_type {
                    LinkType::Worker => 0,
                    LinkType::Manager => 1,
                },
                true,
            );
        }

        Self {
            device: unsafe { port.device_handle() },
            port,
        }
    }

    /// Returns the number of bytes that are waiting to be read from the radio's input buffer.
    ///
    /// # Errors
    ///
    /// - A [`LinkError::ReadFailed`] error is returned if the input buffer could not be accessed.
    pub fn unread_bytes(&self) -> Result<usize, LinkError> {
        match unsafe { vexDeviceGenericRadioReceiveAvail(self.device) } {
            -1 => Err(LinkError::ReadFailed),
            unread => Ok(unread as usize),
        }
    }

    /// Returns the number of bytes free in the radio's output buffer.
    ///
    /// # Errors
    ///
    /// - A [`LinkError::ReadFailed`] error is returned if the output buffer could not be accessed.
    pub fn write_capacity(&self) -> Result<usize, LinkError> {
        match unsafe { vexDeviceGenericRadioWriteFree(self.device) } {
            -1 => Err(LinkError::ReadFailed),
            available => Ok(available as usize),
        }
    }

    /// Returns `true` if there is a link established with another radio.
    #[must_use]
    pub fn is_linked(&self) -> bool {
        unsafe { vexDeviceGenericRadioLinkStatus(self.device) }
    }
}

impl SmartDevice for RadioLink {
    const UPDATE_INTERVAL: Duration = Duration::from_millis(25);

    fn port_number(&self) -> u8 {
        self.port.number()
    }

    fn device_type(&self) -> SmartDeviceType {
        SmartDeviceType::GenericSerial
    }
}
impl From<RadioLink> for SmartPort {
    fn from(device: RadioLink) -> Self {
        device.port
    }
}

/// The type of radio link being established.
///
/// VEXLink is a point-to-point connection, with one "manager" robot and one "worker" robot.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LinkType {
    /// Manager Radio
    ///
    /// This end of the link has a 1040-bytes/sec data rate when communicating with a worker radio.
    Manager,

    /// Worker Radio
    ///
    /// This end of the link has a 520-bytes/sec data rate when communicating with a manager radio.
    Worker,
}

/// Errors that can occur when interacting with a [`RadioLink`].
#[derive(Debug, Clone, Copy, Eq, PartialEq, Snafu)]
pub enum LinkError {
    /// Not linked with another radio.
    NotLinked,

    /// Internal write error occurred.
    WriteFailed,

    /// Internal read error occurred.
    ReadFailed,
}
