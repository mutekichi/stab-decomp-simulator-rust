use crate::{
    StabilizerCHForm,
    error::{Error, Result},
    form::types::PhaseFactor,
};

impl StabilizerCHForm {
    /// Left-multiplies the state by an S gate on qubit `qarg`.
    ///
    /// Time complexity: O(n)
    ///  
    /// See around the end of Proposition 4 of arXiv:1808.00128 for details.
    pub(crate) fn left_multiply_s(&mut self, qarg: usize) -> Result<()> {
        if qarg >= self.n {
            return Err(Error::QubitIndexOutOfBounds(qarg, self.n));
        }
        let g_row = self.mat_g.row(qarg);
        let mut m_row = self.mat_m.row_mut(qarg);
        m_row ^= &g_row;
        self.gamma[qarg] *= PhaseFactor::MINUS_I;
        Ok(())
    }

    pub(crate) fn left_multiply_sdg(&mut self, qarg: usize) -> Result<()> {
        if qarg >= self.n {
            return Err(Error::QubitIndexOutOfBounds(qarg, self.n));
        }
        let g_row = self.mat_g.row(qarg);
        let mut m_row = self.mat_m.row_mut(qarg);
        m_row ^= &g_row;
        self.gamma[qarg] *= PhaseFactor::PLUS_I;
        Ok(())
    }
}
