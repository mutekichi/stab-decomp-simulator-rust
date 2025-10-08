use crate::StabilizerCHForm;

impl StabilizerCHForm {
    /// Computes the tensor product of this state with another.
    ///
    /// Returns: |self> ⊗ |other>
    pub fn kron(&self, other: &StabilizerCHForm) -> StabilizerCHForm {
        self._kron(other)
    }
}
