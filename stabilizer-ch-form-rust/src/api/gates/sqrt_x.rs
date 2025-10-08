use crate::StabilizerCHForm;

pub trait SqrtXGate {
    fn apply_sqrt_x(&mut self, qarg: usize);
    fn apply_sqrt_xdg(&mut self, qarg: usize);
}

impl SqrtXGate for StabilizerCHForm {
    /// Applies the √X gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(n^2)
    fn apply_sqrt_x(&mut self, qarg: usize) {
        self._left_multiply_sqrt_x(qarg);
    }

    /// Applies the adjoint of the √X gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(n^2)
    fn apply_sqrt_xdg(&mut self, qarg: usize) {
        self._left_multiply_sqrt_xdg(qarg);
    }
}
