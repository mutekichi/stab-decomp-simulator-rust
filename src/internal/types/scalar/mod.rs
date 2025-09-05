use num_complex::Complex64;
use std::ops::{Mul, MulAssign};

use crate::prelude::types::{coefficient::{Conj, InnerProduct}, phase_factor::PhaseFactor};

/// Represents a scalar value in the form `phase * 2^(-r/2)` or zero.
/// NOTE: Should be changed to pub(crate)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scalar {
    Zero,
    NonZero { phase: PhaseFactor, r: usize },
}

impl Scalar {
    pub const ZERO: Self = Scalar::Zero;
    pub const ONE: Self = Scalar::NonZero {
        phase: PhaseFactor::PLUS_ONE,
        r: 0,
    };
    pub const MINUS_ONE: Self = Scalar::NonZero {
        phase: PhaseFactor::MINUS_ONE,
        r: 0,
    };
    pub const I: Self = Scalar::NonZero {
        phase: PhaseFactor::PLUS_I,
        r: 0,
    };
    pub const MINUS_I: Self = Scalar::NonZero {
        phase: PhaseFactor::MINUS_I,
        r: 0,
    };
    pub const ONE_OVER_SQRT_2: Self = Scalar::NonZero {
        phase: PhaseFactor::PLUS_ONE,
        r: 1,
    };

    /// Converts the scalar to its `Complex64` representation.
    pub fn to_complex(&self) -> Complex64 {
        match self {
            Scalar::Zero => Complex64::new(0.0, 0.0),
            Scalar::NonZero { phase, r } => {
                let norm = 2.0_f64.powf(-(*r as f64) / 2.0);
                phase.to_complex() * norm
            }
        }
    }
}

impl Mul for Scalar {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Scalar::Zero, _) | (_, Scalar::Zero) => Scalar::Zero,
            (Scalar::NonZero { phase: p1, r: r1 }, Scalar::NonZero { phase: p2, r: r2 }) => {
                Scalar::NonZero {
                    phase: p1 * p2,
                    r: r1 + r2,
                }
            }
        }
    }
}

impl From<Scalar> for Complex64 {
    fn from(scalar: Scalar) -> Self {
        scalar.to_complex()
    }
}

impl Conj for Scalar {
    fn conj(&self) -> Self {
        match self {
            Scalar::Zero => Scalar::Zero,
            Scalar::NonZero { phase, r } => Scalar::NonZero {
                phase: phase.conjugated(),
                r: *r,
            },
        }
    }
}

impl MulAssign for Scalar {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl Mul<PhaseFactor> for Scalar {
    type Output = Self;

    fn mul(self, rhs: PhaseFactor) -> Self::Output {
        match self {
            Scalar::Zero => Scalar::Zero,
            Scalar::NonZero { phase, r } => Scalar::NonZero {
                phase: phase * rhs,
                r,
            },
        }
    }
}

impl MulAssign<PhaseFactor> for Scalar {
    fn mul_assign(&mut self, rhs: PhaseFactor) {
        *self = *self * rhs;
    }
}
