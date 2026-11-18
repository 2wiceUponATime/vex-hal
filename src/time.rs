// SPDX-FileCopyrightText: The Rust Project Developers
// SPDX-FileCopyrightText: 2025 vexide
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

// Some of the doc comments in this file are derived from Rust's standard library

use core::{
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration,
};

use crate::platform::{vexSystemPowerupTimeGet, vexSystemTimeGet};

const fn duration_to_micros(d: Duration) -> Option<u64> {
    let micros = d.as_micros();
    if micros > u64::MAX as u128 {
        None
    } else {
        Some(micros as u64)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant(u64);

impl Instant {
    /// An instant that corresponds to the time that the VEX brain was powered on.
    pub const POWERUP: Self = Self(0);

    /// Returns an instant corresponding to "now".
    pub fn now() -> Self {
        Self(unsafe { vexSystemPowerupTimeGet() })
    }

    /// Returns the amount of time elapsed from another instant to this one, or zero duration if
    /// that instant is later than this one.
    pub const fn duration_since(&self, earlier: Instant) -> Duration {
        Duration::from_micros(self.0.saturating_sub(earlier.0))
    }

    /// Returns the amount of time elapsed from another instant to this one, or [`None`] if that
    /// instant is later than this one.
    pub const fn checked_duration_since(&self, earlier: Instant) -> Option<Duration> {
        // `Option::map` and `?` are not yet available in const contexts
        match self.0.checked_sub(earlier.0) {
            Some(micros) => Some(Duration::from_micros(micros)),
            None => None,
        }
    }

    /// Returns the amount of time elapsed since this instant, or [`Duration::ZERO`] if `self` is
    /// later than the current time.
    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }

    /// Returns `Some(t)` where `t` is the time `self + duration` if `t` fits within the bounds of
    /// [`Instant`], [`None`] otherwise.
    pub const fn checked_add(&self, duration: Duration) -> Option<Self> {
        let Some(micros) = duration_to_micros(duration) else {
            return None;
        };
        match self.0.checked_add(micros) {
            Some(t) => Some(Self(t)),
            None => None,
        }
    }

    /// Returns `Some(t)` where `t` is the time `self - duration` if `t` fits within the bounds of
    /// [`Instant`], [`None`] otherwise.
    pub const fn checked_sub(&self, duration: Duration) -> Option<Self> {
        let Some(micros) = duration_to_micros(duration) else {
            return None;
        };
        match self.0.checked_sub(micros) {
            Some(t) => Some(Self(t)),
            None => None,
        }
    }
}

impl Add<Duration> for Instant {
    type Output = Self;

    /// # Panics
    ///
    /// Panics if the resulting point in time cannot be represented by [`Instant`]. See
    /// [`Instant::checked_add`] for a version without panic.
    fn add(self, rhs: Duration) -> Self {
        self.checked_add(rhs)
            .expect("overflow when adding duration to instant")
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl Sub<Duration> for Instant {
    type Output = Self;

    /// # Panics
    ///
    /// Panics if the resulting point in time cannot be represented by [`Instant`]. See
    /// [`Instant::checked_sub`] for a version without panic.
    fn sub(self, rhs: Duration) -> Self {
        self.checked_sub(rhs)
            .expect("overflow when subtracting duration from instant")
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Instant) -> Duration {
        self.duration_since(rhs)
    }
}

/// A timestamp recorded by the Brain's low-resolution private timer.
///
/// This type is not in sync with [`Instant`], which instead uses the brain's global high-resolution
/// timer.
///
/// [`Instant`]: https://doc.rust-lang.org/stable/std/time/struct.Instant.html
///
/// # Precision
///
/// This type has a precision of 1 millisecond.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LowResolutionTime {
    millis: u32,
}

impl LowResolutionTime {
    /// An anchor in time which represents the start of the clock.
    ///
    /// In practice, the epoch represents the start of the Brain's user processor, meaning the start
    /// of the current user program.
    pub const EPOCH: LowResolutionTime = LowResolutionTime { millis: 0 };

    /// Returns a low-resolution timestamp corresponding to "now".
    #[must_use]
    pub fn now() -> Self {
        Self {
            millis: unsafe { vexSystemTimeGet() },
        }
    }

    /// Creates a new timestamp at the provided number of milliseconds since
    /// [`LowResolutionTime::EPOCH`].
    ///
    /// # Use this sparingly.
    ///
    /// This method generally only exists for compatibility with FFI and system APIs. The only clock
    /// measurement that should be provided to `millis` should be measurements derived from the CPU1
    /// private timer (e.g. [`vexSystemTimeGet`]) to ensure that clock drift is not a factor.
    ///
    /// When possible, prefer using [`LowResolutionTime::now`].
    #[must_use]
    pub const fn from_millis_since_epoch(millis: u32) -> Self {
        Self { millis }
    }

    /// Returns the amount of time elapsed from another timestamp to this one, or zero duration if
    /// that timestamp is later than this one.
    #[must_use]
    pub fn duration_since(&self, earlier: LowResolutionTime) -> Duration {
        self.checked_duration_since(earlier).unwrap_or_default()
    }

    /// Returns the amount of time elapsed from another timestamp to this one, or None if that
    /// timestamp is later than this one.
    #[must_use]
    pub const fn checked_duration_since(&self, earlier: LowResolutionTime) -> Option<Duration> {
        if earlier.millis < self.millis {
            Some(Duration::from_millis((self.millis - earlier.millis) as _))
        } else {
            None
        }
    }

    /// Returns the amount of time elapsed from another timestamp to this one, or zero duration if
    /// that timestamp is later than this one.
    #[must_use]
    pub fn saturating_duration_since(&self, earlier: LowResolutionTime) -> Duration {
        self.checked_duration_since(earlier).unwrap_or_default()
    }

    /// Returns the amount of time elapsed since this timestamp.
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        Self::now() - *self
    }

    /// Returns `Some(t)` where `t` is the time `self + duration` if `t` can be represented as
    /// `LowResolutionTime` (which means it's inside the bounds of the underlying data structure),
    /// `None` otherwise.
    #[must_use]
    pub fn checked_add(self, rhs: Duration) -> Option<LowResolutionTime> {
        Some(Self {
            millis: self.millis.checked_add(rhs.as_millis().try_into().ok()?)?,
        })
    }

    /// Returns `Some(t)` where `t` is the time `self - duration` if `t` can be represented as
    /// `LowResolutionTime` (which means it's inside the bounds of the underlying data structure),
    /// `None` otherwise.
    #[must_use]
    pub fn checked_sub(self, rhs: Duration) -> Option<LowResolutionTime> {
        Some(Self {
            millis: self.millis.checked_sub(rhs.as_millis().try_into().ok()?)?,
        })
    }
}

impl Add<Duration> for LowResolutionTime {
    type Output = LowResolutionTime;

    /// # Panics
    ///
    /// This function may panic if the resulting point in time cannot be represented by the
    /// underlying data structure. See [`LowResolutionTime::checked_add`] for a version without
    /// panic.
    fn add(self, rhs: Duration) -> Self::Output {
        self.checked_add(rhs)
            .expect("overflow when adding duration to timestamp")
    }
}

impl AddAssign<Duration> for LowResolutionTime {
    fn add_assign(&mut self, other: Duration) {
        *self = *self + other;
    }
}

impl Sub<Duration> for LowResolutionTime {
    type Output = LowResolutionTime;

    fn sub(self, other: Duration) -> LowResolutionTime {
        self.checked_sub(other)
            .expect("overflow when subtracting duration from timestamp")
    }
}

impl SubAssign<Duration> for LowResolutionTime {
    fn sub_assign(&mut self, other: Duration) {
        *self = *self - other;
    }
}

impl Sub<LowResolutionTime> for LowResolutionTime {
    type Output = Duration;

    /// Returns the amount of time elapsed from another time to this one, or zero duration if that
    /// time is later than this one.
    fn sub(self, other: LowResolutionTime) -> Duration {
        self.duration_since(other)
    }
}
