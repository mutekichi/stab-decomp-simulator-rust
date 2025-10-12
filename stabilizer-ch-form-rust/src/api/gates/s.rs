use crate::{StabilizerCHForm, error::ChFormError};

impl StabilizerCHForm {
    /// Applies the Phase (S) gate to the qubit at index `qarg`.
    ///     
    /// Time complexity: O(n)
    ///
    /// See around the end of Proposition 4 of arXiv:1808.00128 for details.
    pub fn apply_s(&mut self, qarg: usize) -> Result<(), ChFormError> {
        self._left_multiply_s(qarg)
    }

    /// Applies the adjoint Phase (S†) gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(n)
    pub fn apply_sdg(&mut self, qarg: usize) -> Result<(), ChFormError> {
        self._left_multiply_sdg(qarg)
    }
}
