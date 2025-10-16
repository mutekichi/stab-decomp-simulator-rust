use crate::{StabilizerCHForm, error::Result};

impl StabilizerCHForm {
    /// Computes the tensor product of this state with another.
    ///
    /// Returns: |self> ⊗ |other>
    pub fn kron(&self, other: &StabilizerCHForm) -> Result<StabilizerCHForm> {
        self._kron(other)
    }
}
