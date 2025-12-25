use crate::{StabilizerCHForm, error::Result, form::types::PhaseFactor};

impl StabilizerCHForm {
    pub(crate) fn left_multiply_y(&mut self, qarg: usize) -> Result<()> {
        // We do not check for qarg out of bounds here
        // as _left_multiply_z and _left_multiply_x will do that.
        self.left_multiply_z(qarg)?;
        self.left_multiply_x(qarg)?;
        self.phase_factor *= PhaseFactor::PLUS_I;
        Ok(())
    }
}
