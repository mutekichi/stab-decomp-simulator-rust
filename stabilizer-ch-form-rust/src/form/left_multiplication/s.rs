use crate::{
    StabilizerCHForm,
    error::{Error, Result},
    form::types::PhaseFactor,
};

impl StabilizerCHForm {
    pub(crate) fn _left_multiply_s(&mut self, qarg: usize) -> Result<()> {
        if qarg >= self.n {
            return Err(Error::QubitIndexOutOfBounds(qarg, self.n));
        }
        let g_row = self.mat_g.row(qarg).to_owned();
        let mut m_row = self.mat_m.row_mut(qarg);
        m_row ^= &g_row;
        self.gamma[qarg] *= PhaseFactor::MINUS_I;
        Ok(())
    }

    pub(crate) fn _left_multiply_sdg(&mut self, qarg: usize) -> Result<()> {
        if qarg >= self.n {
            return Err(Error::QubitIndexOutOfBounds(qarg, self.n));
        }
        let g_row = self.mat_g.row(qarg).to_owned();
        let mut m_row = self.mat_m.row_mut(qarg);
        m_row ^= &g_row;
        self.gamma[qarg] *= PhaseFactor::PLUS_I;
        Ok(())
    }
}
