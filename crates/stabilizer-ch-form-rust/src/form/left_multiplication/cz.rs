use crate::{
    StabilizerCHForm,
    error::{Error, Result},
};

impl StabilizerCHForm {
    /// Left-multiplies the state by a CZ gate between qubits `q1` and `q2`.
    ///
    /// Time complexity: O(n)
    ///
    /// See around eq.(50) of arXiv:1808.00128 for details.
    pub(crate) fn left_multiply_cz(&mut self, q1: usize, q2: usize) -> Result<()> {
        if q1 >= self.n {
            return Err(Error::QubitIndexOutOfBounds(q1, self.n));
        }
        if q2 >= self.n {
            return Err(Error::QubitIndexOutOfBounds(q2, self.n));
        }
        if q1 == q2 {
            return Err(Error::DuplicateQubitIndices(q1));
        }

        {
            let g2_row = self.mat_g.row(q2);
            let mut m1_row = self.mat_m.row_mut(q1);
            m1_row ^= &g2_row;
        }

        {
            let g1_row = self.mat_g.row(q1);
            let mut m2_row = self.mat_m.row_mut(q2);
            m2_row ^= &g1_row;
        }
        Ok(())
    }
}
