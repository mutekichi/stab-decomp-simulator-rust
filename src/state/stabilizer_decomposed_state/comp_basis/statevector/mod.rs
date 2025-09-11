use ndarray::Array1;
use num_complex::Complex64;

use crate::state::{Coefficient, StabilizerDecomposedState};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    /// Converts the stabilizer decomposed state to a statevector representation.
    pub(crate) fn _to_statevector(&self) -> Array1<Complex64> {
        let mut statevector = Array1::<Complex64>::zeros(1 << self.num_qubits);
        for (stab, coeff) in self.stabilizers.iter().zip(self.coefficients.iter()) {
            let stab_vector = stab.to_statevector();
            let coeff_complex: Complex64 = (*coeff).into();
            statevector = statevector + stab_vector * coeff_complex;
        }
        statevector
    }
}
