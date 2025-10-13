use crate::{StabilizerCHForm, error::Result};

impl StabilizerCHForm {
    /// Applies the Pauli-X gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(n)
    ///
    /// See around eq.(48) of arXiv:1808.00128 for details.
    pub fn apply_x(&mut self, qarg: usize) -> Result<()> {
        self._left_multiply_x(qarg)
    }
}
