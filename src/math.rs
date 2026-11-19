// SPDX-FileCopyrightText: 2023 Gavin Niederman
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

use core::f64::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_8, PI, TAU};

use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use easy_ext::ext;
use num_traits::{AsPrimitive, Euclid};

pub use libm::{floorf, roundf, truncf};
pub use mint::{EulerAngles, Point2, Quaternion, Vector3};

/// An unbounded angular position.
///
/// This type stores a unit-agnostic angle (a signed displacement from some rotation representing
/// `Angle::ZERO`).
///
/// # Non-modularity
///
/// This type is unbounded and is NOT modular 2π. This means that `Angle::from_degrees(0) !=
/// Angle::from_degrees(360)`, for instance.
///
/// # Precision
///
/// This type internally stores angles as *radians* inside of an `f64`.
#[derive(
    Default,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Add,
    AddAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Sub,
    SubAssign,
    Neg,
)]
pub struct Angle {
    radians: f64,
}

impl Angle {
    /// Angle representing zero rotation.
    pub const ZERO: Self = Self { radians: 0.0 };

    /// Angle representing a sixteenth turn around a full circle.
    pub const SIXTEENTH_TURN: Self = Self { radians: FRAC_PI_8 };

    /// Angle representing an eighth turn around a full circle.
    pub const EIGHTH_TURN: Self = Self { radians: FRAC_PI_4 };

    /// Angle representing a quarter turn around a full circle.
    pub const QUARTER_TURN: Self = Self { radians: FRAC_PI_2 };

    /// Angle representing a half turn around a full circle.
    pub const HALF_TURN: Self = Self { radians: PI };

    /// Angle representing a full turn around a circle.
    pub const FULL_TURN: Self = Self { radians: TAU };

    // MARK: Angle Conversion

    /// Creates an angle from a specified number of degrees.
    #[inline]
    #[must_use]
    pub const fn from_degrees(degrees: f64) -> Self {
        Self {
            radians: degrees.to_radians(),
        }
    }

    /// Creates an angle from a specified number of radians.
    #[inline]
    #[must_use]
    pub const fn from_radians(radians: f64) -> Self {
        Self { radians }
    }

    /// Creates an angle from a specified number of gradians.
    #[must_use]
    pub const fn from_gradians(gradians: f64) -> Self {
        Self {
            radians: gradians * (PI / 200.0),
        }
    }

    /// Creates an angle from a specified number of turns (revolutions).
    #[inline]
    #[must_use]
    pub const fn from_turns(turns: f64) -> Self {
        Self {
            radians: turns * TAU,
        }
    }

    /// Creates an angle from a custom tick reading with a given ticks-per-revolution value.
    #[inline]
    #[must_use]
    pub(crate) const fn from_ticks(ticks: f64, ticks_per_revolution: u32) -> Self {
        Self::from_turns(ticks / ticks_per_revolution as f64)
    }

    // MARK: Angle Conversion

    /// Returns the number of degrees rotated in this angle.
    #[inline]
    #[must_use]
    pub const fn as_degrees(&self) -> f64 {
        self.radians.to_degrees()
    }

    /// Returns the number of radians rotated in this angle.
    #[inline]
    #[must_use]
    pub const fn as_radians(&self) -> f64 {
        self.radians
    }

    /// Returns the number of gradians rotated in this angle.
    #[inline]
    #[must_use]
    pub const fn as_gradians(&self) -> f64 {
        self.radians * (200.0 / PI)
    }

    /// Returns the number of turns (revolutions) rotated in this angle.
    #[inline]
    #[must_use]
    pub const fn as_turns(&self) -> f64 {
        self.radians / TAU
    }

    /// Returns this angle's value scaled to a raw tick value with the provided TPR.
    #[inline]
    #[must_use]
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub(crate) const fn as_ticks(&self, ticks_per_revolution: u32) -> f64 {
        self.radians / TAU * (ticks_per_revolution as f64)
    }
}

// MARK: Angle Math
impl Angle {
    /// Normalizes the angle to the bounds [0, 2pi).
    #[inline]
    #[must_use]
    pub fn wrapped_full(&self) -> Self {
        Self {
            radians: Euclid::rem_euclid(&self.radians, &TAU),
        }
    }

    /// Normalizes the angle to the bounds [-pi, pi).
    #[inline]
    #[must_use]
    pub fn wrapped_half(&self) -> Self {
        Self {
            radians: Euclid::rem_euclid(&(self.radians + PI), &TAU) - PI,
        }
    }

    /// Computes the arcsine of a number. Return value is in the range [-pi/2, pi/2] or NaN if the
    /// angle is outside the range [-1, 1].
    #[inline]
    #[must_use]
    pub fn asin(y: f64) -> Self {
        Self {
            radians: libm::asin(y),
        }
    }

    /// Computes the arccosine of a number. Return value is in the range [0, pi] or NaN if the angle
    /// is outside the range [-1, 1].
    #[inline]
    #[must_use]
    pub fn acos(x: f64) -> Self {
        Self {
            radians: libm::acos(x),
        }
    }

    /// Computes the arctangent of an angle. Return value is in radians in the range [-pi/2, pi/2];
    #[inline]
    #[must_use]
    pub fn atan(tan: f64) -> Self {
        Self {
            radians: libm::atan(tan),
        }
    }

    /// Computes the four quadrant arctangent angle of `y` and `x`.
    #[inline]
    #[must_use]
    pub fn atan2(y: f64, x: f64) -> Self {
        Self {
            radians: libm::atan2(y, x),
        }
    }

    /// Computes the absolute value of `self`.
    #[allow(clippy::missing_const_for_fn)]
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn abs(self) -> Self {
        Self {
            radians: libm::fabs(self.radians),
        }
    }

    /// Returns a number that represents the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - NaN if the number is NaN
    #[allow(clippy::missing_const_for_fn)]
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn signum(self) -> f64 {
        if self.radians.is_nan() {
            f64::NAN
        } else {
            libm::copysign(1.0, self.radians)
        }
    }

    /// Returns an angle composed of the magnitude of `self` and the sign of
    /// `sign`.
    ///
    /// Equal to `self` if the sign of `self` and `sign` are the same, otherwise equal to `-self`.
    /// If `self` is a NaN, then a NaN with the same payload as `self` and the sign bit of `sign` is
    /// returned.
    ///
    /// If `sign` is a NaN, then this operation will still carry over its sign into the result. Note
    /// that IEEE 754 doesn't assign any meaning to the sign bit in case of a NaN, and as Rust
    /// doesn't guarantee that the bit pattern of NaNs are conserved over arithmetic operations, the
    /// result of `copysign` with `sign` being a NaN might produce an unexpected or non-portable
    /// result. See the [specification of NaN bit patterns](primitive@f32#nan-bit-patterns) for more
    /// info.
    #[allow(clippy::missing_const_for_fn)]
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn copysign(self, sign: Self) -> Self {
        Self {
            radians: libm::copysign(self.radians, sign.radians),
        }
    }

    /// Fused multiply-add. Computes `(self * a) + b` with only one rounding error, yielding a more
    /// accurate result than an unfused multiply-add.
    ///
    /// Using `mul_add` *may* be more performant than an unfused multiply-add if the target
    /// architecture has a dedicated `fma` CPU instruction. However, this is not always true, and
    /// will be heavily dependant on designing algorithms with specific target hardware in mind.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    #[allow(clippy::missing_const_for_fn, reason = "libm::fma branch is not const")]
    pub fn mul_add(self, a: f64, b: Self) -> Self {
        Self {
            radians: libm::fma(self.radians, a, b.radians),
        }
    }

    /// The positive difference of two numbers.
    ///
    /// * If `self <= other`: `0.0`
    /// * Else: `self - other`
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn abs_sub(self, other: Self) -> Self {
        Self {
            radians: libm::fdim(self.radians, other.radians),
        }
    }

    /// Computes the sine of an angle.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn sin(self) -> f64 {
        libm::sin(self.radians)
    }

    /// Computes the cosine of an angle.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn cos(self) -> f64 {
        libm::cos(self.radians)
    }

    /// Computes the tangent of an angle.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn tan(self) -> f64 {
        libm::tan(self.radians)
    }

    /// Simultaneously computes the sine and cosine of the number, `x`. Returns `(sin(x), cos(x))`.
    #[inline]
    #[must_use = "this returns the result of the operation, without modifying the original"]
    pub fn sin_cos(self) -> (f64, f64) {
        (self.sin(), self.cos())
    }
}

#[ext(AngleExt)]
pub impl<T: AsPrimitive<f64>> T {
    fn degrees(&self) -> Angle {
        Angle::from_degrees(self.as_())
    }

    fn radians(&self) -> Angle {
        Angle::from_radians(self.as_())
    }

    fn gradians(&self) -> Angle {
        Angle::from_gradians(self.as_())
    }

    fn turns(&self) -> Angle {
        Angle::from_turns(self.as_())
    }

    fn ticks(&self, ticks_per_revolution: u32) -> Angle {
        Angle::from_ticks(self.as_(), ticks_per_revolution)
    }
}

macro_rules! impl_debug {
    ($ty:ident => |$self:ident| $($fmt:tt)+) => {
        impl ::core::fmt::Debug for $ty {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let $self = self;
                write!(f, concat!(stringify!($ty), "({})"), format_args!($($fmt)+))
            }
        }
    };
}

impl_debug!(Angle => |s| "{} rad", s.radians);

/// A rotational direction.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    /// Rotates in the forward direction.
    Forward,

    /// Rotates in the reverse direction.
    Reverse,
}

impl Direction {
    /// Returns `true` if the direction is [`Forward`](Direction::Forward).
    #[must_use]
    pub const fn is_forward(&self) -> bool {
        match self {
            Self::Forward => true,
            Self::Reverse => false,
        }
    }

    /// Returns `true` if the direction is [`Reverse`](Direction::Reverse).
    #[must_use]
    pub const fn is_reverse(&self) -> bool {
        match self {
            Self::Forward => false,
            Self::Reverse => true,
        }
    }
}
