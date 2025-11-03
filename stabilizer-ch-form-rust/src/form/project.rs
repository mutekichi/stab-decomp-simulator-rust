use crate::{
    StabilizerCHForm,
    error::{Error, Result},
    form::types::{PhaseFactor, QubitState},
};

impl StabilizerCHForm {
    pub(crate) fn _project(&mut self, qarg: usize, outcome: bool) -> Result<bool> {
        if qarg >= self.n {
            return Err(Error::QubitIndexOutOfBounds(qarg, self.n));
        }

        let qubit_state = self._get_qubit_state(qarg)?;
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
