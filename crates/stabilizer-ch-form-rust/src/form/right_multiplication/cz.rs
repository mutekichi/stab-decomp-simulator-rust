use crate::{
    StabilizerCHForm,
    error::{Error, Result},
    form::types::PhaseFactor,
};

impl StabilizerCHForm {
    pub(crate) fn right_multiply_cz(&mut self, q1: usize, q2: usize) -> Result<()> {
        if q1 >= self.n {
            return Err(Error::QubitIndexOutOfBounds(q1, self.n));
        }
        if q2 >= self.n {
            return Err(Error::QubitIndexOutOfBounds(q2, self.n));
        }
        if q1 == q2 {
            return Err(Error::DuplicateQubitIndices(q1));
        }

        let f1_col = self.mat_f.column(q1);
        let f2_col = self.mat_f.column(q2);

        let mut m1_col = self.mat_m.column_mut(q1);
        m1_col ^= &f2_col;

        let mut m2_col = self.mat_m.column_mut(q2);
        m2_col ^= &f1_col;

        for p in 0..self.n {
            if f1_col[p] && f2_col[p] {
                self.gamma[p] *= PhaseFactor::MINUS_ONE;
            }
        }

        Ok(())
    }
}
