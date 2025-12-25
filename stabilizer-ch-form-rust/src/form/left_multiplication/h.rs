use crate::{
    StabilizerCHForm,
    error::{Error, Result},
};
use ndarray::Array1;

impl StabilizerCHForm {
    /// Left-multiplies the state by a Hadamard gate on qubit `qarg`.    
    ///
    /// Time complexity: O(n^2)
    ///
    /// See around Proposition 4. of arXiv:1808.00128 for details.
    pub(crate) fn left_multiply_h(&mut self, qarg: usize) -> Result<()> {
        if qarg >= self.n {
            return Err(Error::QubitIndexOutOfBounds(qarg, self.n));
        }
        let (vec_t, vec_u, alpha, beta) = self.prepare_h_superposition_args(qarg);
        let delta = if alpha ^ beta {
            self.gamma[qarg].flipped()
        } else {
            self.gamma[qarg]
        };
        if alpha {
            self.phase_factor.flip_sign();
        }
        self._resolve_superposition(&vec_t, &vec_u, delta)?;

        Ok(())
    }

    /// Prepares vec_t, vec_u, alpha, beta for applying H to qubit `qarg`.
    pub(crate) fn prepare_h_superposition_args(
        &self,
        qarg: usize,
    ) -> (Array1<bool>, Array1<bool>, bool, bool) {
        let g_row = self.mat_g.row(qarg);
        let f_row = self.mat_f.row(qarg);
        let m_row = self.mat_m.row(qarg);

        let not_vec_v = !&self.vec_v;

        // eq. (48) of arXiv:1808.00128
        let vec_t = &self.vec_s ^ (&g_row & &self.vec_v);
        let vec_u = &self.vec_s ^ (&f_row & &not_vec_v) ^ (&m_row & &self.vec_v);

        // eq. (49) of arXiv:1808.00128
        let alpha = g_row
            .iter()
            .zip(&self.vec_v)
            .zip(&self.vec_s)
            .fold(false, |acc, ((&g, &v), &s)| acc ^ (g && !v && s));

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

        (vec_t, vec_u, alpha, beta)
    }
}
