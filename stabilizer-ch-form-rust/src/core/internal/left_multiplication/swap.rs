use crate::{StabilizerCHForm, error::{Error, Result}};

impl StabilizerCHForm {
    pub(crate) fn _left_multiply_swap(&mut self, qarg1: usize, qarg2: usize) -> Result<()> {
        if qarg1 >= self.n {
            return Err(Error::QubitIndexOutOfBounds(qarg1, self.n));
        }
        if qarg2 >= self.n {
            return Err(Error::QubitIndexOutOfBounds(qarg2, self.n));
        }
        if qarg1 == qarg2 {
            // No-op if the qubits are the same, but consistent with other gate error handling.
            return Err(Error::DuplicateQubitIndices(qarg1));
        }

        self._left_multiply_cx(qarg1, qarg2)?;
        self._left_multiply_cx(qarg2, qarg1)?;
        self._left_multiply_cx(qarg1, qarg2)?;

        Ok(())
    }
}
