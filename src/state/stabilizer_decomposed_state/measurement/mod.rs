use crate::error::{Error, Result};
use crate::state::{Coefficient, StabilizerDecomposedState};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    pub(crate) fn _measure(&mut self, qargs: &[usize]) -> Result<Vec<bool>> {
        dbg!(qargs);
        Err(Error::NotImplemented("Not implemented".to_string()))
    }
}
