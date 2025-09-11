use num_complex::Complex64;

use crate::state::{Coefficient, StabilizerDecomposedState};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    /// Computes the inner product between two `StabilizerDecomposedState` instances.
    /// i.e. ⟨self|other⟩
    ///
    /// ### Arguments
    /// * `other` - A reference to another `QuantumState` instance.
    ///
    /// ### Returns
    /// A `Complex64` representing the inner product.
    pub(crate) fn _inner_product(&self, other: &Self) -> Complex64 {
        let mut result = Complex64::new(0.0, 0.0);

        for (stab1, coeff1) in self.stabilizers.iter().zip(self.coefficients.iter()) {
            for (stab2, coeff2) in other.stabilizers.iter().zip(other.coefficients.iter()) {
                let ip = stab1.inner_product(stab2);
                result += (coeff1.conj() * *coeff2).into() * ip;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        prelude::QuantumState,
        test_utils::{assert_eq_complex, random_circuit_with_t_gate},
    };

    #[test]
    fn test_inner_product() {
        for _ in 0..10 {
            let circuit_1 = random_circuit_with_t_gate(6, 100, 10, None);
            let circuit_2 = random_circuit_with_t_gate(6, 100, 10, None);

            let state_1 = QuantumState::from_circuit(&circuit_1).unwrap();
            let state_2 = QuantumState::from_circuit(&circuit_2).unwrap();

            let inner_prod_naive = {
                let sv1 = state_1.to_statevector();
                let sv2 = state_2.to_statevector();
                sv2.dot(&sv1.mapv(|x| x.conj()))
            };

            let inner_prod_efficient = state_1.inner_product(&state_2);

            assert_eq_complex(inner_prod_naive, inner_prod_efficient);
        }
    }
}
