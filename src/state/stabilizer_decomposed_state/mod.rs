pub mod comp_basis;
pub mod evolution;
pub mod exp_value;
pub mod inner_product;
pub mod kron;
pub mod measurement;
pub mod projection;
pub mod sampling;

use num_complex::Complex64;
use stabilizer_ch_form_rust::prelude::*;

use crate::state::Coefficient;

#[derive(Clone, Debug)]
pub(crate) struct StabilizerDecomposedState<T: Coefficient> {
    pub num_qubits: usize,
    pub stabilizers: Vec<StabilizerCHForm>,
    pub coefficients: Vec<T>,
    pub global_factor: Complex64, // stands for the global phase and normalization factor
}

impl<T: Coefficient> StabilizerDecomposedState<T> {
    /// Creates a new StabilizerDecomposedState representing the all-zero state |0...0>.
    pub fn new(
        num_qubits: usize,
        stabilizers: Vec<StabilizerCHForm>,
        coefficients: Vec<T>,
    ) -> Self {
        // We do not check if the input stabilizers and coefficients are valid here for performance reasons.
        StabilizerDecomposedState {
            num_qubits,
            stabilizers,
            coefficients,
            global_factor: Complex64::new(1.0, 0.0),
        }
    }
}
