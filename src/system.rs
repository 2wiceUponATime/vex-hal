// SPDX-FileCopyrightText: 2023 Gavin Niederman
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

//! VEXos-related functionality.
//!
//! This module provides utilities for for interacting and retrieving information from VEXos.

use core::fmt;

use crate::platform::vexSystemVersion;

/// A VEXos firmware version.
///
/// This type represents a version identifier for VEXos firmware. VEXos is versioned using a
/// slightly modified [semantic versioning] scheme. To check the version currently running on a
/// brain, use [`system_version`].
///
/// [semantic versioning]: https://semver.org/
///
/// This type implements `PartialOrd`, meaning it can be compared to other instances of itself.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Version {
    /// The major version
    pub major: u8,
    /// The minor version
    pub minor: u8,
    /// The build version
    pub build: u8,
    /// The beta version
    pub beta: u8,
}

impl Version {
    #[must_use]
    pub const fn new(major: u8, minor: u8, build: u8) -> Self {
        Self {
            major,
            minor,
            build,
            beta: 0,
        }
    }

    #[must_use]
    pub const fn with_beta(mut self, beta: u8) -> Self {
        self.beta = beta;
        self
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}-b{}",
            self.major, self.minor, self.build, self.beta
        )
    }
}

/// Returns the currently running VEXos [firmware version] on a brain.
///
/// [firmware version]: Version
#[must_use]
pub fn system_version() -> Version {
    let version_bytes = unsafe { vexSystemVersion() }.to_be_bytes();

    Version {
        major: version_bytes[0],
        minor: version_bytes[1],
        build: version_bytes[2],
        beta: version_bytes[3],
    }
}
