use crate::{core::StabilizerCHForm, error::ChFormError};

impl StabilizerCHForm {
    pub(crate) fn _left_multiply_z(&mut self, qarg: usize) -> Result<(), ChFormError> {
        if qarg >= self.n {
            return Err(ChFormError::QubitIndexOutOfBounds(qarg, self.n));
        }
        self.gamma[qarg].flip_sign();

        Ok(())
    }
}
