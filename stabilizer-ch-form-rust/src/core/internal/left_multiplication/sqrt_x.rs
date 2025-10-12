use crate::{StabilizerCHForm, error::ChFormError};

impl StabilizerCHForm {
    pub(crate) fn _left_multiply_sqrt_x(&mut self, qarg: usize) -> Result<(), ChFormError> {
        self._left_multiply_h(qarg)?;
        self._left_multiply_s(qarg)?;
        self._left_multiply_h(qarg)
    }

    pub(crate) fn _left_multiply_sqrt_xdg(&mut self, qarg: usize) -> Result<(), ChFormError> {
        self._left_multiply_h(qarg)?;
        self._left_multiply_sdg(qarg)?;
        self._left_multiply_h(qarg)
    }
}
