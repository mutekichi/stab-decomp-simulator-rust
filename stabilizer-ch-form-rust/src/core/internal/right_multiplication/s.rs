use crate::core::{PhaseFactor, StabilizerCHForm};

impl StabilizerCHForm {
    pub(crate) fn _right_multiply_s(&mut self, qarg: usize) {
        if qarg >= self.n {
            panic!("Qubit index out of bounds.");
        }

        let f_col = self.mat_f.column(qarg).to_owned();
        let mut m_col = self.mat_m.column_mut(qarg);
        m_col ^= &f_col;

        for p in 0..self.n {
            if f_col[p] {
                self.gamma[p] *= PhaseFactor::MINUS_I;
            }
        }
    }
}
