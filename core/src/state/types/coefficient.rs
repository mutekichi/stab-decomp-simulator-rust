use num_complex::Complex64;
use num_traits::One;
use std::fmt::Debug;
use std::ops::Mul;
/// Trait representing the complex conjugate operation.
pub(crate) trait Conj {
    fn conj(&self) -> Self;
}

impl Conj for Complex64 {
    /// Returns the complex conjugate of the Complex64 number.
    fn conj(&self) -> Self {
        self.conj()
    }
}

/// Trait for types that support inner product operations.
pub(crate) trait InnerProduct: Conj + Mul<Self, Output = Self> + Sized + Copy {}

impl<T> InnerProduct for T where T: Conj + Mul<Self, Output = Self> + Copy {}

/// Trait for types that support amplification by powers of two.
pub(crate) trait Amplify: Copy {
    /// Amplifies the value by multiplying it with 2^(factor/2).
    fn amplify(&self, factor: isize) -> Self;
}

impl Amplify for Complex64 {
    /// Amplifies the complex number by multiplying it with 2^(factor/2).
    fn amplify(&self, factor: isize) -> Self {
        let scale = 2f64.powf(factor as f64 / 2.0);
        self * scale
    }
}

/// Trait representing a coefficient in the stabilizer decomposed state.
pub(crate) trait Coefficient:
    InnerProduct + Into<Complex64> + One + Amplify + Debug
{
}

impl<T> Coefficient for T where T: InnerProduct + Into<Complex64> + One + Amplify + Debug {}
