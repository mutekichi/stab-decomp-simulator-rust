use crate::{
    core::StabilizerCHForm,
    error::{Error, Result},
};

impl StabilizerCHForm {
    pub(crate) fn _left_multiply_z(&mut self, qarg: usize) -> Result<()> {
        if qarg >= self.n {
            return Err(Error::QubitIndexOutOfBounds(qarg, self.n));
        }
        self.gamma[qarg].flip_sign();

        Ok(())
    }
}
