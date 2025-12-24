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

        let buffer = if num_qubits <= 32 {
            let mut outcomes = Vec::new();
            self.recursive_sample(qargs, 0, shots, u32::zero(), &mut outcomes, &mut rng)?;
            SamplingBuffer::U32(outcomes)
        } else if num_qubits <= 64 {
            let mut outcomes = Vec::new();
            self.recursive_sample(qargs, 0, shots, u64::zero(), &mut outcomes, &mut rng)?;
            SamplingBuffer::U64(outcomes)
        } else if num_qubits <= 128 {
            let mut outcomes = Vec::new();
            self.recursive_sample(qargs, 0, shots, u128::zero(), &mut outcomes, &mut rng)?;
            SamplingBuffer::U128(outcomes)
        } else {
            return Err(Error::SamplingTooManyQubits);
        };

        Ok(buffer.finalize(num_qubits))
    }

    /// Used to recursively sample from the stabilizer decomposed state.
    fn recursive_sample<I: OutcomeInteger>(
        &self,
        qubit_indices: &[usize],
        current_qarg: usize,
        current_shots: usize,
        current_outcome: I,
        outcome_counts: &mut Vec<(I, usize)>,
        rng: &mut StdRng,
    ) -> Result<()> {
        // There are no shots to process, stop the recursion
        if current_shots == 0 {
            return Ok(());
        }
        // Base case: If we've processed all qubits, record the result
        if current_qarg == qubit_indices.len() {
            outcome_counts.push((current_outcome, current_shots));
            return Ok(());
        }
        let qarg = qubit_indices[current_qarg];

        // Project the qubit onto |0> and |1> to further sample the outcomes and also to
        // calculate the probabilities of measuring 0 and 1.
        let mut state_zero = self.clone();
        let mut state_one = self.clone();
        let proj_zero_result = state_zero.project_unnormalized(qarg, false);
        let proj_one_result = state_one.project_unnormalized(qarg, true);

        let next_bits = current_outcome.set_bit(current_qarg);

        if proj_zero_result.is_err() {
            // Projection to |0> is impossible, all shots must be |1>
            // If the projection to |0> is impossible, the projection to |1> must be possible
            state_one.recursive_sample(
                qubit_indices,
                current_qarg + 1,
                current_shots,
                next_bits,
                outcome_counts,
                rng,
            )?;
            return Ok(());
        }

        if proj_one_result.is_err() {
            // Projection to |1> is impossible, all shots must be |0>
            // We do not set the bit since it is already 0
            // If the projection to |1> is impossible, the projection to |0> must be possible
            state_zero.recursive_sample(
                qubit_indices,
                current_qarg + 1,
                current_shots,
                current_outcome,
                outcome_counts,
                rng,
            )?;
            return Ok(());
        }

        let mut prob_zero =
            state_zero.norm_squared()? / (state_zero.norm_squared()? + state_one.norm_squared()?);

        // Ensure probability is not larger than 1 due to numerical errors
        prob_zero = prob_zero.clamp(0.0, 1.0);

        // Sample the number of 0 outcomes using a binomial distribution
        let binom = match Binomial::new(current_shots as u64, prob_zero) {
            Ok(b) => b,
            Err(e) => {
                eprintln!("{}", prob_zero);
                return Err(Error::Binomial(e));
            }
        };

        // Sample the number of outcome 0 from the binomial distribution
        let num_zeros = binom.sample(rng) as usize;
        let num_ones = current_shots - num_zeros;

        // Recurse for outcome 0
        state_zero.recursive_sample(
            qubit_indices,
            current_qarg + 1,
            num_zeros,
            current_outcome,
            outcome_counts,
            rng,
        )?;

        // Recurse for outcome 1
        state_one.recursive_sample(
            qubit_indices,
            current_qarg + 1,
            num_ones,
            next_bits,
            outcome_counts,
            rng,
        )?;

        Ok(())
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
        let result = sample_state.sample(&qargs, shots, seed);
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
