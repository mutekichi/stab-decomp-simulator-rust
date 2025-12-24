use crate::error::{Error, Result};
use crate::state::{Coefficient, StabilizerDecomposedState};
use num_complex::Complex64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    pub(crate) fn measure(&mut self, qargs: &[usize], seed: Option<[u8; 32]>) -> Result<Vec<bool>> {
        self.validate_qargs(qargs)?;
        let mut rng = match seed {
            Some(s) => rand::rngs::StdRng::from_seed(s),
            None => rand::rngs::StdRng::from_entropy(),
        };
        let mut outcomes = Vec::with_capacity(qargs.len());

        for &qubit in qargs {
            let outcome = self.measure_single_qubit(qubit, &mut rng)?;
            outcomes.push(outcome);
        }

        Ok(outcomes)
    }

    pub(crate) fn measure_all(&mut self, seed: Option<[u8; 32]>) -> Result<Vec<bool>> {
        let num_qubits = self.num_qubits;
        let qargs: Vec<usize> = (0..num_qubits).collect();
        self.measure(&qargs, seed)
    }

    fn measure_single_qubit(&mut self, qubit: usize, rng: &mut StdRng) -> Result<bool> {
        let mut state_zero = self.clone();
        let mut state_one = self.clone();

        let can_project_zero = state_zero.project_unnormalized(qubit, false).is_ok();
        let can_project_one = state_one.project_unnormalized(qubit, true).is_ok();

        // Match statement now returns a tuple: (measurement outcome, squared norm of the resulting unnormalized state)
        let (measurement_outcome, norm_sq_after_proj) = match (can_project_zero, can_project_one) {
            (false, false) => {
                // This case should not happen for a valid quantum state, but we handle it defensively.
                return Err(Error::ImpossibleProjection {
                    qubit_index: qubit,
                    desired: false, // Represents impossibility for both outcomes
                });
            }
            (true, false) => {
                // Deterministic outcome: 0.
                // The norm squared of state_zero should be the original norm squared (i.e., 1.0).
                let norm_sq = state_zero.norm_squared()?;
                (false, norm_sq)
            }
            (false, true) => {
                // Deterministic outcome: 1.
                let norm_sq = state_one.norm_squared()?;
                (true, norm_sq)
            }
            (true, true) => {
                // Superposition case.
                let prob_zero = state_zero.norm_squared()?;
                let prob_one = state_one.norm_squared()?;
                let total_prob = prob_zero + prob_one;

                if total_prob.abs() < 1e-12 {
                    return Err(Error::NotImplemented(
                        "Measurement on a zero-norm state is not possible.".to_string(),
                    ));
                }

                let prob_zero_normalized = prob_zero / total_prob;

                let sample: f64 = rng.r#gen();

                if sample < prob_zero_normalized {
                    // Outcome is 0 (false)
                    (false, prob_zero)
                } else {
                    // Outcome is 1 (true)
                    (true, prob_one)
                }
            }
        };

        if measurement_outcome {
            *self = state_one;
        } else {
            *self = state_zero;
        }

        let norm = norm_sq_after_proj.sqrt();
        if norm.abs() < 1e-12 {
            return Err(Error::NotImplemented(
                "Measurement resulted in a zero-norm state, which is not physical.".to_string(),
            ));
        }

        self.amplify_global_factor(Complex64::new(1.0 / norm, 0.0));

        Ok(measurement_outcome)
    }
}

#[cfg(test)]
mod tests {
    use num_complex::Complex64;

    #[test]
    fn test_measurement() {
        // sample_state = |000> + |100> + |010> + |111>
        let sample_state = crate::test_utils::create_sample_stab_decomp_state();
        let trials = 100;
        let mut counts = std::collections::HashMap::new();
        for _ in 0..trials {
            let mut state = sample_state.clone();
            let outcome = state.measure_all(None).unwrap();
            *counts.entry(outcome).or_insert(0) += 1;
            dbg!(state.global_factor);
            dbg!(state.norm().unwrap());
            assert!((state.norm().unwrap() - 1.0).abs() < 1e-10);
        }
        for (outcome, count) in counts {
            let outcome_str: String = outcome.iter().map(|&b| if b { '1' } else { '0' }).collect();
            println!("Outcome: {}, Count: {}", outcome_str, count);
        }
    }

    #[test]
    #[ignore]
    fn test_measurement_large_state() {
        // base_state = |000> + |100> + |010> + |111>
        let base_state = crate::test_utils::create_sample_stab_decomp_state();

        fn tensor(
            n: usize,
            state: &crate::state::StabilizerDecomposedState<Complex64>,
        ) -> crate::state::StabilizerDecomposedState<Complex64> {
            if n == 1 {
                state.clone()
            } else {
                let smaller = tensor(n - 1, state);
                smaller.kron(state).unwrap()
            }
        }

        let large_state = tensor(4, &base_state); // 4 copies -> 12 qubits

        let trials = 1600;
        let mut counts = std::collections::HashMap::new();

        let qubits_to_measure: Vec<usize> = vec![0, 2, 11];

        // Expected: 3/8: [0,0,0], 1/8: [0,0,1], 3/16: [1,0,0], 1/16: [1,0,1], 3/16: [1,1,0], 1/16: [1,1,1]
        for _ in 0..trials {
            let mut state = large_state.clone();
            let outcome = state.measure(&qubits_to_measure, None).unwrap();
            *counts.entry(outcome).or_insert(0) += 1;
            dbg!(state.global_factor);
            dbg!(state.norm().unwrap());
            assert!((state.norm().unwrap() - 1.0).abs() < 1e-10);
        }
        for (outcome, count) in counts {
            let outcome_str: String = outcome.iter().map(|&b| if b { '1' } else { '0' }).collect();
            println!("Outcome: {}, Count: {}", outcome_str, count);
        }
    }
}
