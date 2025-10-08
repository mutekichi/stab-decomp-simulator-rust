use crate::core::{PhaseFactor, StabilizerCHForm};

impl StabilizerCHForm {
    pub(crate) fn _left_multiply_cx(&mut self, control: usize, target: usize) {
        if control >= self.n || target >= self.n {
            panic!("Qubit index out of bounds.");
        }
        if control == target {
            return;
        }

        // 1. Update gamma (must be done before matrix updates)
        let m_control_row = self.mat_m.row(control);
        let f_target_row = self.mat_f.row(target);
        let dot_product_is_one = m_control_row
            .iter()
            .zip(f_target_row.iter())
            .fold(false, |acc, (&m, &f)| acc ^ (m & f));

        if dot_product_is_one {
            let gamma_c = self.gamma[control];
            let gamma_t = self.gamma[target];
            self.gamma[control] = gamma_c * gamma_t * PhaseFactor::MINUS_ONE;
        } else {
            let gamma_c = self.gamma[control];
            let gamma_t = self.gamma[target];
            self.gamma[control] = gamma_c * gamma_t;
        }

        // 2. Update matrices
        let g_control_row = self.mat_g.row(control).to_owned();
        let mut g_target_row = self.mat_g.row_mut(target);
        g_target_row ^= &g_control_row;

        let f_target_row = self.mat_f.row(target).to_owned();
        let mut f_control_row = self.mat_f.row_mut(control);
        f_control_row ^= &f_target_row;

        let m_target_row = self.mat_m.row(target).to_owned();
        let mut m_control_row = self.mat_m.row_mut(control);
        m_control_row ^= &m_target_row;
    }
}
