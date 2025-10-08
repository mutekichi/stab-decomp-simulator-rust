use crate::StabilizerCHForm;

pub trait XGate {
    fn apply_x(&mut self, qarg: usize);
}

impl XGate for StabilizerCHForm {
    /// Applies the Pauli-X gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(n)
    ///
    /// See around eq.(48) of arXiv:1808.00128 for details.
    fn apply_x(&mut self, qarg: usize) {
        self._left_multiply_x(qarg);
    }
}
