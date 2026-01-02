use crate::{
    StabilizerCHForm,
    error::{Error, Result},
};
use ndarray::Axis;

impl StabilizerCHForm {
    /// Returns a new StabilizerCHForm with the qubits permuted.
    ///
    /// ## Arguments
    /// * `axes` - A slice representing the new order of qubits. For `n` qubits,
    ///   this must be a permutation of `[0, 1, ..., n-1]`.
    ///
    /// ## Returns
    /// A [`Result`] containing the new `StabilizerCHForm` with permuted qubits.
    pub fn permuted(&self, axes: &[usize]) -> Result<Self> {
        if axes.len() != self.n {
            return Err(Error::InvalidPermutationLength(axes.len(), self.n));
        }
        // Check that `axes` is a valid permutation of [0, 1, ..., n-1]
        let mut check_vec = vec![false; self.n];
        for &axis in axes {
            if axis >= self.n || check_vec[axis] {
                return Err(Error::InvalidPermutation(axes.to_vec()));
            }
            check_vec[axis] = true;
        }

        let mut new_state = StabilizerCHForm::new(self.n)?;

        for (new_i, &old_i) in axes.iter().enumerate() {
            for (new_j, &old_j) in axes.iter().enumerate() {
                new_state.mat_g[[new_i, new_j]] = self.mat_g[[old_i, old_j]];
                new_state.mat_f[[new_i, new_j]] = self.mat_f[[old_i, old_j]];
                new_state.mat_m[[new_i, new_j]] = self.mat_m[[old_i, old_j]];
            }
        }

        new_state.gamma = self.gamma.select(Axis(0), axes);
        new_state.vec_v = self.vec_v.select(Axis(0), axes);
        new_state.vec_s = self.vec_s.select(Axis(0), axes);

        new_state.omega = self.omega;
        new_state.phase_factor = self.phase_factor;

        Ok(new_state)
    }

    /// Permutes the qubits of the state in-place.
    ///
    /// ## Arguments
    /// * `axes` - A slice representing the new order of qubits. For `n` qubits,
    ///   this must be a permutation of `[0, 1, ..., n-1]`.
    ///
    /// ## Returns
    /// A [`Result`] indicating success or failure.
    pub fn permute(&mut self, axes: &[usize]) -> Result<()> {
        *self = self.permuted(axes)?;
        Ok(())
    }
}
