use ndarray::Array1;
use num_complex::Complex64;

use crate::error::{Error, Result};
use crate::state::{Coefficient, StabilizerDecomposedState};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    /// Converts the stabilizer decomposed state to a statevector representation.
    /// Note that the state is represented as a dense vector, which may be inefficient for large
    /// number of qubits.
    /// The indexing of the statevector is in little-endian order like in Qiskit.
    pub(crate) fn to_statevector(&self) -> Result<Array1<Complex64>> {
        const MAX_QUBITS_FOR_STATEVECTOR: usize = 28;
        if self.num_qubits > MAX_QUBITS_FOR_STATEVECTOR {
            return Err(Error::StatevectorTooLarge(self.num_qubits));
        }
        let mut statevector = Array1::<Complex64>::zeros(1 << self.num_qubits);
        for (stab, coeff) in self.stabilizers.iter().zip(self.coefficients.iter()) {
            let stab_vector = stab.to_statevector()?;
            let coeff_complex: Complex64 = (*coeff).into();
            statevector = statevector + stab_vector * coeff_complex;
        }
        Ok(statevector * self.global_factor)
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{assert_eq_complex_array1, create_sample_stab_decomp_state};
    use ndarray::array;
    use num_complex::Complex64;

    #[test]
    fn test_to_statevector() {
        // sample_state = |000> + |100> + |010> + |111>
        let sample_state = create_sample_stab_decomp_state();
        let statevector = sample_state.to_statevector().unwrap();
        let expected_statevector = array![
            Complex64::new(0.5, 0.0),
            Complex64::new(0.5, 0.0),
            Complex64::new(0.5, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.5, 0.0)
        ];
        assert_eq_complex_array1(&statevector, &expected_statevector);
    }
}
// DONE
