use crate::error::{Error, Result};
use crate::state::{Coefficient, StabilizerDecomposedState};
use num_complex::Complex64;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    /// Measures the specified qubits and returns the measurement outcomes as a vector of booleans.
    /// `true` represents outcome `1`, and `false` represents outcome `0`.
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

    /// Measures all qubits in the state and returns the measurement outcomes as a vector of
    /// booleans.
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
    use std::collections::HashMap;
    use std::collections::HashSet;

    use super::*;
    use crate::error::Error;
    use crate::test_utils::create_all_zero_state;
    use crate::test_utils::create_sample_stab_decomp_state;

    #[test]
    fn test_measure_deterministic() {
        for i in 0..10 {
            let num_qubits = 3;
            // |000>
            let mut state = create_all_zero_state(num_qubits);
            let outcomes = state.measure_all(Some([i as u8 + 42; 32])).unwrap();
            assert_eq!(outcomes, vec![false, false, false]);
            assert!((state.norm().unwrap() - 1.0).abs() < 1e-10);
        }
    }

    #[test]
    fn test_measure_reproducibility() {
        for i in 0..10 {
            // Prepare bell state
            let mut base_state = create_all_zero_state(2);
            base_state.apply_h(0).unwrap();
            base_state.apply_cx(0, 1).unwrap();

            let seed = Some([i as u8 + 123; 32]);

            let mut state1 = base_state.clone();
            let outcome1 = state1.measure(&[0, 1], seed).unwrap();

            // Reset and measure again with the same seed
            let mut state2 = base_state.clone();
            let outcome2 = state2.measure(&[0, 1], seed).unwrap();

            // Assert reproducibility
            assert_eq!(
                outcome1, outcome2,
                "Measurements with the same seed must yield the same outcome."
            );
        }
    }

    #[test]
    fn test_measure_state_collapse() {
        // |+> state
        let mut state = create_all_zero_state(1);
        state.apply_h(0).unwrap();

        let outcomes = state.measure(&[0], None).unwrap();
        let outcome = outcomes[0];

        // Measure again immediately. The outcome must be consistent with the first measurement.
        let outcomes_retry = state.measure(&[0], None).unwrap();
        assert_eq!(
            outcomes, outcomes_retry,
            "Subsequent measurements on collapsed state must be consistent."
        );

        // Check the internal statevector matches the outcome
        let sv = state.to_statevector().unwrap();
        if outcome {
            // |1> -> index 1 should be approx 1.0, index 0 should be 0.0
            assert!((sv[1].norm() - 1.0).abs() < 1e-10);
            assert!(sv[0].norm() < 1e-10);
        } else {
            // |0> -> index 0 should be approx 1.0, index 1 should be 0.0
            assert!((sv[0].norm() - 1.0).abs() < 1e-10);
            assert!(sv[1].norm() < 1e-10);
        }
    }

    #[test]
    fn test_measure_statistics_with_seed() {
        // 1/2 (|000> + |001> + |010> + |111>)
        let base_state = create_sample_stab_decomp_state();

        let master_seed = 12345u64;
        let mut rng = StdRng::seed_from_u64(master_seed);

        let trials = 2000;
        let mut counts = HashMap::new();

        for _ in 0..trials {
            let mut state = base_state.clone();
            let sub_seed: [u8; 32] = rng.r#gen();
            let outcomes = state.measure_all(Some(sub_seed)).unwrap();
            *counts.entry(outcomes).or_insert(0) += 1;
        }

        let expected_outcomes = vec![
            vec![false, false, false], // |000>
            vec![true, false, false],  // |001>
            vec![false, true, false],  // |010>
            vec![true, true, true],    // |111>
        ];
        for expected in expected_outcomes {
            let count = *counts.get(&expected).unwrap_or(&0);
            let frequency = count as f64 / trials as f64;
            // We set a tolerance of 0.05 for frequency deviation
            assert!(
                (frequency - 0.25).abs() < 0.05,
                "Outcome {:?} frequency {} deviates from expected 0.25",
                expected,
                frequency
            );
        }
    }

    #[test]
    fn test_measure_rearranged_order() {
        let base_state = create_sample_stab_decomp_state();
        let rearranged_qargs = vec![2, 0, 1];
        let mut outcomes = HashSet::new();
        let trials = 30;

        for _ in 0..trials {
            let mut state_clone = base_state.clone();
            let outcome = state_clone.measure(&rearranged_qargs, None).unwrap();
            outcomes.insert(outcome);
        }
        let expected_outcomes = vec![
            vec![false, false, false], // |000> -> 000
            vec![false, true, false],  // |001> -> 010
            vec![false, false, true],  // |010> -> 001
            vec![true, true, true],    // |111> -> 111
        ];
        for expected in expected_outcomes {
            assert!(
                outcomes.contains(&expected),
                "Expected outcome {:?} not observed in measurements with reordered qargs",
                expected
            );
        }
    }

    #[test]
    fn test_measure_invalid_arguments() {
        let num_qubits = 3;
        let mut state = create_all_zero_state(num_qubits);

        // Qubit index out of bounds
        let res_oob = state.measure(&[3], None);
        assert!(matches!(res_oob, Err(Error::QubitIndexOutOfBounds(3, 3))));

        // Duplicate qubit indices
        let res_dup = state.measure(&[0, 1, 0], None);
        assert!(matches!(res_dup, Err(Error::DuplicateQubitIndex(0))));

        // Empty qubit indices
        let res_empty = state.measure(&[], None);
        assert!(matches!(res_empty, Err(Error::EmptyQubitIndices)));
    }
}
