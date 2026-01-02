use crate::error::Result;
use crate::state::{Coefficient, StabilizerDecomposedState};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    /// Returns a new [`StabilizerDecomposedState`] representing the tensor product of `self` and
    /// `other`.
    pub(crate) fn kron(&self, other: &Self) -> Result<Self> {
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

#[cfg(test)]
mod tests {
    use crate::test_utils::{assert_eq_complex_array1, create_sample_stab_decomp_state};

    #[test]
    fn test_kron() {
        let state1 = create_sample_stab_decomp_state(); // 3 qubits
        let state2 = create_sample_stab_decomp_state(); // 3 qubits
        let kron_state = state1.kron(&state2).unwrap();
        assert_eq!(kron_state.num_qubits, 6);

        let sv = kron_state.to_statevector().unwrap();
        let expected_sv = {
            let sv1 = state1.to_statevector().unwrap();
            let sv2 = state2.to_statevector().unwrap();
            crate::test_utils::tensor_statevectors(&sv1, &sv2)
        };
        assert_eq_complex_array1(&sv, &expected_sv);
    }
}
