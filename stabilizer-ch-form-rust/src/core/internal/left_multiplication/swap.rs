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

        self.mat_g.swap_axes(qarg1, qarg2);
        self.mat_f.swap_axes(qarg1, qarg2);
        self.mat_m.swap_axes(qarg1, qarg2);
        self.gamma.swap(qarg1, qarg2);
        self.vec_v.swap(qarg1, qarg2);
        self.vec_s.swap(qarg1, qarg2);

        Ok(())
    }
}
