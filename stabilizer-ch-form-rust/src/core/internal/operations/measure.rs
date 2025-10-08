use crate::StabilizerCHForm;

use crate::core::internal::types::measurement::QubitState;
use rand::random;

impl StabilizerCHForm {
    pub(crate) fn _measure(&mut self, qarg: usize) -> bool {
        if qarg >= self.n {
            panic!("Qubit index out of bounds.");
        }

        let z_basis_state = self._get_qubit_state(qarg);
        match z_basis_state {
            QubitState::Determined(state) => state,
            QubitState::Superposition => {
                // Randomly collapse the qubit to |0> or |1>
                let outcome = random::<bool>();
                self._project(qarg, outcome)
                    .expect("Projection failed during measurement.");
                outcome
            }
        }
    }
}
