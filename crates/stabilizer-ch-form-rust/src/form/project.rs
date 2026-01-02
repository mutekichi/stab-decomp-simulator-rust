use crate::{
    StabilizerCHForm,
    error::{Error, Result},
    form::types::{PhaseFactor, QubitState},
};

impl StabilizerCHForm {
    /// Projects a qubit onto a computational basis state (`|0>` or `|1>`).
    ///
    /// This operation modifies the stabilizer state in place.
    ///
    /// In a stabilizer state, measuring a qubit in the computational basis yields either a
    /// deterministic outcome (`|0>` or `|1>`) or a perfectly random one (50% probability for each).
    /// This function attempts to force the qubit into the specified `outcome`, succeeding if the
    /// projection is physically possible.
    ///
    /// ## Arguments
    /// * `qarg`: The index of the qubit to project.
    /// * `outcome`: The desired basis state to project onto (`false` for `|0>`, `true` for `|1>`).
    ///
    /// ## Returns
    /// A `Result` indicating the outcome of the projection:
    /// * `Ok(true)` if the projection was **deterministic**. This means the qubit was already
    ///   in the desired state. The stabilizer state is unchanged.
    /// * `Ok(false)` if the projection was **non-deterministic** (probabilistic). This means the
    ///   qubit was in a superposition and has now been collapsed to the desired state. The
    ///   stabilizer state has been updated.
    ///
    /// ## Errors
    /// Returns an `ChFormError` if the projection is impossible. This occurs when the qubit has a
    /// deterministic value that is orthogonal to the desired `outcome` (e.g., attempting to
    /// project a qubit in state `|0>` onto `|1>`).
    pub fn project(&mut self, qarg: usize, outcome: bool) -> Result<bool> {
        if qarg >= self.n {
            return Err(Error::QubitIndexOutOfBounds(qarg, self.n));
        }

        let qubit_state = self.get_qubit_state(qarg)?;
        match qubit_state {
            QubitState::Determined(value) => {
                if value != outcome {
                    Err(Error::ImpossibleProjection {
                        qubit_index: qarg,
                        desired: outcome,
                    })
                } else {
                    // No change needed if the state is already determined and matches the outcome.
                    Ok(true)
                }
            }
            QubitState::Superposition => {
                // Collapse the state to the desired outcome.
                // Applys the operator: (I + (-1)^(1-outcome) * Z_qarg) / 2
                // Z_arg application can be represented as:
                //   Z_qarg U_C U_H |s> = (-1)^α |t>
                // according to eq.(48) and (49) in arXiv:1808.00128
                let g_row = self.mat_g.row(qarg);
                let vec_t = &(&g_row & &self.vec_v) ^ &self.vec_s;
                let alpha = g_row
                    .iter()
                    .zip(&self.vec_v)
                    .zip(&self.vec_s)
                    .filter(|&((&g, &v), &s)| g && !v && s)
                    .count()
                    % 2
                    != 0;
                let delta = if alpha ^ outcome {
                    PhaseFactor::MINUS_ONE
                } else {
                    PhaseFactor::PLUS_ONE
                };
                self._resolve_superposition(&self.vec_s.to_owned(), &vec_t, delta)?;
                Ok(false)
            }
        }
    }
}
