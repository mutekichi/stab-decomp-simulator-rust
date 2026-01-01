use num_complex::Complex64;

use crate::error::Result;
use crate::state::{Coefficient, StabilizerDecomposedState};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    /// Computes the inner product between two [`StabilizerDecomposedState`] instances.
    /// i.e. ⟨self|other⟩
    pub(crate) fn inner_product(&self, other: &Self) -> Result<Complex64> {
        let mut result = Complex64::new(0.0, 0.0);

        for (stab1, coeff1) in self.stabilizers.iter().zip(self.coefficients.iter()) {
            for (stab2, coeff2) in other.stabilizers.iter().zip(other.coefficients.iter()) {
                let ip = stab1.inner_product(stab2)?;
                result += (coeff1.conj() * *coeff2).into() * ip;
            }
        }
        Ok(result * self.global_factor.conj() * other.global_factor)
    }
}

#[cfg(test)]
mod tests {
    use num_complex::Complex64;
    use stabilizer_ch_form_rust::StabilizerCHForm;

    use crate::{
        prelude::QuantumState,
        state::StabilizerDecomposedState,
        test_utils::{
            assert_eq_complex, create_sample_stab_decomp_state, random_circuit_with_t_gate,
        },
    };

    #[test]
    fn test_inner_product_simple() {
        // 1/2 (|000> + |010> + |001> + |111>)
        let state1 = create_sample_stab_decomp_state();
        // |000>
        let state2 = {
            let stab = StabilizerCHForm::new(3).unwrap();
            let coeffs = vec![Complex64::new(1.0, 0.0)];
            StabilizerDecomposedState::new(3, vec![stab], coeffs)
        };
        let inner_prod = state1.inner_product(&state2).unwrap();
        assert_eq_complex(inner_prod, Complex64::new(0.5, 0.0));
    }

    #[test]
    fn test_inner_product_invalid_length() {
        let state1 = create_sample_stab_decomp_state();
        let state2 = {
            let stab = StabilizerCHForm::new(2).unwrap();
            let coeffs = vec![Complex64::new(1.0, 0.0)];
            StabilizerDecomposedState::new(2, vec![stab], coeffs)
        };
        let result = state1.inner_product(&state2);
        assert!(result.is_err());
    }

    #[test]
    fn test_inner_product_random() {
        for i in 0..10 {
            let seed1 = 42 + i;
            let seed2 = 123 + i;
            let circuit_1 = random_circuit_with_t_gate(6, 100, 10, Some(seed1));
            let circuit_2 = random_circuit_with_t_gate(6, 100, 10, Some(seed2));

            let state_1 = QuantumState::from_circuit(&circuit_1).unwrap();
            let state_2 = QuantumState::from_circuit(&circuit_2).unwrap();

            let inner_prod_naive = {
                let sv1 = state_1.to_statevector().unwrap();
                let sv2 = state_2.to_statevector().unwrap();
                sv2.dot(&sv1.mapv(|x| x.conj()))
            };

            let inner_prod_efficient = state_1.inner_product(&state_2).unwrap();

            assert_eq_complex(inner_prod_naive, inner_prod_efficient);
        }
    }
}
// WIP: Add simple case tests
