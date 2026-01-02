use crate::StabilizerCHForm;
use crate::error::{Error, Result};

use crate::form::types::QubitState;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

impl StabilizerCHForm {
    /// Measures the specified qubit in the computational basis.
    ///
    /// ## Arguments
    /// * `qarg` - The index of the qubit to measure.
    ///
    /// ## Returns
    /// A [`Result`] containing the measurement outcome: `false` for `|0>`, `true` for `|1>`.
    pub fn measure(&mut self, qarg: usize, seed: Option<[u8; 32]>) -> Result<bool> {
        if qarg >= self.n {
            return Err(Error::QubitIndexOutOfBounds(qarg, self.n));
        }

        let z_basis_state = self.get_qubit_state(qarg)?;
        match z_basis_state {
            QubitState::Determined(state) => Ok(state),
            QubitState::Superposition => {
                // Randomly collapse the qubit to |0> or |1>
                let mut rng = match seed {
                    Some(s) => StdRng::from_seed(s),
                    None => StdRng::from_entropy(),
                };
                let outcome = rng.r#gen::<bool>();
                self.project(qarg, outcome)?;
                Ok(outcome)
            }
        }
    }
}
