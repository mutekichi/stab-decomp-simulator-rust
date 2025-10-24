use crate::{StabilizerCHForm, error::Result};

impl StabilizerCHForm {
    /// Applies the SWAP gate between the qubits at indices `qarg1` and `qarg2`.
    ///
    /// Time complexity: O(n)
    pub fn apply_swap(&mut self, qarg1: usize, qarg2: usize) -> Result<()> {
        self._left_multiply_swap(qarg1, qarg2)
    }
}
