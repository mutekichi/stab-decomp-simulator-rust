
use num_complex::Complex64;
use std::ops::Mul;
use std::fmt::Debug;

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

pub trait Coefficient: InnerProduct + Into<Complex64> + Debug {}

impl<T> Coefficient for T where T: InnerProduct + Into<Complex64> + Debug {}
