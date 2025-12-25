use crate::form::types::PhaseFactor;
use num_complex::Complex64;
use std::ops::{Mul, MulAssign};

/// Represents a scalar value in the form `phase * 2^(-r/2)` or zero.
/// This is used for representing amplitudes and inner products exactly, avoiding floating-point errors.#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Scalar {
    Zero,
    NonZero { phase: PhaseFactor, r: usize },
}

impl Scalar {
    /// Converts the scalar to its `Complex64` representation.
    pub(crate) fn to_complex(self) -> Complex64 {
        match self {
            Scalar::Zero => Complex64::new(0.0, 0.0),
            Scalar::NonZero { phase, r } => {
                let norm = 2.0_f64.powf(-(r as f64) / 2.0);
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
