use crate::error::Result;
use crate::state::{Coefficient, StabilizerDecomposedState};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    pub fn _discard(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.discard(qarg)?;
        }
        Ok(())
    }
}
