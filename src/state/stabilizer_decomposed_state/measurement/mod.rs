use crate::{
    state::{Coefficient, StabilizerDecomposedState},
    types::error::Error,
};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    pub(crate) fn _measure(&mut self, qargs: &[usize]) -> Result<Vec<bool>, Error> {
        dbg!(qargs);
        Err(Error::Measurement("Not implemented".to_string())) // Placeholder
    }
}
