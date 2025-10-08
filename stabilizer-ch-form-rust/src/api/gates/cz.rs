use crate::StabilizerCHForm;

pub trait CZGate {
    fn apply_cz(&mut self, qarg1: usize, qarg2: usize);
}

impl CZGate for StabilizerCHForm {
    /// Applies the CZ gate between qubits at indices `qarg1` and `qarg2`.
    ///
    /// Time complexity: O(n)
    ///
    /// See around eq.(50) of arXiv:1808.00128 for details.
    fn apply_cz(&mut self, qarg1: usize, qarg2: usize) {
        self._left_multiply_cz(qarg1, qarg2);
    }
}
