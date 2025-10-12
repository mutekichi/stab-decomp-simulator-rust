use crate::{StabilizerCHForm, error::ChFormError};

impl StabilizerCHForm {
    /// Applies the Pauli-Y gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(n)
    pub fn apply_y(&mut self, qarg: usize) -> Result<(), ChFormError> {
        self._left_multiply_y(qarg)
    }
}
