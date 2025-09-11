pub mod comp_basis;
pub mod evolution;
pub mod exp_value;
pub mod inner_product;
pub mod measurement;
pub mod sampling;

use stabilizer_ch_form_rust::prelude::*;

use crate::state::Coefficient;

#[derive(Clone, Debug)]
pub(crate) struct StabilizerDecomposedState<T: Coefficient> {
    pub num_qubits: usize,
    pub stabilizers: Vec<StabilizerCHForm>,
    pub coefficients: Vec<T>,
}

impl<T: Coefficient> StabilizerDecomposedState<T> {
    /// Returns a new StabilizerDecomposedState representing the tensor product of self and other.
    pub fn kron(&self, other: &Self) -> Self {
        let mut new_stabilizers = Vec::new();
        let mut new_coefficients = Vec::new();

        for (stab1, coeff1) in self.stabilizers.iter().zip(self.coefficients.iter()) {
            for (stab2, coeff2) in other.stabilizers.iter().zip(other.coefficients.iter()) {
                new_stabilizers.push(stab1.kron(stab2));
                new_coefficients.push(*coeff1 * *coeff2);
            }
        }

        StabilizerDecomposedState {
            num_qubits: self.num_qubits + other.num_qubits,
            stabilizers: new_stabilizers,
            coefficients: new_coefficients,
        }
    }
}
