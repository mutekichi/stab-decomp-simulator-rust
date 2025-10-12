use crate::StabilizerCHForm;
use crate::core::internal::types::InternalGate;
use crate::core::internal::types::PhaseFactor;
use crate::error::ChFormError;
use num_complex::Complex64;

impl StabilizerCHForm {
    // TODO: Implement batch inner product calculation since the result of
    // `_self._get_normalize_to_zero_ops()` can be reused.
    pub(crate) fn _inner_product(
        &self,
        other: &StabilizerCHForm,
    ) -> Result<Complex64, ChFormError> {
        if self.n != other.n {
            return Err(ChFormError::QubitCountMismatch {
                operation: "calculating inner product",
                left: self.n,
                right: other.n,
            });
        }

        // Get operations to transform `self` to |0...0>
        // i.e. U_{ops} |self> = global_phase * phase * |0...0>
        let (ops, phase) = self._get_normalize_to_zero_ops()?;

        // Apply the same operations to `other`
        // i.e. U_{ops} |other> = |transformed_other>
        let transformed_other = other._get_ops_applied_state(&ops)?;

        // Get the amplitude of |0...0> in `transformed_other`
        // i.e. res = <0...0| U_{ops} |other>
        let res = transformed_other._amplitude_at_zero()?;

        // Combine the results
        // The inner product is <self|other> = <self|U_dag U|other>
        // Since U|self> = omega * phase * |0...0>, then <self|U_dag = omega.conj() * phase.conj() * <0...0|
        // So, <self|other> = omega.conj() * phase.conj() * <0...0|U|other>
        // <0...0|U|other> is `res.to_complex() / other.omega`, because _get_ops_applied_state carries over the omega.
        // The total omega of transformed_other is other.omega, but _amplitude_at_zero doesn't include it.
        // So res.to_complex() is the part without the original omega.
        let inner_product_val = (res * phase.conjugated()).to_complex();

        // We need to account for the global phases
        Ok(self.global_phase() * other.global_phase() * inner_product_val)
    }

    /// Returns the sequence of operations needed to transform the current state to |0...0>
    /// along with the phase factor of the resulting state.
    fn _get_normalize_to_zero_ops(&self) -> Result<(Vec<InternalGate>, PhaseFactor), ChFormError> {
        let mut ops = Vec::new();
        let mut self_clone = self.clone();
        let n = self_clone.n;

        // Step 1: Convert G to identity matrix using left CNOTs
        // NOTE: When G is converted to identity, F also becomes identity
        for j in 0..n {
            let mut pivot_row = j;
            if !self_clone.mat_g[[j, j]] {
                if let Some(k) = (j + 1..n).find(|&k| self_clone.mat_g[[k, j]]) {
                    pivot_row = k;
                } else {
                    // Unreachable if the state is valid
                    unreachable!("G matrix is not full rank, invalid stabilizer state.");
                }
            }

            if pivot_row != j {
                // Swap rows j and pivot_row using CNOTs: (k,j), (j,k), (k,j)
                ops.push(InternalGate::CX(pivot_row, j));
                self_clone._left_multiply_cx(pivot_row, j)?;
                ops.push(InternalGate::CX(j, pivot_row));
                self_clone._left_multiply_cx(j, pivot_row)?;
                ops.push(InternalGate::CX(pivot_row, j));
                self_clone._left_multiply_cx(pivot_row, j)?;
            }

            for i in 0..n {
                if i != j && self_clone.mat_g[[i, j]] {
                    ops.push(InternalGate::CX(j, i));
                    self_clone._left_multiply_cx(j, i)?;
                }
            }
        }

        // Step 2-1: Convert off-diagonal M to zero using left CZs
        for r in 0..n {
            for c in (r + 1)..n {
                if self_clone.mat_m[[r, c]] {
                    ops.push(InternalGate::CZ(r, c));
                    self_clone._left_multiply_cz(r, c)?;
                }
            }
        }

        // Step 2-2: Convert diagonal M to zero using left Sdg gates
        for q in 0..n {
            if self_clone.mat_m[[q, q]] {
                ops.push(InternalGate::Sdg(q));
                self_clone._left_multiply_sdg(q)?;
            }
        }

        // Step 3: Convert vec_v to zero using Hs
        for i in 0..n {
            if self_clone.vec_v[i] {
                ops.push(InternalGate::H(i));
                self_clone._left_multiply_h(i)?;
            }
        }

        // Step 4: Convert vec_s to zero using Xs
        for i in 0..n {
            if self_clone.vec_s[i] {
                ops.push(InternalGate::X(i));
                self_clone._left_multiply_x(i)?;
            }
        }

        Ok((ops, self_clone.phase_factor))
    }

    fn _get_ops_applied_state(
        &self,
        ops: &[InternalGate],
    ) -> Result<StabilizerCHForm, ChFormError> {
        let mut new_state = self.clone();
        for op in ops {
            match op {
                InternalGate::H(q) => new_state._left_multiply_h(*q)?,
                // InternalGate::S(q) => new_state._left_multiply_s(*q),
                InternalGate::Sdg(q) => new_state._left_multiply_sdg(*q)?,
                InternalGate::X(q) => new_state._left_multiply_x(*q)?,
                InternalGate::CX(c, t) => new_state._left_multiply_cx(*c, *t)?,
                InternalGate::CZ(c, t) => new_state._left_multiply_cz(*c, *t)?,
            }
        }
        Ok(new_state)
    }
}
