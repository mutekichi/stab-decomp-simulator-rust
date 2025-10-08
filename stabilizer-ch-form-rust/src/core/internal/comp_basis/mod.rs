use crate::StabilizerCHForm;
use crate::core::internal::types::Scalar;

impl StabilizerCHForm {
    /// Computes the amplitude <0...0|φ> for the stabilizer state φ.
    ///
    /// NOTE: The amplitude includes the phase factor, but not ω.
    /// See around eq.(55) of arXiv:1808.00128 for details.
    pub(crate) fn _amplitude_at_zero(&self) -> Scalar {
        for j in 0..self.n {
            if !self.vec_v[j] && self.vec_s[j] {
                return Scalar::Zero;
            }
        }
        let weight_v = self.vec_v.iter().filter(|&&x| x).count();

        Scalar::NonZero {
            phase: self.phase_factor,
            r: weight_v,
        }
    }

    /// Computes the amplitude <s|φ> for the stabilizer state φ and bitstring state s.
    ///
    /// NOTE: The amplitude includes the phase factor, but not the global phase ω.
    /// NOTE: This implementation might be inefficient.
    pub(crate) fn _amplitude_at_computational_basis(&self, s: &ndarray::Array1<bool>) -> Scalar {
        if s.len() != self.n_qubits() {
            panic!("Input bitstring length does not match number of qubits.");
        }

        let mut ch_form_clone = self.clone();
        for (i, &bit) in s.iter().enumerate() {
            if bit {
                ch_form_clone._left_multiply_x(i);
            }
        }

        ch_form_clone._amplitude_at_zero()
    }
}
