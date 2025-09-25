use crate::{
    state::{Coefficient, StabilizerDecomposedState},
    types::error::Error,
};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    pub(crate) fn _project_normalized(&mut self, qubit: usize, outcome: bool) -> Result<(), Error> {
        self._project_unnormalized(qubit, outcome)?;
        let norm = self._norm();
        if norm == 0.0 {
            return Err(Error::Projection(
                "Projection resulted in zero norm state".to_string(),
            ));
        }
        self.global_factor /= norm;
        Ok(())
    }
}

impl<T: Coefficient> StabilizerDecomposedState<T> {
    // NOTE: This function always successes even if the projection is impossible for the state.
    //       When the projection is impossible, the norm of the state becomes zero.
    pub(crate) fn _project_unnormalized(
        &mut self,
        qubit: usize,
        outcome: bool,
    ) -> Result<(), Error> {
        // Filter out stabilizers that cannot be projected to the desired outcome
        // NOTE: We can optimize this by avoiding the allocation of a new vector
        //       and instead using `retain` if performance becomes an issue.
        let (stabs, coeffs): (Vec<_>, Vec<_>) = self
            .stabilizers
            .drain(..)
            .zip(self.coefficients.drain(..))
            .filter_map(|(mut stab, coeff)| {
                if stab.project(qubit, outcome).is_ok() {
                    Some((stab, coeff))
                } else {
                    None
                }
            })
            .unzip();

        self.stabilizers = stabs;
        self.coefficients = coeffs;

        Ok(())
    }
}
