use crate::error::Result;
use crate::state::{Coefficient, StabilizerDecomposedState};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    pub(crate) fn discard(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.discard(qarg)?;
        }
        self.num_qubits -= 1;
        Ok(())
    }
}
// WIP: Add tests
