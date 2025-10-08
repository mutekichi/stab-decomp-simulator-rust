use crate::StabilizerCHForm;

pub trait ZGate {
    fn apply_z(&mut self, qarg: usize);
}

impl ZGate for StabilizerCHForm {
    /// Applies the Pauli-Z gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(1)
    fn apply_z(&mut self, qarg: usize) {
        self._left_multiply_z(qarg);
    }
}
