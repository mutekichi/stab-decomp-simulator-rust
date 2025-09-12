use num_complex::Complex64;
use stabilizer_ch_form_rust::types::pauli::PauliString;

use crate::{
    state::{Coefficient, StabilizerDecomposedState},
    types::error::Error,
};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    pub(crate) fn _exp_value(&self, pauli_string: &PauliString) -> Result<Complex64, Error> {
        let mut exp_val = Complex64::new(0.0, 0.0);
        // Iterate over all pairs of stabilizers to compute the expectation value
        //  sum_{i,j} c_j* c_i <S_j|P|S_i>
        //  stab1 stands for S_i, coeff1 for c_i
        //  stab2 stands for S_j, coeff2 for c_j
        // We apply the pauli_string to stab1 to get |P S_i>, then compute the inner product with stab2
        for (stab1, coeff1) in self.stabilizers.iter().zip(self.coefficients.iter()) {
            let evolved_stab = {
                let mut temp_stab = stab1.clone();
                temp_stab.apply_pauli(pauli_string);
                temp_stab
            };
            for (stab2, coeff2) in self.stabilizers.iter().zip(self.coefficients.iter()) {
                // Calculate <S_j|P S_i>
                let inner_prod = stab2.inner_product(&evolved_stab);
                exp_val += (coeff2.conj() * *coeff1).into() * inner_prod;
            }
        }
        Ok(exp_val)
    }
}
