use num_complex::Complex64;
use stabilizer_ch_form_rust::types::pauli::PauliString;

use crate::{
    state::{Coefficient, StabilizerDecomposedState},
    types::error::Error,
};
impl<T: Coefficient> StabilizerDecomposedState<T> {
    pub(crate) fn _exp_value(&self, pauli_string: &PauliString) -> Result<Complex64, Error> {
        let mut exp_val = Complex64::new(0.0, 0.0);
        
        // To avoid repeated zipping, create a vector of pairs (stabilizer, coefficient).
        let terms: Vec<_> = self.stabilizers.iter().zip(self.coefficients.iter()).collect();

        for (i, (stab_i, coeff_i)) in terms.iter().enumerate() {
            // Apply Pauli P to |S_i> once per outer loop iteration.
            let evolved_stab = {
                let mut temp_stab = (*stab_i).clone();
                temp_stab.apply_pauli(pauli_string);
                temp_stab
            };
            
            // --- Diagonal term (j == i) ---
            // This calculates c_i* c_i <S_i|P|S_i>.
            let inner_prod_diag = stab_i.inner_product(&evolved_stab);
            exp_val += (coeff_i.conj() * **coeff_i).into() * inner_prod_diag;
            
            // --- Off-diagonal terms (j > i) ---
            // Loop through the remaining terms where j > i.
            for (stab_j, coeff_j) in terms.iter().skip(i + 1) {
                // Calculate the term for (i, j): c_j* c_i <S_j|P|S_i>.
                let inner_prod_off_diag = stab_j.inner_product(&evolved_stab);
                let term = (coeff_j.conj() * **coeff_i).into() * inner_prod_off_diag;
                
                // Add the term and its complex conjugate, which covers the (j, i) case.
                exp_val += term + term.conj();
            }
        }
        
        Ok(exp_val)
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    #[test]
    fn test_exp_value() {
        // sample_state = |000> + |100> + |010> + |111>
        let sample_state = crate::test_utils::create_sample_stab_decomp_state();
        let pauli_string = stabilizer_ch_form_rust::types::pauli::PauliString::from_str("IIZ").unwrap();
        let expected_result = 0.5;
        let result = sample_state._exp_value(&pauli_string).unwrap();
        dbg!(result);
        assert!((result.re - expected_result).abs() < 1e-10);
    }
}