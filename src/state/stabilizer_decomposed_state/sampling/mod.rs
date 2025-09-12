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
        // Base case: If we've processed all qubits, record the result
        if current_qarg == qubit_indices.len() {
            *shot_count.entry(current_outcome.clone()).or_insert(0) += current_shots;
            return;
        }
        // There are no shots to process, stop the recursion
        if current_shots == 0 {
            return;
        }
        let qarg = qubit_indices[current_qarg];
        // Calculate the probability of measuring 0 on the current qubit
        let prob_0 = self._probability_of_zero(qarg);

        // Sample the number of 0 outcomes using a binomial distribution
        let binom = match Binomial::new(current_shots as u64, prob_0) {
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
        self._recursive_sample(
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
        self._recursive_sample(
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

    /// Calculate the probability of measuring 0 on a specific qubit.
    fn _probability_of_zero(&self, qarg: usize) -> f64 {
        dbg!(qarg);
        // Placeholder implementation
        0.7
    }
}
