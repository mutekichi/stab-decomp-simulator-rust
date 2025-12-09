use crate::{
    StabilizerCHForm,
    error::{Error, Result},
};

impl StabilizerCHForm {
    // Applies the Pauli-X gate to the qubit at index `qarg`.
    //
    // Time complexity: O(n)
    //
    // See around eq.(48) of arXiv:1808.00128 for details.
    pub(crate) fn _left_multiply_x(&mut self, qarg: usize) -> Result<()> {
        if qarg >= self.n {
            return Err(Error::QubitIndexOutOfBounds(qarg, self.n));
        }
        let f_row = self.mat_f.row(qarg);
        let m_row = self.mat_m.row(qarg);

        // calculate beta appearing in eq.(49) of arXiv:1808.00128
        let beta = m_row
            .iter()
            .zip(f_row.iter())
            .zip(self.vec_v.iter())
            .zip(self.vec_s.iter())
            .fold(false, |acc, (((&m, &f), &v), &s)| {
                let t1 = m & !v & s;
                let t23 = f & v & (m ^ s);
                acc ^ t1 ^ t23
            });

        if beta {
            self.phase_factor = self.phase_factor.flipped();
        }

        // calculate u appearing in eq.(48) of arXiv:1808.00128
        self.vec_s
            .iter_mut()
            .zip(self.vec_v.iter())
            .zip(f_row.iter())
            .zip(m_row.iter())
            .for_each(|(((s, &v), &f), &m)| {
                *s ^= (f & !v) ^ (m & v);
            });

        self.phase_factor *= self.gamma[qarg];

        Ok(())
    }
}
