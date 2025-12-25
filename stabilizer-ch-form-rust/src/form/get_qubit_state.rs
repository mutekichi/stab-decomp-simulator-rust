use crate::StabilizerCHForm;
use crate::error::{Error, Result};
use crate::form::types::QubitState;

impl StabilizerCHForm {
    pub(crate) fn get_qubit_state(&self, qarg: usize) -> Result<QubitState> {
        if qarg >= self.n {
            return Err(Error::QubitIndexOutOfBounds(qarg, self.n));
        }

        let g_row = self.mat_g.row(qarg);

        // Check for superposition: the state is a superposition if any v[i] is true
        // where the corresponding G_row[i] is also true.
        let is_determined = !g_row.iter().zip(&self.vec_v).any(|(&g, &v)| g && v);

        if is_determined {
            // If determined, the value is the parity of the inner product
            // of the g_row and the s vector.
            let value = g_row.iter().zip(&self.vec_s).fold(
                0,
                |acc, (&g, &s)| {
                    if g && s { acc + 1 } else { acc }
                },
            ) % 2
                == 1;
            Ok(QubitState::Determined(value))
        } else {
            Ok(QubitState::Superposition)
        }
    }
}
