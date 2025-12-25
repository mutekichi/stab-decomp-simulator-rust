use crate::{
    error::Error,
    error::Result,
    state::{Coefficient, StabilizerDecomposedState},
};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    pub(crate) fn project_normalized(&mut self, qubit: usize, outcome: bool) -> Result<()> {
        self.project_unnormalized(qubit, outcome)?;
        let norm = self.norm()?;
        if norm.abs() < 1e-12 || norm.is_nan() {
            return Err(Error::ImpossibleProjection {
                qubit_index: qubit,
                desired: outcome,
            });
        }
        self.global_factor /= norm;
        Ok(())
    }

    // NOTE: This function always successes even if the projection is impossible for the state.
    //       When the projection is impossible, the norm of the state becomes zero.
    pub(crate) fn project_unnormalized(&mut self, qubit: usize, outcome: bool) -> Result<()> {
        // Filter out stabilizers that cannot be projected to the desired outcome
        // NOTE: We can optimize this by avoiding the allocation of a new vector
        //       and instead using `retain` if performance becomes an issue.

        let (stabs, coeffs): (Vec<_>, Vec<_>) = self
            .stabilizers
            .drain(..)
            .zip(self.coefficients.drain(..))
            .filter_map(|(mut stab, coeff)| {
                match stab.project(qubit, outcome) {
                    // deterministic projection, keep this stabilizer
                    Ok(true) => Some((stab, coeff)),
                    // non-deterministic projection, amplify coeff by 1/sqrt(2)
                    Ok(false) => Some((stab, coeff.amplify(-1))),
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
    use crate::circuit::QuantumCircuit;
    use crate::error::Error;
    use ndarray::Array1;
    use num_complex::Complex64;

    /// Projects the given statevector (as an Array1<Complex64>) onto the subspace where the qubit
    /// at `qubit_index` is in the state `|value>` (0 or 1) and returns the projected statevector
    /// for reference.
    /// If `normalize` is true, the resulting statevector is normalized.
    pub fn project_statevector(
        statevector: &Array1<Complex64>,
        qubit_index: usize,
        value: u8,
        normalize: bool,
    ) -> (Array1<Complex64>, f64) {
        let num_elements = statevector.len();
        assert!(
            num_elements > 0 && num_elements.is_power_of_two(),
            "Statevector length must be a non-zero power of 2."
        );

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

        if normalize && probability > 1e-12 {
            // Avoid division by zero for very small probabilities.
            let norm = probability.sqrt();
            projected_statevector.mapv_inplace(|c| c / norm);
        }

        (projected_statevector, probability)
    }

    use crate::{
        state::QuantumState,
        test_utils::{assert_eq_complex_array1, random_circuit_with_t_gate},
    };

    #[test]
    fn test_project_unnormalized_random() {
        let trials = 50;
        let n_qubits = 5;
        let clifford_count = 100;
        let t_count = 10;

        for i in 0..trials {
            let seed = i as u64;
            let target_qubit = i % n_qubits;
            let outcome = (i % 2) == 1;

            let random_circuit =
                random_circuit_with_t_gate(n_qubits, clifford_count, t_count, Some(seed));

            let mut state = QuantumState::from_circuit(&random_circuit).unwrap();
            let statevector = state.to_statevector().unwrap();

            let (statevector_ref, _) =
                project_statevector(&statevector, target_qubit, outcome as u8, false);

            state.project_unnormalized(target_qubit, outcome).unwrap();
            let statevector_test = state.to_statevector().unwrap();

            assert_eq_complex_array1(&statevector_ref, &statevector_test);
        }
    }

    #[test]
    fn test_project_normalized_random() {
        let trials = 50;
        let n_qubits = 5;
        let clifford_count = 100;
        let t_count = 10;

        for i in 0..trials {
            let seed = (i + 123) as u64;
            let target_qubit = (i + 1) % n_qubits;
            let outcome = (i % 2) == 0;

            let random_circuit =
                random_circuit_with_t_gate(n_qubits, clifford_count, t_count, Some(seed));

            let mut state = QuantumState::from_circuit(&random_circuit).unwrap();
            let statevector = state.to_statevector().unwrap();

            let (statevector_ref, prob) =
                project_statevector(&statevector, target_qubit, outcome as u8, true);

            if prob.abs() < 1e-12 {
                // Projection is impossible, expect an error
                let result = state.project_normalized(target_qubit, outcome);
                match result {
                    Err(Error::ImpossibleProjection {
                        qubit_index,
                        desired,
                    }) => {
                        assert_eq!(qubit_index, target_qubit);
                        assert_eq!(desired, outcome);
                    }
                    _ => panic!("Expected ImpossibleProjection error."),
                }
            } else {
                state.project_normalized(target_qubit, outcome).unwrap();
                let statevector_test = state.to_statevector().unwrap();

                assert_eq_complex_array1(&statevector_ref, &statevector_test);
            }
        }
    }

    #[test]
    fn test_inner_product_between_projected_states() {
        let n_trials = 20;
        let n_qubits = 4;
        let clifford_count = 200;
        let t_count = 5;

        let mut success_count = 0;
        let mut attempt = 0;

        while success_count < n_trials {
            let seed_1 = (attempt + 200) as u64;
            let seed_2 = (attempt + 300) as u64;
            let target_qubit_1 = (attempt + 2) % n_qubits;
            let target_qubit_2 = (2 * attempt + n_qubits - 1) % n_qubits;
            let outcome_1 = (attempt % 2) == 0;
            let outcome_2 = (attempt % 3) == 1;

            // Increment attempt to ensure different seeds in the next iteration
            attempt += 1;

            let random_circuit_1 =
                random_circuit_with_t_gate(n_qubits, clifford_count, t_count, Some(seed_1));
            let random_circuit_2 =
                random_circuit_with_t_gate(n_qubits, clifford_count, t_count, Some(seed_2));

            let mut state_1 = QuantumState::from_circuit(&random_circuit_1).unwrap();
            let mut state_2 = QuantumState::from_circuit(&random_circuit_2).unwrap();

            if let Err(Error::ImpossibleProjection { .. }) =
                state_1.project_normalized(target_qubit_1, outcome_1)
            {
                continue;
            }
            if let Err(Error::ImpossibleProjection { .. }) =
                state_2.project_normalized(target_qubit_2, outcome_2)
            {
                continue;
            }

            let inner_product = state_1.inner_product(&state_2).unwrap();

            let statevector_1 = state_1.to_statevector().unwrap();
            let statevector_2 = state_2.to_statevector().unwrap();
            let inner_product_ref: Complex64 = statevector_1
                .iter()
                .zip(statevector_2.iter())
                .map(|(&a, &b)| a.conj() * b)
                .sum();

            let tolerance = 1e-10;
            assert!(
                (inner_product - inner_product_ref).norm() < tolerance,
                "Inner product mismatch: computed = {}, reference = {}",
                inner_product,
                inner_product_ref
            );

            success_count += 1;
        }
    }
    #[test]
    fn test_project_deterministic() {
        let mut circuit = QuantumCircuit::new(2);
        circuit.apply_h(0);
        circuit.apply_s(0);
        circuit.apply_cz(0, 1);
        let mut state = QuantumState::from_circuit(&circuit).unwrap();
        let statevector_before = state.to_statevector().unwrap();
        // This projection is expected to be deterministic
        state.project_unnormalized(1, false).unwrap();
        let statevector_after = state.to_statevector().unwrap();
        assert_eq_complex_array1(&statevector_before, &statevector_after);
    }

    #[test]
    fn test_project_normalized_impossible() {
        let mut circuit = QuantumCircuit::new(2);
        circuit.apply_h(0);
        circuit.apply_cz(0, 1);
        let mut state = QuantumState::from_circuit(&circuit).unwrap();
        // This projection is expected to be impossible
        // The normalized projection should return an error.
        let result = state.project_normalized(1, true);

        match result {
            Err(Error::ImpossibleProjection {
                qubit_index,
                desired,
            }) => {
                assert_eq!(qubit_index, 1);
                assert_eq!(desired, true);
            }
            _ => panic!("Expected ImpossibleProjection error."),
        }
    }

    #[test]
    fn test_project_unnormalize_impossible() {
        let mut circuit = QuantumCircuit::new(2);
        circuit.apply_h(0);
        circuit.apply_cz(0, 1);
        let mut state = QuantumState::from_circuit(&circuit).unwrap();
        // This projection is expected to be impossible
        // The unnormalized projection always return Ok, but the norm becomes zero if the
        // projection is actually impossible.
        let result = state.project_unnormalized(1, true);
        assert!(result.is_ok());
        let norm = state.norm().unwrap();
        assert_eq!(norm, 0.0);
    }
}
