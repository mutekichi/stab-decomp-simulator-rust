use crate::{StabilizerCHForm, error::ChFormError};

impl StabilizerCHForm {
    pub fn measure(&mut self, qarg: usize) -> Result<bool, ChFormError> {
        self._measure(qarg)
    }
}
