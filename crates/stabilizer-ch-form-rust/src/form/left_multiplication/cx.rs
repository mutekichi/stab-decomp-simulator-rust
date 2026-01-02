use crate::{
    StabilizerCHForm,
    error::{Error, Result},
    form::types::PhaseFactor,
};

impl StabilizerCHForm {
    /// Left-multiplies the state by a CNOT (CX) gate with control qubit `control` and target qubit `target`.
    ///
    /// See around eq.(49) of arXiv:1808.00128 for details.
    ///
    /// Time complexity: O(n)
    pub(crate) fn left_multiply_cx(&mut self, control: usize, target: usize) -> Result<()> {
        if control >= self.n {
            return Err(Error::QubitIndexOutOfBounds(control, self.n));
        }
        if target >= self.n {
            return Err(Error::QubitIndexOutOfBounds(target, self.n));
        }
        if control == target {
            return Err(Error::DuplicateQubitIndices(control));
        }

        // Update gamma
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

        // Update matrices
        // mat_g[target, :] ^= mat_g[control, :]
        Self::xor_rows(&mut self.mat_g, target, control);
        // mat_f[control, :] ^= mat_f[target, :]
        Self::xor_rows(&mut self.mat_f, control, target);
        // mat_m[control, :] ^= mat_m[target, :]
        Self::xor_rows(&mut self.mat_m, control, target);

        Ok(())
    }
}
