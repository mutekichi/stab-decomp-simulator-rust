use crate::{
    StabilizerCHForm,
    error::{Error, Result},
    form::types::PhaseFactor,
};

impl StabilizerCHForm {
    pub(crate) fn right_multiply_s(&mut self, qarg: usize) -> Result<()> {
        if qarg >= self.n {
            return Err(Error::QubitIndexOutOfBounds(qarg, self.n));
        }

        let f_col = self.mat_f.column(qarg);
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
