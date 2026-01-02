use num_complex::Complex64;
use std::ops::{Mul, MulAssign};

/// Represents a phase of the form e^(i * k * pi / 4) for k in {0, 1, ..., 7}.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct PhaseFactor(u8);

impl PhaseFactor {
    pub(crate) const PLUS_ONE: Self = Self(0); // k=0
    pub(crate) const EXP_I_PI_4: Self = Self(1); // k=1
    pub(crate) const PLUS_I: Self = Self(2); // k=2
    // pub(crate) const EXP_I_3PI_4: Self = Self(3); // k=3
    pub(crate) const MINUS_ONE: Self = Self(4); // k=4
    // pub(crate) const EXP_I_5PI_4: Self = Self(5); // k=5
    pub(crate) const MINUS_I: Self = Self(6); // k=6
    pub(crate) const EXP_I_7PI_4: Self = Self(7); // k=7

    /// Converts the phase factor to a complex number.
    pub(crate) fn to_complex(self) -> Complex64 {
        let angle = (self.0 as f64) * std::f64::consts::FRAC_PI_4;
        Complex64::new(angle.cos(), angle.sin())
    }

    /// Returns the inverse of the phase factor (complex conjugate).
    pub(crate) fn conjugated(&self) -> Self {
        Self((8 - self.0) % 8)
    }

    /// Multiplies the phase by -1 (adds pi to the angle, which is k=4).
    pub(crate) fn flipped(&self) -> Self {
        Self((self.0 + 4) % 8)
    }

    /// In-place version of `flipped`.
    pub(crate) fn flip_sign(&mut self) {
        *self = self.flipped();
    }
}

impl Mul for PhaseFactor {
    type Output = Self;

    /// Phase multiplication corresponds to adding the internal `k` values modulo 8.
    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 + rhs.0) % 8)
    }
}

impl MulAssign for PhaseFactor {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
