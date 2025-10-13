use crate::{StabilizerCHForm, core::internal::types::PhaseFactor, error::Result};

impl StabilizerCHForm {
    pub(crate) fn _left_multiply_y(&mut self, qarg: usize) -> Result<()> {
        // We do not check for qarg out of bounds here
        // as _left_multiply_z and _left_multiply_x will do that.
        self._left_multiply_z(qarg)?;
        self._left_multiply_x(qarg)?;
        self.phase_factor *= PhaseFactor::PLUS_I;
        Ok(())
    }
}
