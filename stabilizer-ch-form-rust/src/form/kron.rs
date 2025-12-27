use crate::{StabilizerCHForm, error::Result};
use ndarray::{Axis, s};

impl StabilizerCHForm {
    /// Computes the tensor product of this state with another: |self> ⊗ |other>.
    ///
    /// ## Arguments
    /// * `other` - The other StabilizerCHForm to tensor with.
    ///
    /// ## Returns
    /// A [`Result`] containing the new `StabilizerCHForm` representing the tensor product state.
    pub fn kron(&self, other: &StabilizerCHForm) -> Result<StabilizerCHForm> {
        let n_total = self.n + other.n;
        let mut new_state = StabilizerCHForm::new(n_total)?;

        // Create block-diagonal matrices for G, F, and M
        new_state
            .mat_g
            .slice_mut(s![..self.n, ..self.n])
            .assign(&self.mat_g);
        new_state
            .mat_g
            .slice_mut(s![self.n.., self.n..])
            .assign(&other.mat_g);

        new_state
            .mat_f
            .slice_mut(s![..self.n, ..self.n])
            .assign(&self.mat_f);
        new_state
            .mat_f
            .slice_mut(s![self.n.., self.n..])
            .assign(&other.mat_f);

        new_state
            .mat_m
            .slice_mut(s![..self.n, ..self.n])
            .assign(&self.mat_m);
        new_state
            .mat_m
            .slice_mut(s![self.n.., self.n..])
            .assign(&other.mat_m);

        // Concatenate vectors by creating a slice of views directly.
        new_state.gamma = ndarray::concatenate(Axis(0), &[self.gamma.view(), other.gamma.view()])
            .expect("Failed to concatenate gamma vectors");

        new_state.vec_v = ndarray::concatenate(Axis(0), &[self.vec_v.view(), other.vec_v.view()])
            .expect("Failed to concatenate v vectors");

        new_state.vec_s = ndarray::concatenate(Axis(0), &[self.vec_s.view(), other.vec_s.view()])
            .expect("Failed to concatenate s vectors");

        // Combine global phases and phase factors
        new_state.set_global_phase(self.global_phase() * other.global_phase());
        new_state.phase_factor = self.phase_factor * other.phase_factor;

        Ok(new_state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::circuit::CliffordCircuit;
    use crate::test_utils::{assert_eq_complex_array1, tensor_statevectors};

    #[test]
    fn test_kron() {
        let num_qubits_1 = 3;
        let num_qubits_2 = 3;
        let trials = 10;
        for i in 0..trials {
            let circuit_1 = CliffordCircuit::random_clifford(num_qubits_1, Some([i + 56; 32]));
            let circuit_2 = CliffordCircuit::random_clifford(num_qubits_2, Some([i + 78; 32]));
            let state_1 = StabilizerCHForm::from_clifford_circuit(&circuit_1).unwrap();
            let state_2 = StabilizerCHForm::from_clifford_circuit(&circuit_2).unwrap();
            let kron_state = state_1.kron(&state_2).unwrap();
            let expected_statevector = tensor_statevectors(
                &state_1.to_statevector().unwrap(),
                &state_2.to_statevector().unwrap(),
            );
            let kron_statevector = kron_state.to_statevector().unwrap();
            assert_eq_complex_array1(&kron_statevector, &expected_statevector);
        }
    }
}
