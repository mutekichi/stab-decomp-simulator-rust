use crate::StabilizerCHForm;

pub trait SwapGate {
    fn apply_swap(&mut self, qarg1: usize, qarg2: usize);
}

impl SwapGate for StabilizerCHForm {
    /// Applies the SWAP gate between the qubits at indices `qarg1` and `qarg2`.
    ///
    /// Time complexity: O(n)
    fn apply_swap(&mut self, qarg1: usize, qarg2: usize) {
        self._left_multiply_swap(qarg1, qarg2);
    }
}
