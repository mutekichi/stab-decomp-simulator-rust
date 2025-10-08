use crate::core::StabilizerCHForm;

impl StabilizerCHForm {
    pub(crate) fn _left_multiply_cz(&mut self, q1: usize, q2: usize) {
        if q1 >= self.n || q2 >= self.n {
            panic!("Qubit index out of bounds.");
        }
        if q1 == q2 {
            return;
        }

        let g1_row = self.mat_g.row(q1).to_owned();
        let g2_row = self.mat_g.row(q2).to_owned();

        let mut m1_row = self.mat_m.row_mut(q1);
        m1_row ^= &g2_row;

        let mut m2_row = self.mat_m.row_mut(q2);
        m2_row ^= &g1_row;
    }
}
