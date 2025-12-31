use num_complex::Complex64;

use crate::error::Result;
use crate::state::{Coefficient, StabilizerDecomposedState};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    /// Calculates the squared norm of the state.
    pub(crate) fn norm_squared(&self) -> Result<f64> {
        let mut sum = Complex64::new(0.0, 0.0);
        let terms: Vec<_> = self
            .stabilizers
            .iter()
            .zip(self.coefficients.iter())
            .collect();

        for (i, (stab_i, coeff_i)) in terms.iter().enumerate() {
            // Diagonal term (j == i)
            let inner_prod_diag = stab_i.inner_product(stab_i)?;
            sum += (coeff_i.conj() * **coeff_i).into() * inner_prod_diag;

            // Off-diagonal terms (j > i)
            for (stab_j, coeff_j) in terms.iter().skip(i + 1) {
                let inner_prod_off_diag = stab_i.inner_product(stab_j)?;
                let term = (coeff_i.conj() * **coeff_j).into() * inner_prod_off_diag;
                sum += term + term.conj();
            }
        }

        Ok(sum.re * self.global_factor.norm_sqr())
    }

    /// Calculates the norm of the state.
    pub(crate) fn norm(&self) -> Result<f64> {
        Ok(self.norm_squared()?.sqrt())
    }
}
// WIP: Add tests
