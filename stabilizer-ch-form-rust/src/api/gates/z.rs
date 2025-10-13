use crate::{StabilizerCHForm, error::Result};

impl StabilizerCHForm {
    /// Applies the Pauli-Z gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(1)
    pub fn apply_z(&mut self, qarg: usize) -> Result<()> {
        self._left_multiply_z(qarg)
    }
}
