use crate::{StabilizerCHForm, error::{Error, Result}};
use ndarray::Axis;

impl StabilizerCHForm {
    pub(crate) fn _permuted(&self, axes: &[usize]) -> Result<Self> {
        if axes.len() != self.n {
            return Err(Error::InvalidPermutationLength(axes.len(), self.n));
        }

        let mut new_state = StabilizerCHForm::new(self.n)?;

        // Permute matrices by selecting rows and columns according to `axes`.
        for (new_i, &old_i) in axes.iter().enumerate() {
            for (new_j, &old_j) in axes.iter().enumerate() {
                new_state.mat_g[[new_i, new_j]] = self.mat_g[[old_i, old_j]];
                new_state.mat_f[[new_i, new_j]] = self.mat_f[[old_i, old_j]];
                new_state.mat_m[[new_i, new_j]] = self.mat_m[[old_i, old_j]];
            }
        }

        // Permute vectors. `select` takes the axes slice directly.
        new_state.gamma = self.gamma.select(Axis(0), axes);
        new_state.vec_v = self.vec_v.select(Axis(0), axes);
        new_state.vec_s = self.vec_s.select(Axis(0), axes);

        // Copy the scalar phase.
        new_state.omega = self.omega;
        new_state.phase_factor = self.phase_factor;

        Ok(new_state)
    }

    pub(crate) fn _permute(&mut self, axes: &[usize]) -> Result<()> {
        *self = self._permuted(axes)?;
        Ok(())
    }
}
