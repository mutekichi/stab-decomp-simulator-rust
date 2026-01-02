use crate::error::Result;
use crate::state::{Coefficient, StabilizerDecomposedState};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    pub(crate) fn discard(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.discard(qarg)?;
        }
        self.num_qubits -= 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::circuit::QuantumCircuit;
    use crate::state::QuantumState;
    use crate::test_utils::{assert_eq_complex_array1, create_sample_stab_decomp_state};

    #[test]
    fn test_discard() {
        // 1/2 (|000> + |010> + |001> + |111>)
        let mut state = create_sample_stab_decomp_state();
        // 1/sqrt(2) (|000> + |010>)
        state.project_normalized(0, false).unwrap();
        // 1/sqrt(2) (|00> + |01>)
        state.discard(0).unwrap();

        assert_eq!(state.num_qubits, 2);

        let sv = state.to_statevector().unwrap();
        let expected_sv = {
            let mut circuit = QuantumCircuit::new(2);
            circuit.apply_h(0);
            let state = QuantumState::from_circuit(&circuit).unwrap();
            state.to_statevector().unwrap()
        };
        assert_eq_complex_array1(&sv, &expected_sv);

        // Invalid qubit index (out of bounds)
        let mut state = create_sample_stab_decomp_state();
        let result = state.discard(3);
        assert!(result.is_err());
    }
}
