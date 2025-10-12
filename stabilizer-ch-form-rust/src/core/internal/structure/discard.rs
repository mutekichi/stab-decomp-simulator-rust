use crate::{StabilizerCHForm, error::ChFormError};
use ndarray::{Array1, Array2};

impl StabilizerCHForm {
    /// Discards (traces out) the qubit at index `qarg`.
    ///
    /// This is an in-place operation that modifies the state.
    /// NOTE: This function assumes that the qubit `qarg` has already been
    /// projected onto the |0> state and is disentangled from the rest.
    pub(crate) fn _discard(&mut self, qarg: usize) -> Result<(), ChFormError> {
        if qarg >= self.n {
            return Err(ChFormError::QubitIndexOutOfBounds(qarg, self.n));
        }

        // Ensure s[qarg], v[qarg] are false
        // and also G[qarg, :] and G[:, qarg] are zero except for the diagonal.
        // Also ensure M[qarg, :] and M[:, qarg] are zero.
        self._set_s_v_to_false(qarg)?;
        self._transform_g(qarg)?;
        self._transform_m(qarg);

        // Update self with the new (n-1)-qubit state
        self.n -= 1;
        self.mat_g = self._remove_row_col_from_matrix(&self.mat_g, qarg);
        self.mat_f = self._remove_row_col_from_matrix(&self.mat_f, qarg);
        self.mat_m = self._remove_row_col_from_matrix(&self.mat_m, qarg);

        self.gamma = self._remove_element_from_vector(&self.gamma, qarg);
        self.vec_v = self._remove_element_from_vector(&self.vec_v, qarg);
        self.vec_s = self._remove_element_from_vector(&self.vec_s, qarg);

        Ok(())
    }

    /// Returns a new StabilizerCHForm with the specified qubit discarded.
    ///
    /// NOTE: This function assumes that the qubit `qarg` has already been
    /// projected onto the |0> state.
    pub fn discarded(&self, qarg: usize) -> Result<StabilizerCHForm, &'static str> {
        let mut self_clone = self.clone();
        self_clone.discard(qarg)?;
        Ok(self_clone)
    }

    // --- Private helper methods ---

    /// Creates a new matrix by removing a specified row and column from the input matrix.
    fn _remove_row_col_from_matrix<T: Clone>(&self, matrix: &Array2<T>, index: usize) -> Array2<T> {
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
    fn _remove_element_from_vector<T: Clone>(&self, vector: &Array1<T>, index: usize) -> Array1<T> {
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

    // --- Private helper methods ---

    /// Sets s[qarg] and v[qarg] to false without changing the state.
    fn _set_s_v_to_false(&mut self, qarg: usize) -> Result<(), ChFormError> {
        if !self.vec_v[qarg] && !self.vec_s[qarg] {
            return Ok(());
        }

        let mut ok_index = None;
        for i in 0..self.n {
            if i != qarg && !self.vec_v[i] && !self.vec_s[i] {
                ok_index = Some(i);
                break;
            }
        }

        let final_ok_index = match ok_index {
            Some(i) => i,
            None => {
                let mut first = None;
                let mut second = None;
                for i in 0..self.n {
                    if !self.vec_v[i] && self.vec_s[i] {
                        if first.is_none() {
                            first = Some(i);
                        } else {
                            second = Some(i);
                            break;
                        }
                    }
                }

                if let (Some(f), Some(s)) = (first, second) {
                    self._right_multiply_cx(f, s)?;
                    self.vec_s[s] = false;
                    if s == qarg {
                        return Ok(());
                    }
                    s
                } else {
                    // Unreachable if the state is valid.
                    unreachable!("Could not find suitable qubits to zero out s[i].");
                }
            }
        };

        // SWAP qarg and final_ok_index
        self._right_multiply_cx(final_ok_index, qarg)?;
        self._right_multiply_cx(qarg, final_ok_index)?;
        self._right_multiply_cx(final_ok_index, qarg)?;

        self.vec_v.swap(qarg, final_ok_index);
        self.vec_s.swap(qarg, final_ok_index);

        Ok(())
    }

    /// Transforms G so that G[qarg, :] and G[:, qarg] are zero except for the diagonal.
    fn _transform_g(&mut self, qarg: usize) -> Result<(), ChFormError> {
        if !self.mat_g[[qarg, qarg]] {
            if let Some(pivot) = (0..self.n).find(|&i| i != qarg && self.mat_g[[qarg, i]]) {
                self._right_multiply_cx(qarg, pivot)?;
            } else {
                // This case should not happen if the state is valid.
            }
        }

        // Make G[i, qarg] = false for i != qarg (left-multiplication)
        for i in 0..self.n {
            if i != qarg && self.mat_g[[i, qarg]] {
                self._left_multiply_cx(qarg, i)?;
            }
        }

        // Make G[qarg, i] = false for i != qarg (right-multiplication)
        for i in 0..self.n {
            if i != qarg && self.mat_g[[qarg, i]] {
                if self.vec_v[i] {
                    unreachable!("Cannot zero G column due to v vector state.");
                }
                self._right_multiply_cx(i, qarg)?;
            }
        }
        Ok(())
    }

    /// Transforms M so that M[qarg, :] and M[:, qarg] are zero.
    fn _transform_m(&mut self, qarg: usize) -> Result<(), ChFormError> {
        // Left-multiplication gates
        for i in 0..self.n {
            if i != qarg && self.mat_m[[i, qarg]] {
                self._left_multiply_cx(qarg, i)?;
            }
        }
        if self.mat_m[[qarg, qarg]] {
            self._left_multiply_sdg(qarg)?;
        }

        // Right-multiplication gates
        for i in 0..self.n {
            if i != qarg && self.mat_m[[qarg, i]] {
                self._right_multiply_cz(qarg, i)?;
            }
        }

        Ok(())
    }
}
