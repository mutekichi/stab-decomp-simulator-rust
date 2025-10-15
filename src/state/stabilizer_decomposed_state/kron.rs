use crate::error::Result;
use crate::state::{Coefficient, StabilizerDecomposedState};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    /// Returns a new StabilizerDecomposedState representing the tensor product of self and other.
    pub fn kron(&self, other: &Self) -> Result<Self> {
        let mut new_stabilizers = Vec::new();
        let mut new_coefficients = Vec::new();

        for (stab1, coeff1) in self.stabilizers.iter().zip(self.coefficients.iter()) {
            for (stab2, coeff2) in other.stabilizers.iter().zip(other.coefficients.iter()) {
                new_stabilizers.push(stab1.kron(stab2)?);
                new_coefficients.push(*coeff1 * *coeff2);
            }
        }
        Ok(StabilizerDecomposedState::new(
            self.num_qubits + other.num_qubits,
            new_stabilizers,
            new_coefficients,
        ))
    }
}
