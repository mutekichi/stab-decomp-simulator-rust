use crate::StabilizerCHForm;
use crate::error::Result;
use crate::form::types::PhaseFactor;

impl StabilizerCHForm {
    pub(crate) fn _resolve_superposition(
        &mut self,
        vec_t: &ndarray::Array1<bool>,
        vec_u: &ndarray::Array1<bool>,
        delta: PhaseFactor,
    ) -> Result<()> {
        if vec_t == vec_u {
            self.handle_same_vecs_case(delta, vec_t);
            return Ok(());
        }
        let (diff_indices_0, diff_indices_1) = self.get_differing_indices(vec_t, vec_u);
        let pivot = self.apply_basis_transform_circuit(&diff_indices_0, &diff_indices_1)?;
        // if t_q == 1, (y_q, z_q) = (1,0)
        // if t_q == 0, (y_q, z_q) = (0,1)
        if vec_t[pivot] {
            self.vec_s.assign(vec_u);

            match delta {
                PhaseFactor::PLUS_ONE => {
                    // H(|1> + |0>) = |0>
                    // |1> + |0> = H|0>
                    self.vec_s[pivot] = false;
                    self.vec_v[pivot] = !self.vec_v[pivot];
                }
                PhaseFactor::MINUS_ONE => {
                    // H(|1> - |0>) = -|1>
                    // |1> - |0> = -H|1>
                    self.vec_s[pivot] = true;
                    self.vec_v[pivot] = !self.vec_v[pivot];
                    self.phase_factor.flip_sign();
                }
                PhaseFactor::PLUS_I => {
                    if self.vec_v[pivot] {
                        // H(|1> + i|0>) = e^{iπ/4}SH|0>
                        self.phase_factor *= PhaseFactor::EXP_I_PI_4;
                        self.vec_s[pivot] = false;
                        self.right_multiply_s(pivot)?;
                    } else {
                        // |1> + i|0> = iSH|1>
                        self.vec_s[pivot] = true;
                        self.vec_v[pivot] = true;
                        self.right_multiply_s(pivot)?;
                        self.phase_factor *= PhaseFactor::PLUS_I;
                    }
                }
                PhaseFactor::MINUS_I => {
                    if self.vec_v[pivot] {
                        // H(|1> - i|0>) = e^{-iπ/4}SH|1>
                        self.phase_factor *= PhaseFactor::EXP_I_7PI_4;
                        self.vec_s[pivot] = true;
                        self.right_multiply_s(pivot)?;
                    } else {
                        // |1> - i|0> = -iSH|0>
                        self.vec_s[pivot] = false;
                        self.vec_v[pivot] = true;
                        self.right_multiply_s(pivot)?;
                        self.phase_factor *= PhaseFactor::MINUS_I;
                    }
                }
                _ => unreachable!(),
            }
        } else {
            self.vec_s.assign(vec_t);

            match delta {
                PhaseFactor::PLUS_ONE => {
                    // H(|0> + |1>) = |0>
                    // |0> + |1> = H|0>
                    self.vec_s[pivot] = false;
                    self.vec_v[pivot] = !self.vec_v[pivot];
                }
                PhaseFactor::MINUS_ONE => {
                    // H(|0> - |1>) = -|1>
                    // |0> - |1> = -H|1>
                    self.vec_s[pivot] = true;
                    self.vec_v[pivot] = !self.vec_v[pivot];
                }
                PhaseFactor::PLUS_I => {
                    if self.vec_v[pivot] {
                        // H(|0> + i|1>) = e^{iπ/4}SH|1>
                        self.phase_factor *= PhaseFactor::EXP_I_PI_4;
                        self.vec_s[pivot] = true;
                        self.right_multiply_s(pivot)?;
                    } else {
                        // |0> + i|1> = SH|0>
                        self.vec_s[pivot] = false;
                        self.vec_v[pivot] = true;
                        self.right_multiply_s(pivot)?;
                    }
                }
                PhaseFactor::MINUS_I => {
                    if self.vec_v[pivot] {
                        // H(|0> - i|1>) = e^{-iπ/4}SH|0>
                        self.phase_factor *= PhaseFactor::EXP_I_7PI_4;
                        self.vec_s[pivot] = false;
                        self.right_multiply_s(pivot)?;
                    } else {
                        // |0> - i|1> = SH|1>
                        self.vec_s[pivot] = true;
                        self.vec_v[pivot] = true;
                        self.right_multiply_s(pivot)?;
                    }
                }
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    fn handle_same_vecs_case(&mut self, delta: PhaseFactor, vec_t: &ndarray::Array1<bool>) {
        match delta {
            PhaseFactor::PLUS_ONE => {
                self.vec_s = vec_t.clone();
            }
            PhaseFactor::MINUS_ONE => {
                panic!("Inconsistent state: superposition with -1 coefficient.");
            }
            PhaseFactor::PLUS_I => {
                self.vec_s = vec_t.clone();
                self.phase_factor *= PhaseFactor::EXP_I_PI_4;
            }
            PhaseFactor::MINUS_I => {
                self.vec_s = vec_t.clone();
                self.phase_factor *= PhaseFactor::EXP_I_7PI_4;
            }
            _ => unreachable!(),
        }
    }

    fn get_differing_indices(
        &self,
        vec_t: &ndarray::Array1<bool>,
        vec_u: &ndarray::Array1<bool>,
    ) -> (Vec<usize>, Vec<usize>) {
        let mut diff_indices_0 = Vec::new();
        let mut diff_indices_1 = Vec::new();
        for i in 0..self.n {
            if vec_t[i] != vec_u[i] {
                if self.vec_v[i] {
                    diff_indices_1.push(i);
                } else {
                    diff_indices_0.push(i);
                }
            }
        }

        (diff_indices_0, diff_indices_1)
    }

    fn apply_basis_transform_circuit(
        &mut self,
        diff_indices_0: &[usize],
        diff_indices_1: &[usize],
    ) -> Result<usize> {
        if diff_indices_0.is_empty() {
            let pivot = diff_indices_1[0];
            for &i in &diff_indices_1[1..] {
                self.right_multiply_cx(i, pivot)?;
            }
            Ok(pivot)
        } else {
            let pivot = diff_indices_0[0];
            for &i in &diff_indices_0[1..] {
                self.right_multiply_cx(pivot, i)?;
            }
            for &i in diff_indices_1 {
                self.right_multiply_cz(pivot, i)?;
            }
            Ok(pivot)
        }
    }
}
