use crate::{
    StabilizerCHForm,
    error::{Error, Result},
};
use ndarray::{Array1, Array2};

impl StabilizerCHForm {
    /// Discards the specified qubit from the state.
    ///
    /// NOTE: This function assumes that the qubit `qarg` has already been
    /// projected onto the |0> state. You need to project the qubit onto |0> before
    /// calling this function. If this is not the case, the behavior is undefined.
    ///
    ///
    /// # Arguments
    /// * `qarg` - The index of the qubit to discard.
    ///
    /// ## Errors
    /// Returns an [`Error`] if the qubit index is out of bounds. Note that
    /// this function does not check if the qubit is properly projected onto |0>.
    pub fn discard(&mut self, qarg: usize) -> Result<()> {
        if qarg >= self.n {
            return Err(Error::QubitIndexOutOfBounds(qarg, self.n));
        }

        // Ensure s[qarg], v[qarg] are false
        // and also G[qarg, :] and G[:, qarg] are zero except for the diagonal.
        // Also ensure M[qarg, :] and M[:, qarg] are zero.
        self.set_s_v_to_false(qarg)?;
        self.transform_g(qarg)?;
        self.transform_m(qarg)?;

        // Update self with the new (n-1)-qubit state
        self.n -= 1;
        self.mat_g = self.remove_row_col_from_matrix(&self.mat_g, qarg);
        self.mat_f = self.remove_row_col_from_matrix(&self.mat_f, qarg);
        self.mat_m = self.remove_row_col_from_matrix(&self.mat_m, qarg);

        self.gamma = self.remove_element_from_vector(&self.gamma, qarg);
        self.vec_v = self.remove_element_from_vector(&self.vec_v, qarg);
        self.vec_s = self.remove_element_from_vector(&self.vec_s, qarg);

        Ok(())
    }

    /// Returns a new StabilizerCHForm with the specified qubit discarded.
    ///
    /// NOTE: This function assumes that the qubit `qarg` has already been
    /// projected onto the |0> state. You need to project the qubit onto |0> before
    /// calling this function. If this is not the case, the behavior is undefined.
    ///
    /// ## Arguments
    /// * `qarg` - The index of the qubit to discard.
    ///
    /// ## Returns
    /// A [`Result`] containing the new `StabilizerCHForm` with the specified qubit discarded.
    pub fn discarded(&self, qarg: usize) -> Result<StabilizerCHForm> {
        let mut self_clone = self.clone();
        self_clone.discard(qarg)?;
        Ok(self_clone)
    }

    /// Creates a new matrix by removing a specified row and column from the input matrix.
    fn remove_row_col_from_matrix<T: Clone>(&self, matrix: &Array2<T>, index: usize) -> Array2<T> {
        let mut new_matrix = Array2::from_elem((self.n, self.n), matrix[[0, 0]].clone());
        let mut new_i = 0;
        for i in 0..=self.n {
            if i == index {
                continue;
            }
            let mut new_j = 0;
            for j in 0..=self.n {
                if j == index {
                    continue;
                }
                new_matrix[[new_i, new_j]] = matrix[[i, j]].clone();
                new_j += 1;
            }
            new_i += 1;
        }
        new_matrix
    }

    /// Creates a new vector by removing a specified element from the input vector.
    fn remove_element_from_vector<T: Clone>(&self, vector: &Array1<T>, index: usize) -> Array1<T> {
        let mut new_vector = Array1::from_elem(self.n, vector[0].clone());
        let mut new_i = 0;
        for i in 0..=self.n {
            if i == index {
                continue;
            }
            new_vector[new_i] = vector[i].clone();
            new_i += 1;
        }
        new_vector
    }

    fn set_s_v_to_false(&mut self, qarg: usize) -> Result<()> {
        if !self.vec_v[qarg] && !self.vec_s[qarg] {
            return Ok(());
        }

        // Find a 'clean' qubit that satisfies v[i] = s[i] = 0
        let mut clean_index = (0..self.n).find(|&i| i != qarg && !self.vec_v[i] && !self.vec_s[i]);
        if clean_index.is_none() {
            // Find two qubits to create a clean one. qarg can be one of them.
            let mut candidates = (0..self.n).filter(|&i| !self.vec_v[i] && self.vec_s[i]);
            if let (Some(f), Some(s)) = (candidates.next(), candidates.next()) {
                self.right_multiply_cx(f, s)?;
                self.vec_s[s] = false;
                clean_index = Some(s);
            }
        }

        // If the qubit state is valid (i.e. can be discarded), there must be a clean qubit.
        let target = clean_index.ok_or(Error::CannotDiscardQubit(qarg))?;

        // If qarg itself became clean during the process, no SWAP is needed.
        if target == qarg {
            return Ok(());
        }

        // Move the clean qubit to qarg position
        self.right_multiply_cx(target, qarg)?;
        self.right_multiply_cx(qarg, target)?;
        self.right_multiply_cx(target, qarg)?;

        self.vec_v.swap(qarg, target);
        self.vec_s.swap(qarg, target);

        Ok(())
    }

    /// Transforms G so that G[qarg, :] and G[:, qarg] are zero except for the diagonal.
    fn transform_g(&mut self, qarg: usize) -> Result<()> {
        if !self.mat_g[[qarg, qarg]] {
            if let Some(pivot) = (0..self.n).find(|&i| i != qarg && self.mat_g[[qarg, i]]) {
                self.right_multiply_cx(qarg, pivot)?;
            } else {
                unreachable!("Cannot zero G row due to G matrix state.");
            }
        }

        // Make G[i, qarg] = false for i != qarg.
        for i in 0..self.n {
            if i != qarg && self.mat_g[[i, qarg]] {
                self.left_multiply_cx(qarg, i)?;
            }
        }

        // Make G[qarg, i] = false for i != qarg.
        for i in 0..self.n {
            if i != qarg && self.mat_g[[qarg, i]] {
                if self.vec_v[i] {
                    unreachable!("Cannot zero G column due to v vector state.");
                }
                self.right_multiply_cx(i, qarg)?;
            }
        }
        Ok(())
    }

    /// Transforms M so that M[qarg, :] and M[:, qarg] are zero.
    fn transform_m(&mut self, qarg: usize) -> Result<()> {
        for i in 0..self.n {
            if i != qarg && self.mat_m[[i, qarg]] {
                self.left_multiply_cx(qarg, i)?;
            }
        }
        if self.mat_m[[qarg, qarg]] {
            self.left_multiply_sdg(qarg)?;
        }

        for i in 0..self.n {
            if i != qarg && self.mat_m[[qarg, i]] {
                self.right_multiply_cz(qarg, i)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use num_complex::Complex64;

    use crate::circuit::CliffordCircuit;
    use crate::test_utils::{assert_eq_complex_array1, tensor_statevectors};

    use super::*;

    #[test]
    fn test_discard() {
        let mut trials = 0;
        let mut successes = 0;
        let num_qubits = 5;
        while successes < 10 && trials < 1000 {
            trials += 1;
            let mut seed = [0u8; 32];
            let trial_bytes = (trials as u64).to_le_bytes();
            seed[0..8].copy_from_slice(&trial_bytes);
            let random_circuit = CliffordCircuit::random_clifford(num_qubits, Some(seed));
            let mut ch_form = StabilizerCHForm::from_clifford_circuit(&random_circuit).unwrap();

            let qubit_to_discard = num_qubits - 1;
            // Project the qubit onto |0>]
            if ch_form.project(qubit_to_discard, false).is_err() {
                continue; // Cannot discard this qubit, try again
            }
            let expected_original_sv = ch_form.to_statevector().unwrap();
            // |0>
            let zero_state = ndarray::array![Complex64::new(1.0, 0.0), Complex64::new(0.0, 0.0)];

            match ch_form.discard(qubit_to_discard) {
                Ok(()) => {
                    let sv_after_discard = ch_form.to_statevector().unwrap();
                    let expected_after_discard =
                        tensor_statevectors(&sv_after_discard, &zero_state);
                    assert_eq_complex_array1(&expected_original_sv, &expected_after_discard);
                    successes += 1;
                }
                Err(_) => {
                    // Discard is always expected to succeed after projection
                    panic!("Discard failed after successful projection");
                }
            }
        }
    }
}
