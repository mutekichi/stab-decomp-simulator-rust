use crate::{
    core::{PhaseFactor, StabilizerCHForm},
    error::{Error, Result},
};

impl StabilizerCHForm {
    pub(crate) fn _right_multiply_s(&mut self, qarg: usize) -> Result<()> {
        if qarg >= self.n {
            return Err(Error::QubitIndexOutOfBounds(qarg, self.n));
        }

        let f_col = self.mat_f.column(qarg).to_owned();
        let mut m_col = self.mat_m.column_mut(qarg);
        m_col ^= &f_col;

        for p in 0..self.n {
            if f_col[p] {
                self.gamma[p] *= PhaseFactor::MINUS_I;
            }
        }

        Ok(())
    }
}
