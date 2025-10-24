use crate::{StabilizerCHForm, error::Result};
impl StabilizerCHForm {
    /// Applies the CZ gate between qubits at indices `qarg1` and `qarg2`.
    ///
    /// Time complexity: O(n)
    ///
    /// See around eq.(50) of arXiv:1808.00128 for details.
    pub fn apply_cz(&mut self, qarg1: usize, qarg2: usize) -> Result<()> {
        self._left_multiply_cz(qarg1, qarg2)
    }
}
