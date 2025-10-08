use crate::StabilizerCHForm;

pub trait YGate {
    fn apply_y(&mut self, qarg: usize);
}

impl YGate for StabilizerCHForm {
    /// Applies the Pauli-Y gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(n)
    fn apply_y(&mut self, qarg: usize) {
        self._left_multiply_y(qarg);
    }
}
