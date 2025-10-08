use crate::StabilizerCHForm;

pub trait HGate {
    fn apply_h(&mut self, qarg: usize);
}

impl HGate for StabilizerCHForm {
    /// Applies the Hadamard gate to the qubit at index `qarg`.
    ///     
    /// Time complexity: O(n^2)
    ///
    /// See around Proposition 4. of arXiv:1808.00128 for details.
    fn apply_h(&mut self, qarg: usize) {
        self._left_multiply_h(qarg);
    }
}
