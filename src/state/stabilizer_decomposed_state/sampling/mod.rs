use crate::{
    state::{Coefficient, StabilizerDecomposedState},
    types::{
        error::Error,
        result::shot_count::{self, ShotCount},
    },
};
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand_distr::{Binomial, Distribution};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    pub(crate) fn _sample(
        &self,
        qargs: &[usize],
        shots: usize,
        seed: Option<[u8; 32]>,
    ) -> Result<ShotCount, Error> {
        // ShotCount: HashMap<Vec<bool>, usize>
        let mut shot_count: ShotCount = shot_count::ShotCount::new();
        let mut rng = match seed {
            Some(s) => StdRng::from_seed(s),
            None => StdRng::from_entropy(),
        };
        // Sort qargs to
        // Start the recursive sampling process
        self._recursive_sample(
            qargs,
            0,
            shots,
            &mut Vec::with_capacity(qargs.len()),
            &mut shot_count,
            &mut rng,
        );

        Ok(shot_count)
    }

    /// Used to recursively sample from the stabilizer decomposed state.
    fn _recursive_sample(
        &self,
        qubit_indices: &[usize],
        current_qarg: usize,
        current_shots: usize,
        current_outcome: &mut Vec<bool>,
        shot_count: &mut ShotCount,
        rng: &mut StdRng,
    ) {
        // There are no shots to process, stop the recursion
        if current_shots == 0 {
            return;
        }
        // Base case: If we've processed all qubits, record the result
        if current_qarg == qubit_indices.len() {
            *shot_count.entry(current_outcome.clone()).or_insert(0) += current_shots;
            return;
        }
        let qarg = qubit_indices[current_qarg];

        // Project the qubit onto |0> and |1> to further sample the outcomes and also to
        // calculate the probabilities of measuring 0 and 1.
        let mut state_zero = self.clone();
        let mut state_one = self.clone();
        let proj_zero_result = state_zero._project_unnormalized(qarg, false);
        let proj_one_result = state_one._project_unnormalized(qarg, true);

        if proj_zero_result.is_err() {
            // Projection to |0> is impossible, all shots must be |1>
            current_outcome.push(true);
            // If the projection to |0> is impossible, the projection to |1> must be possible
            state_one._recursive_sample(
                qubit_indices,
                current_qarg + 1,
                current_shots,
                current_outcome,
                shot_count,
                rng,
            );
            current_outcome.pop();
            return;
        }

        if proj_one_result.is_err() {
            // Projection to |1> is impossible, all shots must be |0>
            current_outcome.push(false);
            // If the projection to |1> is impossible, the projection to |0> must be possible
            assert!(proj_zero_result.is_ok());
            state_zero._recursive_sample(
                qubit_indices,
                current_qarg + 1,
                current_shots,
                current_outcome,
                shot_count,
                rng,
            );
            current_outcome.pop();
            return;
        }

        let prob_zero =
            state_zero._norm_squared() / (state_zero._norm_squared() + state_one._norm_squared());

        // Sample the number of 0 outcomes using a binomial distribution
        let binom = match Binomial::new(current_shots as u64, prob_zero) {
            Ok(b) => b,
            Err(_) => {
                Error::Sampling("Failed to create binomial distribution".to_string());
                return;
            }
        };

        // Sample the number of outcome 0 from the binomial distribution
        let num_zeros = binom.sample(rng) as usize;
        let num_ones = current_shots - num_zeros;

        // forward the current outcome with a 0 measurement result
        current_outcome.push(false);
        // Recurse for the next qubit with the number of 0 and 1 outcomes
        state_zero._recursive_sample(
            qubit_indices,
            current_qarg + 1,
            num_zeros,
            current_outcome,
            shot_count,
            rng,
        );

        // backtrack the current outcome to replace the last entry with a 1 measurement result
        current_outcome.pop();
        current_outcome.push(true);

        // Recurse for the next qubit with the number of 1 outcomes
        state_one._recursive_sample(
            qubit_indices,
            current_qarg + 1,
            num_ones,
            current_outcome,
            shot_count,
            rng,
        );

        // backtrack the current outcome to remove the last entry
        current_outcome.pop();
    }
}

#[cfg(test)]
mod test {
    use num_complex::Complex64;

    #[test]
    fn test_sampling() {
        // sample_state = |000> + |100> + |010> + |111>
        let sample_state = crate::test_utils::create_sample_stab_decomp_state();
        let shots = 6400;
        let qargs = vec![0, 1, 2];
        let seed = None;
        let result = sample_state._sample(&qargs, shots, seed);
        match result {
            Ok(shot_count) => {
                for (outcome, count) in shot_count.iter() {
                    println!("{:?}: {}", outcome, count);
                }
            }
            Err(e) => {
                panic!("Sampling failed with error: {:?}", e);
            }
        }
    }

    #[test]
    #[ignore]
    fn test_sampling_large_state() {
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
                smaller.kron(state)
            }
        }

        fn run_sampling(
            state: &crate::state::StabilizerDecomposedState<Complex64>,
            qargs: &[usize],
            shots: usize,
            seed: Option<[u8; 32]>,
            samples_to_print: usize,
        ) {
            let result = state._sample(qargs, shots, seed);
            match result {
                Ok(shot_count) => {
                    println!("Showing up to {} samples:", samples_to_print);
                    for (i, (outcome, count)) in shot_count.iter().enumerate() {
                        if i >= samples_to_print {
                            break;
                        }
                        let outcome_str: String = outcome
                            .iter()
                            .map(|&b| if b { '1' } else { '0' })
                            .collect();
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
