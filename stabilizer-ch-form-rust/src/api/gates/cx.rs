use crate::StabilizerCHForm;

pub trait CXGate {
    fn apply_cx(&mut self, control: usize, target: usize);
}

impl CXGate for StabilizerCHForm {
    /// Applies the CNOT (CX) gate with control qubit at index `control` and target qubit at index `target`.
    ///
    /// Time complexity: O(n)
    ///
    /// See around eq.(49) of arXiv:1808.00128 for details.
    fn apply_cx(&mut self, control: usize, target: usize) {
        self._left_multiply_cx(control, target);
    }
}
