// SPDX-FileCopyrightText: 2023 Gavin Niederman
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! ADI Pulse-width Modulation (PWM)
//!
//! This module provides an interface for generating 8-bit PWM signals through ADI ports.
//!
//! # Hardware Overview
//!
//! Pulse-width modulation (PWM) is a digital signaling technique that creates a variable width high
//! pulse over a fixed period, allowing you to communicate analog data over digital signals by
//! measuring the length of the pulse (how long it was high compared to how long it was low).
//!
//! PWM signals consist of two components:
//! - ON time (pulse width): When the signal is high
//!   ([`LogicLevel::High`](super::digital::LogicLevel::High)/3.3V)
//! - OFF time: When the signal is low
//!   ([`LogicLevel::Low`](super::digital::LogicLevel::Low)/0V)
//!
//! The ratio between ON time and OFF time (the "duty cycle") is used to encode
//! information for commands to devices:
//!
//! ```text
//!             |<-->| pulse width (0.94-2.03ms)
//! 3.3V  ┐     ┌────┐     ┌──┐       ┌──────┐
//! 0V    └─────┘    └─────┘  └───────┘      └────
//!             |<-------->| period (16ms)
//! ```

use crate::{
    peripherals::adi::{AdiDevice, AdiDeviceType, AdiPort, PortError},
    platform::vexDeviceAdiValueSet,
};

/// Generic PWM Output over ADI
#[derive(Debug, Eq, PartialEq)]
pub struct AdiPwmOut {
    port: AdiPort,
}

impl AdiPwmOut {
    /// Create a PWM output from an [`AdiPort`].
    #[must_use]
    pub fn new(port: AdiPort) -> Self {
        port.configure(AdiDeviceType::PwmOut);

        Self { port }
    }

    /// Sets the PWM output width.
    ///
    /// This value is sent over 16ms periods with pulse widths ranging from roughly 0.94ms to
    /// 2.03ms.
    ///
    /// # Errors
    ///
    /// These errors are only returned if the device is plugged into an
    /// [`AdiExpander`](crate::peripherals::expander::AdiExpander).
    ///
    /// - A [`PortError::Disconnected`] error is returned if no expander was connected to the port.
    /// - A [`PortError::IncorrectDevice`] error is returned if a device other than an expander was
    ///   connected to the port.
    pub fn set_output(&mut self, value: u8) -> Result<(), PortError> {
        self.port.validate_expander()?;

        unsafe {
            vexDeviceAdiValueSet(
                self.port.device_handle(),
                self.port.index(),
                i32::from(value),
            );
        }

        Ok(())
    }
}

impl AdiDevice<1> for AdiPwmOut {
    fn port_numbers(&self) -> [u8; 1] {
        [self.port.number()]
    }

    fn expander_port_number(&self) -> Option<u8> {
        self.port.expander_number()
    }

    fn device_type(&self) -> AdiDeviceType {
        AdiDeviceType::PwmOut
    }
}
