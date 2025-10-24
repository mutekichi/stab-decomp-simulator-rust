use crate::{StabilizerCHForm, error::Result};
use ndarray::{Axis, s};

impl StabilizerCHForm {
    pub fn _kron(&self, other: &StabilizerCHForm) -> Result<StabilizerCHForm> {
        let n_total = self.n + other.n;
        let mut new_state = StabilizerCHForm::new(n_total)?;

        // Create block-diagonal matrices for G, F, and M
        new_state
            .mat_g
            .slice_mut(s![..self.n, ..self.n])
            .assign(&self.mat_g);
        new_state
            .mat_g
            .slice_mut(s![self.n.., self.n..])
            .assign(&other.mat_g);

        new_state
            .mat_f
            .slice_mut(s![..self.n, ..self.n])
            .assign(&self.mat_f);
        new_state
            .mat_f
            .slice_mut(s![self.n.., self.n..])
            .assign(&other.mat_f);

        new_state
            .mat_m
            .slice_mut(s![..self.n, ..self.n])
            .assign(&self.mat_m);
        new_state
            .mat_m
            .slice_mut(s![self.n.., self.n..])
            .assign(&other.mat_m);

        // Concatenate vectors by creating a slice of views directly.
        new_state.gamma = ndarray::concatenate(Axis(0), &[self.gamma.view(), other.gamma.view()])
            .expect("Failed to concatenate gamma vectors");

        new_state.vec_v = ndarray::concatenate(Axis(0), &[self.vec_v.view(), other.vec_v.view()])
            .expect("Failed to concatenate v vectors");

        new_state.vec_s = ndarray::concatenate(Axis(0), &[self.vec_s.view(), other.vec_s.view()])
            .expect("Failed to concatenate s vectors");

        // Combine global phases and phase factors
        new_state.set_global_phase(self.global_phase() * other.global_phase());
        new_state.phase_factor = self.phase_factor * other.phase_factor;

        Ok(new_state)
    }
}
