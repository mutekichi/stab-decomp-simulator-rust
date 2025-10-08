use crate::core::StabilizerCHForm;

impl StabilizerCHForm {
    pub(crate) fn _right_multiply_cx(&mut self, control: usize, target: usize) {
        if control >= self.n || target >= self.n {
            panic!("Qubit index out of bounds.");
        }
        if control == target {
            return;
        }

        let g_target_col = self.mat_g.column(target).to_owned();
        let mut g_control_col = self.mat_g.column_mut(control);
        g_control_col ^= &g_target_col;

        let f_control_col = self.mat_f.column(control).to_owned();
        let mut f_target_col = self.mat_f.column_mut(target);
        f_target_col ^= &f_control_col;

        let m_target_col = self.mat_m.column(target).to_owned();
        let mut m_control_col = self.mat_m.column_mut(control);
        m_control_col ^= &m_target_col;
    }
}
