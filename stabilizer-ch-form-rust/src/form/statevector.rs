use crate::{StabilizerCHForm, error::Result};
use ndarray::Array1;
use num_complex::Complex64;

impl StabilizerCHForm {
    /// Represents this state as a statevector.
    ///
    /// NOTE:
    ///  * This implementation iterates over all 2^n basis states. This functionality is
    ///    mainly for testing and debugging purposes.
    ///  * Uses little-endian convention for basis states.
    pub fn to_statevector(&self) -> Result<Array1<Complex64>> {
        let dim = 1 << self.n_qubits(); // 2^n
        let mut statevector = ndarray::Array1::from_elem(dim, Complex64::new(0.0, 0.0));

        for i in 0..dim {
            let bitstring: ndarray::Array1<bool> =
                (0..self.n_qubits()).map(|j| (i & (1 << j)) != 0).collect();
            statevector[i] = self
                ._amplitude_at_computational_basis(&bitstring)?
                .to_complex()
                * self.global_phase();
        }

        Ok(statevector)
    }
}
