use crate::core::{PhaseFactor, StabilizerCHForm};

impl StabilizerCHForm {
    pub(crate) fn _right_multiply_cz(&mut self, q1: usize, q2: usize) {
        if q1 >= self.n || q2 >= self.n {
            panic!("Qubit index out of bounds.");
        }
        if q1 == q2 {
            return;
        }

        let f1_col = self.mat_f.column(q1).to_owned();
        let f2_col = self.mat_f.column(q2).to_owned();

        let mut m1_col = self.mat_m.column_mut(q1);
        m1_col ^= &f2_col;

        let mut m2_col = self.mat_m.column_mut(q2);
        m2_col ^= &f1_col;

        for p in 0..self.n {
            if f1_col[p] && f2_col[p] {
                self.gamma[p] *= PhaseFactor::MINUS_ONE;
            }
        }
    }
}
