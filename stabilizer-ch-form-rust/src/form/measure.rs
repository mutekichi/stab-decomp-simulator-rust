use crate::StabilizerCHForm;
use crate::error::{Error, Result};

use crate::form::types::QubitState;
use rand::random;

impl StabilizerCHForm {
    pub(crate) fn _measure(&mut self, qarg: usize) -> Result<bool> {
        if qarg >= self.n {
            return Err(Error::QubitIndexOutOfBounds(qarg, self.n));
        }

        let z_basis_state = self._get_qubit_state(qarg)?;
        match z_basis_state {
            QubitState::Determined(state) => Ok(state),
            QubitState::Superposition => {
                // Randomly collapse the qubit to |0> or |1>
                let outcome = random::<bool>();
                self._project(qarg, outcome)?;
                Ok(outcome)
            }
        }
    }
}
