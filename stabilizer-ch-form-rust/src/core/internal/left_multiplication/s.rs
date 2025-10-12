use crate::{
    core::{StabilizerCHForm, internal::types::PhaseFactor},
    error::ChFormError,
};

impl StabilizerCHForm {
    pub(crate) fn _left_multiply_s(&mut self, qarg: usize) -> Result<(), ChFormError> {
        if qarg >= self.n {
            return Err(ChFormError::QubitIndexOutOfBounds(qarg, self.n));
        }
        let g_row = self.mat_g.row(qarg).to_owned();
        let mut m_row = self.mat_m.row_mut(qarg);
        m_row ^= &g_row;
        self.gamma[qarg] *= PhaseFactor::MINUS_I;
        Ok(())
    }

    pub(crate) fn _left_multiply_sdg(&mut self, qarg: usize) -> Result<(), ChFormError> {
        if qarg >= self.n {
            return Err(ChFormError::QubitIndexOutOfBounds(qarg, self.n));
        }
        let g_row = self.mat_g.row(qarg).to_owned();
        let mut m_row = self.mat_m.row_mut(qarg);
        m_row ^= &g_row;
        self.gamma[qarg] *= PhaseFactor::PLUS_I;
        Ok(())
    }
}
