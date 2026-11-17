// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

#[derive(Debug, PartialEq, Eq)]
pub struct SmartPort {
    number: u8,
}

impl SmartPort {
    /// # Safety
    /// Each `number` can only be passed once to [`SmartPort::new`] because having multiple
    /// instances with the same `number` can result in multiple mutable borrows of the same memory.
    #[must_use]
    pub unsafe fn new(number: u8) -> Self {
        Self { number }
    }

    /// Returns a 1-based port number that corresponds with the port label on the brain.
    #[must_use]
    pub fn number(&self) -> u8 {
        self.number
    }

    /// Returns a 0-based port index that can be passed to raw jump table calls.
    #[must_use]
    pub fn index(&self) -> u8 {
        self.number - 1
    }
}

pub struct Peripherals {
    pub port_1: SmartPort,
    pub port_2: SmartPort,
    pub port_3: SmartPort,
    pub port_4: SmartPort,
    pub port_5: SmartPort,
    pub port_6: SmartPort,
    pub port_7: SmartPort,
    pub port_8: SmartPort,
    pub port_9: SmartPort,
    pub port_10: SmartPort,
    #[cfg(any(feature = "platform-iq2", feature = "platform-v5"))]
    pub port_11: SmartPort,
    #[cfg(any(feature = "platform-iq2", feature = "platform-v5"))]
    pub port_12: SmartPort,
    #[cfg(feature = "platform-v5")]
    pub port_13: SmartPort,
    #[cfg(feature = "platform-v5")]
    pub port_14: SmartPort,
    #[cfg(feature = "platform-v5")]
    pub port_15: SmartPort,
    #[cfg(feature = "platform-v5")]
    pub port_16: SmartPort,
    #[cfg(feature = "platform-v5")]
    pub port_17: SmartPort,
    #[cfg(feature = "platform-v5")]
    pub port_18: SmartPort,
    #[cfg(feature = "platform-v5")]
    pub port_19: SmartPort,
    #[cfg(feature = "platform-v5")]
    pub port_20: SmartPort,
    #[cfg(feature = "platform-v5")]
    pub port_21: SmartPort,
}

impl Peripherals {
    /// # Safety
    /// Must be called at most once for the lifetime of the program. Each call
    /// constructs a `SmartPort` for every port number, and `SmartPort::new`
    /// requires each number be used only once (a duplicate would allow aliased
    /// mutable access to the same device). Calling this twice violates that.
    pub unsafe fn new() -> Self {
        unsafe {
            Self {
                port_1: SmartPort::new(1),
                port_2: SmartPort::new(2),
                port_3: SmartPort::new(3),
                port_4: SmartPort::new(4),
                port_5: SmartPort::new(5),
                port_6: SmartPort::new(6),
                port_7: SmartPort::new(7),
                port_8: SmartPort::new(8),
                port_9: SmartPort::new(9),
                port_10: SmartPort::new(10),
                #[cfg(any(feature = "platform-iq2", feature = "platform-v5"))]
                port_11: SmartPort::new(11),
                #[cfg(any(feature = "platform-iq2", feature = "platform-v5"))]
                port_12: SmartPort::new(12),
                #[cfg(feature = "platform-v5")]
                port_13: SmartPort::new(13),
                #[cfg(feature = "platform-v5")]
                port_14: SmartPort::new(14),
                #[cfg(feature = "platform-v5")]
                port_15: SmartPort::new(15),
                #[cfg(feature = "platform-v5")]
                port_16: SmartPort::new(16),
                #[cfg(feature = "platform-v5")]
                port_17: SmartPort::new(17),
                #[cfg(feature = "platform-v5")]
                port_18: SmartPort::new(18),
                #[cfg(feature = "platform-v5")]
                port_19: SmartPort::new(19),
                #[cfg(feature = "platform-v5")]
                port_20: SmartPort::new(20),
                #[cfg(feature = "platform-v5")]
                port_21: SmartPort::new(21),
            }
        }
    }
}
