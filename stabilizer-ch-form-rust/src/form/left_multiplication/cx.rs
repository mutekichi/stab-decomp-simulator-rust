use crate::{
    StabilizerCHForm,
    error::{Error, Result},
    form::types::PhaseFactor,
};

impl StabilizerCHForm {
    pub(crate) fn _left_multiply_cx(&mut self, control: usize, target: usize) -> Result<()> {
        if control >= self.n {
            return Err(Error::QubitIndexOutOfBounds(control, self.n));
        }
        if target >= self.n {
            return Err(Error::QubitIndexOutOfBounds(target, self.n));
        }
        if control == target {
            return Err(Error::DuplicateQubitIndices(control));
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
        // NOTE: `.to_owned()` is used to create an explicit copy.
        // This avoids a borrow checker error, which disallows simultaneous
        // immutable (`.row(control)`) and mutable (`.row_mut(target)`)
        // borrows of `self.mat_g`, even though our logic guarantees
        // `control != target` (no aliasing).
        // This prioritizes safety over a minor copy overhead.
        let g_control_row = self.mat_g.row(control).to_owned();
        let mut g_target_row = self.mat_g.row_mut(target);
        g_target_row ^= &g_control_row;

        let f_target_row = self.mat_f.row(target).to_owned();
        let mut f_control_row = self.mat_f.row_mut(control);
        f_control_row ^= &f_target_row;

        let m_target_row = self.mat_m.row(target).to_owned();
        let mut m_control_row = self.mat_m.row_mut(control);
        m_control_row ^= &m_target_row;

        /*
        // --- Unsafe alternative (for reference) ---
        // An `unsafe` block could avoid the `.to_owned()` copy by using
        // raw pointers, trusting the `control != target` check.
        // This would be faster but sacrifices compile-time safety.
        unsafe {
            let self_ptr: *mut Self = self;
            let g_control_row = (*self_ptr).mat_g.row(control);
            let mut g_target_row = (*self_ptr).mat_g.row_mut(target);
            g_target_row ^= &g_control_row;

            let f_target_row = (*self_ptr).mat_f.row(target);
            let mut f_control_row = (*self_ptr).mat_f.row_mut(control);
            f_control_row ^= &f_target_row;

            let m_target_row = (*self_ptr).mat_m.row(target);
            let mut m_control_row = (*self_ptr).mat_m.row_mut(control);
            m_control_row ^= &m_target_row;
        }
        */

        Ok(())
    }
}
