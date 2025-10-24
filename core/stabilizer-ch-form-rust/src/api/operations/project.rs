use crate::{StabilizerCHForm, error::Result};

impl StabilizerCHForm {
    /// Projects a qubit onto a computational basis state (`|0>` or `|1>`).
    ///
    /// This operation modifies the stabilizer state in place.
    ///
    /// In a stabilizer state, measuring a qubit in the computational basis yields either a
    /// deterministic outcome (`|0>` or `|1>`) or a perfectly random one (50% probability for each).
    /// This function attempts to force the qubit into the specified `outcome`, succeeding if the projection
    /// is physically possible.
    ///
    /// # Arguments
    ///
    /// * `qarg`: The index of the qubit to project.
    /// * `outcome`: The desired basis state to project onto (`false` for `|0>`, `true` for `|1>`).
    ///
    /// # Returns
    ///
    /// A `Result` indicating the outcome of the projection:
    ///
    /// * `Ok(true)` if the projection was **deterministic**. This means the qubit was already
    ///   in the desired state. The stabilizer state is unchanged.
    /// * `Ok(false)` if the projection was **non-deterministic** (probabilistic). This means the
    ///   qubit was in a superposition and has now been collapsed to the desired state. The
    ///   stabilizer state has been updated.
    ///
    /// # Errors
    ///
    /// Returns an `ChFormError` if the projection is impossible. This occurs when the qubit has a
    /// deterministic value that is orthogonal to the desired `outcome` (e.g., attempting to
    /// project a qubit in state `|0>` onto `|1>`).
    pub fn project(&mut self, qarg: usize, outcome: bool) -> Result<bool> {
        self._project(qarg, outcome)
    }
}
