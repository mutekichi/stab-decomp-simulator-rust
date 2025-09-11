use num_complex::Complex64;

use crate::{
    error::Error,
    state::{Coefficient, StabilizerDecomposedState},
};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    pub(crate) fn _exp_value(&self, pauli_string: &str) -> Result<Complex64, Error> {
        dbg!(pauli_string);
        Ok(Complex64::new(0.0, 0.0)) // Placeholder
    }
}
