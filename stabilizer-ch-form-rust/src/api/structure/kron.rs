use crate::{StabilizerCHForm, error::ChFormError};

impl StabilizerCHForm {
    /// Computes the tensor product of this state with another.
    ///
    /// Returns: |self> ⊗ |other>
    pub fn kron(&self, other: &StabilizerCHForm) -> Result<StabilizerCHForm, ChFormError> {
        self._kron(other)
    }
}
