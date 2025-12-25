use crate::{
    StabilizerCHForm,
    error::{Error, Result},
};

impl StabilizerCHForm {
    pub(crate) fn right_multiply_cx(&mut self, control: usize, target: usize) -> Result<()> {
        if control >= self.n {
            return Err(Error::QubitIndexOutOfBounds(control, self.n));
        }
        if target >= self.n {
            return Err(Error::QubitIndexOutOfBounds(target, self.n));
        }
        if control == target {
            return Err(Error::DuplicateQubitIndices(control));
        }

        // mat_g[:, target] ^= mat_g[:, control]
        Self::xor_columns(&mut self.mat_g, control, target);
        // mat_f[:, control] ^= mat_f[:, target]
        Self::xor_columns(&mut self.mat_f, target, control);
        // mat_m[:, control] ^= mat_m[:, target]
        Self::xor_columns(&mut self.mat_m, control, target);

        Ok(())
    }
}
