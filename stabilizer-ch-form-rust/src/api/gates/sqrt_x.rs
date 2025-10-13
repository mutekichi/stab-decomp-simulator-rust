use crate::{StabilizerCHForm, error::Result};

impl StabilizerCHForm {
    /// Applies the √X gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(n^2)
    pub fn apply_sqrt_x(&mut self, qarg: usize) -> Result<()> {
        self._left_multiply_sqrt_x(qarg)
    }

    /// Applies the adjoint of the √X gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(n^2)
    pub fn apply_sqrt_xdg(&mut self, qarg: usize) -> Result<()> {
        self._left_multiply_sqrt_xdg(qarg)
    }
}
