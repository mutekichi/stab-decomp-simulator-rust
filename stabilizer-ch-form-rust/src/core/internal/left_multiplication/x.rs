use crate::{StabilizerCHForm, error::ChFormError};
use ndarray::Array1;

impl StabilizerCHForm {
    // Applies the Pauli-X gate to the qubit at index `qarg`.
    //
    // Time complexity: O(n)
    //
    // See around eq.(48) of arXiv:1808.00128 for details.
    pub(crate) fn _left_multiply_x(&mut self, qarg: usize) -> Result<(), ChFormError> {
        if qarg >= self.n {
            return Err(ChFormError::QubitIndexOutOfBounds(qarg, self.n));
        }
        // calculate u appearing in eq.(48) of arXiv:1808.00128 :
        // $$
        // u_j = s_j \oplus (F_{p,j} \land \lnot v_j) \oplus (M_{p,j} \land v_j)
        // $$
        let f_row = self.mat_f.row(qarg);
        let m_row = self.mat_m.row(qarg);

        let u: Array1<bool> = self
            .vec_s
            .iter()
            .zip(self.vec_v.iter())
            .zip(f_row.iter())
            .zip(m_row.iter())
            .map(|(((&s, &v), &f), &m)| s ^ (f & !v) ^ (m & v))
            .collect();

        let term1 = m_row
            .iter()
            .zip(&self.vec_v)
            .zip(&self.vec_s)
            .filter(|&((&m, &v), &s)| m && !v && s)
            .count();
        let term2 = f_row
            .iter()
            .zip(&self.vec_v)
            .zip(m_row)
            .filter(|&((&f, &v), &m)| f && v && m)
            .count();
        let term3 = f_row
            .iter()
            .zip(&self.vec_v)
            .zip(&self.vec_s)
            .filter(|&((&f, &v), &s)| f && v && s)
            .count();
        let beta = (term1 + term2 + term3) % 2;

        if beta == 1 {
            self.phase_factor = self.phase_factor.flipped();
        }

        self.phase_factor *= self.gamma[qarg];

        self.vec_s = u;

        Ok(())
    }
}
