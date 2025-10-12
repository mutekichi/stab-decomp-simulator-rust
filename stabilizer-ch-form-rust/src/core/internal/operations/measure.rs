use crate::StabilizerCHForm;
use crate::error::ChFormError;

use crate::core::internal::types::measurement::QubitState;
use rand::random;

impl StabilizerCHForm {
    pub(crate) fn _measure(&mut self, qarg: usize) -> Result<bool, ChFormError> {
        if qarg >= self.n {
            return Err(ChFormError::QubitIndexOutOfBounds(qarg, self.n));
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
