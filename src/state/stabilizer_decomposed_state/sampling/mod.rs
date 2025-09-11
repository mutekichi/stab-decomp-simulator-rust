use crate::{
    error::Error,
    state::{Coefficient, StabilizerDecomposedState},
};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    pub(crate) fn _sample(&self, qargs: &[usize], shots: usize) -> Result<Vec<Vec<bool>>, Error> {
        dbg!(qargs, shots);
        Err(Error::Measurement("Not implemented".to_string())) // Placeholder
    }
}
