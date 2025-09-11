use crate::{
    state::{Coefficient, StabilizerDecomposedState}, types::{error::Error, result::shot_count::ShotCount},
};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    pub(crate) fn _sample(&self, qargs: &[usize], shots: usize) -> Result<ShotCount, Error> {
        dbg!(qargs, shots);
        Err(Error::Measurement("Not implemented".to_string())) // Placeholder
    }
}
