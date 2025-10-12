use crate::{StabilizerCHForm, error::ChFormError};

impl StabilizerCHForm {
    pub(crate) fn _left_multiply_swap(
        &mut self,
        qarg1: usize,
        qarg2: usize,
    ) -> Result<(), ChFormError> {
        if qarg1 >= self.n {
            return Err(ChFormError::QubitIndexOutOfBounds(qarg1, self.n));
        }
        if qarg2 >= self.n {
            return Err(ChFormError::QubitIndexOutOfBounds(qarg2, self.n));
        }
        if qarg1 == qarg2 {
            return Err(ChFormError::DuplicateQubitIndices(qarg1));
        }

        let perm: Vec<usize> = (0..self.n)
            .map(|x| match x {
                _ if x == qarg1 => qarg2,
                _ if x == qarg2 => qarg1,
                _ => x,
            })
            .collect();
        self._permute(&perm);

        Ok(())
    }
}
