use crate::{StabilizerCHForm, error::ChFormError};

impl StabilizerCHForm {
    /// Returns a new StabilizerCHForm with the qubits permuted.
    ///
    /// # Arguments
    ///
    /// * `axes` - A slice representing the new order of qubits. For `n` qubits,
    ///   this must be a permutation of `[0, 1, ..., n-1]`.
    pub fn permuted(&self, axes: &[usize]) -> Result<Self, ChFormError> {
        self._permuted(axes)
    }

    /// Permutes the qubits of the state in-place.
    ///
    /// # Arguments
    ///
    /// * `axes` - A slice representing the new order of qubits. For `n` qubits,
    ///   this must be a permutation of `[0, 1, ..., n-1]`.
    ///
    /// # Panics
    ///
    /// Panics if the length of `axes` is not equal to the number of qubits.
    pub fn permute(&mut self, axes: &[usize]) -> Result<(), ChFormError> {
        self._permute(axes)
    }
}
