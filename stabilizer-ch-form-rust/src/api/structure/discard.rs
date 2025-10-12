use crate::{StabilizerCHForm, error::ChFormError};

impl StabilizerCHForm {
    /// Discards the specified qubit from the state.
    ///
    /// NOTE: This function assumes that the qubit `qarg` has already been
    /// projected onto the |0> state.
    ///
    /// # Errors
    //// Returns an `ChFormError` if the qubit index is out of bounds. Note that
    /// this function does not check if the qubit is properly projected onto |0>.
    pub fn discard(&mut self, qarg: usize) -> Result<(), ChFormError> {
        self._discard(qarg)
    }

    /// Returns a new StabilizerCHForm with the specified qubit discarded.
    ///
    /// NOTE: This function assumes that the qubit `qarg` has already been
    /// projected onto the |0> state.
    pub fn discarded(&self, qarg: usize) -> Result<StabilizerCHForm, ChFormError> {
        let mut self_clone = self.clone();
        self_clone.discard(qarg)?;
        Ok(self_clone)
    }
}
