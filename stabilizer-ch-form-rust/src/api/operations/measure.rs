use crate::{StabilizerCHForm, error::Result};

impl StabilizerCHForm {
    pub fn measure(&mut self, qarg: usize) -> Result<bool> {
        self._measure(qarg)
    }
}
