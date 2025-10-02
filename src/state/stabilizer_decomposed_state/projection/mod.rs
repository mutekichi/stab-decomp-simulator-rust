use crate::{
    state::{Coefficient, StabilizerDecomposedState},
    types::error::Error,
};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    pub(crate) fn _project_normalized(&mut self, qubit: usize, outcome: bool) -> Result<(), Error> {
        self._project_unnormalized(qubit, outcome)?;
        let norm = self._norm();
        if norm == 0.0 {
            return Err(Error::Projection(
                "Projection resulted in zero norm state".to_string(),
            ));
        }
        self.global_factor /= norm;
        Ok(())
    }

    // NOTE: This function always successes even if the projection is impossible for the state.
    //       When the projection is impossible, the norm of the state becomes zero.
    pub(crate) fn _project_unnormalized(
        &mut self,
        qubit: usize,
        outcome: bool,
    ) -> Result<(), Error> {
        // Filter out stabilizers that cannot be projected to the desired outcome
        // NOTE: We can optimize this by avoiding the allocation of a new vector
        //       and instead using `retain` if performance becomes an issue.

        let (stabs, coeffs): (Vec<_>, Vec<_>) = self
            .stabilizers
            .drain(..)
            .zip(self.coefficients.drain(..))
            .filter_map(|(mut stab, coeff)| {
                match stab.project(qubit, outcome) {
                    Ok(true) => Some((stab, coeff)), // deterministic projection, keep this stabilizer
                    Ok(false) => Some((stab, coeff.amplify(-1))), // non-deterministic projection, amplify coeff by 1/sqrt(2)
                    Err(_) => {
                        // Ignore the stabilizer if projection fails
                        None
                    }
                }
            })
            .unzip();

        self.stabilizers = stabs;
        self.coefficients = coeffs;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use ndarray::Array1;
    use num_complex::Complex64;

    pub fn project_qubit(
        statevector: &Array1<Complex64>,
        qubit_index: usize,
        value: u8,
        normalize: bool,
    ) -> (Array1<Complex64>, f64) {
        let num_elements = statevector.len();
        assert!(num_elements > 0 && num_elements.is_power_of_two(), "Statevector length must be a non-zero power of 2.");

        let num_qubits = num_elements.ilog2() as usize;
        assert!(qubit_index < num_qubits, "qubit_index is out of bounds.");
        assert!(value <= 1, "value must be 0 or 1.");

        let mut projected_statevector = Array1::<Complex64>::zeros(num_elements);

        for (i, &coeff) in statevector.iter().enumerate() {
            // Check the state of the target qubit using bitwise operations.
            if ((i >> qubit_index) & 1) as u8 == value {
                projected_statevector[i] = coeff;
            }
        }

        // The probability of this outcome is the squared norm of the projected vector.
        let probability: f64 = projected_statevector.iter().map(|c| c.norm_sqr()).sum();

        if normalize && probability > 1e-12 { // Avoid division by zero for very small probabilities.
            let norm = probability.sqrt();
            projected_statevector.mapv_inplace(|c| c / norm);
        }

        (projected_statevector, probability)
    }

    use crate::{state::{QuantumState, StabilizerDecomposedState}, test_utils::{assert_eq_complex_array1, random_circuit_with_t_gate}};

    #[test]
    fn test_project_unnormalized() {
        // random_state
        let random_circuit = random_circuit_with_t_gate(5, 1000, 10, None);

        let mut state = QuantumState::from_circuit(&random_circuit).unwrap();
        let statevector = state.to_statevector();
        let statevector_ref = project_qubit(&statevector, 3, 1, false).0;

        state.project_unnormalized(3, true).unwrap();
        let statevector_test = state.to_statevector();

        let norm_ref = statevector_ref.iter().map(|c| c.norm_sqr()).sum::<f64>().sqrt();
        dbg!(norm_ref);
        dbg!(state.norm());
        
        assert_eq_complex_array1(&statevector_ref, &statevector_test);
    }

    #[test]
    fn test_project_normalized() {
        // random_state
        let random_circuit = random_circuit_with_t_gate(5, 1000, 10, None); 
        let mut state = QuantumState::from_circuit(&random_circuit).unwrap();
        let statevector = state.to_statevector();
        let statevector_ref = project_qubit(&statevector, 3, 1, true).0;

        state.project_normalized(3, true).unwrap();
        let statevector_test = state.to_statevector();
        assert_eq_complex_array1(&statevector_ref, &statevector_test);
        assert!((state.norm() - 1.0).abs() < 1e-10);
    }
}