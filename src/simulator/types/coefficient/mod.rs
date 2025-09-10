use num_complex::Complex64;
use std::fmt::Debug;
use std::ops::Mul;
use num_traits::One;

pub trait Conj {
    fn conj(&self) -> Self;
}

impl Conj for Complex64 {
    fn conj(&self) -> Self {
        self.conj()
    }
}

pub trait InnerProduct: Conj + Mul<Self, Output = Self> + Sized + Copy {
    fn inner_product(self, rhs: Self) -> Self {
        self.conj() * rhs
    }
}

impl<T> InnerProduct for T where T: Conj + Mul<Self, Output = Self> + Copy {}

pub trait Amplify: Copy {
    fn amplify(&self, factor: usize) -> Self;
}

impl Amplify for Complex64 {
    /// Amplifies the complex number by multiplying it with 2^(factor/2).
    fn amplify(&self, factor: usize) -> Self {
        let scale = 2f64.powf(factor as f64 / 2.0);
        self * scale
    }
}

pub trait Coefficient: InnerProduct + Into<Complex64> + One + Amplify + Debug {}

impl<T> Coefficient for T where T: InnerProduct + Into<Complex64> + One + Amplify + Debug {}
