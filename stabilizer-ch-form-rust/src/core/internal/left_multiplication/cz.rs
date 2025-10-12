use crate::{core::StabilizerCHForm, error::ChFormError};

impl StabilizerCHForm {
    pub(crate) fn _left_multiply_cz(&mut self, q1: usize, q2: usize) -> Result<(), ChFormError> {
        if q1 >= self.n {
            return Err(ChFormError::QubitIndexOutOfBounds(q1, self.n));
        }
        if q2 >= self.n {
            return Err(ChFormError::QubitIndexOutOfBounds(q2, self.n));
        }
        if q1 == q2 {
            return Err(ChFormError::DuplicateQubitIndices(q1));
        }

        let g1_row = self.mat_g.row(q1).to_owned();
        let g2_row = self.mat_g.row(q2).to_owned();

        let mut m1_row = self.mat_m.row_mut(q1);
        m1_row ^= &g2_row;

        let mut m2_row = self.mat_m.row_mut(q2);
        m2_row ^= &g1_row;

        Ok(())
    }
}
