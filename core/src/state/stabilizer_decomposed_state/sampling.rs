use crate::{
    error::{Error, Result},
    state::{Coefficient, StabilizerDecomposedState},
    types::shot_count::{OutcomeInteger, SamplingBuffer, ShotCount},
};
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand_distr::{Binomial, Distribution};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    pub(crate) fn sample(
        &self,
        qargs: &[usize],
        shots: usize,
        seed: Option<[u8; 32]>,
    ) -> Result<ShotCount> {
        self.validate_qargs(qargs)?;
        let num_qubits = qargs.len();

        let mut rng = match seed {
            Some(s) => StdRng::from_seed(s),
            None => StdRng::from_entropy(),
        };

        // Pair each qarg with its target bit index in the result.
        // Then, sort by physical qubit index (qarg) in descending order.
        // This ensures that discarding a qubit does not shift the indices of
        // unprocessed (smaller index) qubits, maintaining valid indices for subsequent steps.
        let mut sorted_qargs: Vec<(usize, usize)> = qargs
            .iter()
            .enumerate()
            .map(|(bit_index, &qarg)| (qarg, bit_index))
            .collect();

        sorted_qargs.sort_by(|a, b| b.0.cmp(&a.0));

        let buffer = if num_qubits <= 32 {
            let mut outcomes = Vec::new();
            self.recursive_sample(
                &sorted_qargs,
                0,
                shots,
                u32::zero(),
                &mut outcomes,
                &mut rng,
            )?;
            SamplingBuffer::U32(outcomes)
        } else if num_qubits <= 64 {
            let mut outcomes = Vec::new();
            self.recursive_sample(
                &sorted_qargs,
                0,
                shots,
                u64::zero(),
                &mut outcomes,
                &mut rng,
            )?;
            SamplingBuffer::U64(outcomes)
        } else if num_qubits <= 128 {
            let mut outcomes = Vec::new();
            self.recursive_sample(
                &sorted_qargs,
                0,
                shots,
                u128::zero(),
                &mut outcomes,
                &mut rng,
            )?;
            SamplingBuffer::U128(outcomes)
        } else {
            return Err(Error::SamplingTooManyQubits);
        };

        Ok(buffer.finalize(num_qubits))
    }

    /// Recursively sample from the state, discarding qubits as they are measured.
    fn recursive_sample<I: OutcomeInteger>(
        &self,
        qubit_indices: &[(usize, usize)], // (physical_qarg, bit_position)
        current_idx: usize,
        current_shots: usize,
        current_outcome: I,
        outcome_counts: &mut Vec<(I, usize)>,
        rng: &mut StdRng,
    ) -> Result<()> {
        // Stop recursion if there are no shots to process.
        if current_shots == 0 {
            return Ok(());
        }
        // Base case: All qubits processed, record the result.
        if current_idx == qubit_indices.len() {
            outcome_counts.push((current_outcome, current_shots));
            return Ok(());
        }

        let (qarg, bit_pos) = qubit_indices[current_idx];

        // Clone the state to project and calculate probabilities.
        // Note: Since we discard qubits, the state is consumed in each branch.
        let mut state_zero = self.clone();
        let mut state_one = self.clone();

        let proj_zero_result = state_zero.project_unnormalized(qarg, false);
        let proj_one_result = state_one.project_unnormalized(qarg, true);

        // --- Case 1: Projection to |0> is impossible (Probability ~ 0%) ---
        if proj_zero_result.is_err() {
            // All shots must result in |1>.
            // Process: Project to |1> (implicitly done or checked) -> Apply X to reset to |0> -> Discard -> Recurse.
            if proj_one_result.is_err() {
                // Ideally unreachable unless the state norm is zero.
                unreachable!("Both projections failed in sampling.");
            }

            state_one.apply_x(qarg)?; // |1> -> |0>
            state_one.discard(qarg)?; // Discard the measured qubit.

            let next_outcome = current_outcome.set_bit(bit_pos);
            return state_one.recursive_sample(
                qubit_indices,
                current_idx + 1,
                current_shots,
                next_outcome,
                outcome_counts,
                rng,
            );
        }

        // --- Case 2: Projection to |1> is impossible (Probability ~ 0%) ---
        if proj_one_result.is_err() {
            // All shots must result in |0>.
            // Process: Project to |0> (implicitly done) -> Discard -> Recurse.
            state_zero.discard(qarg)?; // Discard the measured qubit.

            return state_zero.recursive_sample(
                qubit_indices,
                current_idx + 1,
                current_shots,
                current_outcome, // Bit remains 0.
                outcome_counts,
                rng,
            );
        }

        // --- Case 3: Both projections are possible ---
        // Calculate probability of measuring 0.
        let prob_zero = (state_zero.norm_squared()?
            / (state_zero.norm_squared()? + state_one.norm_squared()?))
        .clamp(0.0, 1.0);

        // Distribute shots using a binomial distribution.
        let binom = Binomial::new(current_shots as u64, prob_zero).map_err(Error::Binomial)?;
        let num_zeros = binom.sample(rng) as usize;
        let num_ones = current_shots - num_zeros;

        // Recurse for outcome 0
        if num_zeros > 0 {
            state_zero.discard(qarg)?;
            state_zero.recursive_sample(
                qubit_indices,
                current_idx + 1,
                num_zeros,
                current_outcome,
                outcome_counts,
                rng,
            )?;
        }

        // Recurse for outcome 1
        if num_ones > 0 {
            state_one.apply_x(qarg)?; // |1> -> |0>
            state_one.discard(qarg)?;
            let next_outcome_one = current_outcome.set_bit(bit_pos);
            state_one.recursive_sample(
                qubit_indices,
                current_idx + 1,
                num_ones,
                next_outcome_one,
                outcome_counts,
                rng,
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::error::Error;
    use num_complex::Complex64;
    use std::collections::HashSet;

    #[test]
    fn test_sampling() {
        // sample_state = |000> + |001> + |010> + |111> (Little Endian)
        let sample_state = crate::test_utils::create_sample_stab_decomp_state();
        let shots = 6400;
        let qargs = vec![0, 1, 2];
        let seed = Some(vec![42u8; 32].try_into().unwrap());
        let result = sample_state.sample(&qargs, shots, seed);

        match result {
            Ok(shot_count) => {
                // Assert that the total counts equal the number of shots
                let total_counts: usize = shot_count.iter().map(|(_, count)| *count).sum();
                assert_eq!(total_counts, shots);

                // Check that the outcomes are as expected.
                // Note: The state is little-endian (q0 is LSB).
                // |001> corresponds to q0=1, q1=0, q2=0 -> [true, false, false]
                let expected_outcomes: HashSet<Vec<bool>> = [
                    vec![false, false, false], // |000>
                    vec![true, false, false],  // |001>
                    vec![false, true, false],  // |010>
                    vec![true, true, true],    // |111>
                ]
                .into_iter()
                .collect();

                let mut unique_outcomes = 0;
                for (outcome, count) in shot_count.iter() {
                    assert!(
                        expected_outcomes.contains(outcome),
                        "Unexpected outcome: {:?}",
                        outcome
                    );

                    // Each expected outcome should appear approximately shots / 4 times
                    let expected_count = shots as f64 / 4.0;
                    let tolerance = expected_count * 0.1;
                    assert!(
                        (*count as f64) > (expected_count - tolerance)
                            && (*count as f64) < (expected_count + tolerance),
                        "Count for {:?} is out of expected range: Got {}, expected: ~{}",
                        outcome,
                        count,
                        expected_count
                    );

                    unique_outcomes += 1;
                }
                // Ensure we have all 4 unique outcomes
                assert_eq!(unique_outcomes, 4, "Expected 4 unique outcomes.");
            }
            Err(e) => {
                panic!("Sampling failed with error: {:?}", e);
            }
        }
    }

    #[test]
    fn test_sampling_errors() {
        let state = crate::test_utils::create_sample_stab_decomp_state(); // 3 qubits

        // Empty qargs
        let result_empty = state.sample(&[], 100, None);
        assert!(result_empty.is_err(), "Should fail with empty qargs");
        match result_empty {
            Err(Error::EmptyQubitIndices) => {}
            Err(e) => panic!("Expected EmptyQubitIndices, got {:?}", e),
            Ok(_) => panic!("Should not succeed with empty qargs"),
        }

        // Index out of bounds
        let result_out_of_bounds = state.sample(&[0, 1, 3], 100, None);
        assert!(
            result_out_of_bounds.is_err(),
            "Should fail with index out of bounds"
        );
        match result_out_of_bounds {
            Err(Error::QubitIndexOutOfBounds(idx, num)) => {
                assert_eq!(idx, 3);
                assert_eq!(num, 3);
            }
            Err(e) => panic!("Expected QubitIndexOutOfBounds, got {:?}", e),
            Ok(_) => panic!("Should not succeed with out of bounds index"),
        }

        // Duplicate indices
        let result_duplicate = state.sample(&[0, 1, 1], 100, None);
        assert!(
            result_duplicate.is_err(),
            "Should fail with duplicate indices"
        );
        match result_duplicate {
            Err(Error::DuplicateQubitIndex(idx)) => {
                assert_eq!(idx, 1);
            }
            Err(e) => panic!("Expected DuplicateQubitIndex, got {:?}", e),
            Ok(_) => panic!("Should not succeed with duplicate indices"),
        }
    }

    // This test is ignored by default due to its potentially long runtime.
    #[test]
    #[ignore]
    fn test_sampling_large_state() {
        // base_state = |000> + |001> + |010> + |111>
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

        fn run_sampling(
            state: &crate::state::StabilizerDecomposedState<Complex64>,
            qargs: &[usize],
            shots: usize,
            seed: Option<[u8; 32]>,
            samples_to_print: usize,
        ) {
            let result = state.sample(qargs, shots, seed);
            match result {
                Ok(shot_count) => {
                    println!("Showing up to {} samples:", samples_to_print);
                    for (i, (outcome, count)) in shot_count.iter().enumerate() {
                        if i >= samples_to_print {
                            break;
                        }
                        let outcome_str: String =
                            outcome.iter().map(|&b| if b { '1' } else { '0' }).collect();
                        println!("{}: {}", outcome_str, count);
                    }
                }
                Err(e) => {
                    panic!("Sampling failed with error: {:?}", e);
                }
            }
        }
        let num_tensors_list = [1, 2, 3, 4, 5, 6, 7, 8];
        let shots_list = [10, 100, 10000];

        for &num_tensors in num_tensors_list.iter() {
            let large_state = tensor(num_tensors, &base_state);
            let num_qubits = large_state.num_qubits;
            let qargs: Vec<usize> = (0..num_qubits).collect();
            for &shots in shots_list.iter() {
                println!(
                    "Sampling {} shots from a state with {} qubits",
                    shots, num_qubits
                );
                let start_time = std::time::Instant::now();
                run_sampling(&large_state, &qargs, shots, None, 10);
                let duration = start_time.elapsed();
                println!("Time elapsed in sampling: {:?}", duration);
            }
        }
    }
}
