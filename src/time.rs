// SPDX-FileCopyrightText: The Rust Project Developers
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

use core::{
    ops::{Add, AddAssign, Sub, SubAssign},
    time::Duration,
};

use crate::platform::vexSystemPowerupTimeGet;

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
