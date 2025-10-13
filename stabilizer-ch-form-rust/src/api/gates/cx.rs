use crate::{StabilizerCHForm, error::Result};

impl StabilizerCHForm {
    /// Applies the CNOT (CX) gate with control qubit at index `control` and target qubit at index `target`.
    ///
    /// Time complexity: O(n)
    ///
    /// See around eq.(49) of arXiv:1808.00128 for details.
    pub fn apply_cx(&mut self, control: usize, target: usize) -> Result<()> {
        self._left_multiply_cx(control, target)
    }
}
