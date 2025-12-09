use crate::{StabilizerCHForm, error::Result};

impl StabilizerCHForm {
    // This implementation may not be the most efficient due to multiple H gate applications.
    // We assume that SqrtX gate is not so frequently used in practice...
    pub(crate) fn _left_multiply_sqrt_x(&mut self, qarg: usize) -> Result<()> {
        self._left_multiply_h(qarg)?;
        self._left_multiply_s(qarg)?;
        self._left_multiply_h(qarg)
    }

    pub(crate) fn _left_multiply_sqrt_xdg(&mut self, qarg: usize) -> Result<()> {
        self._left_multiply_h(qarg)?;
        self._left_multiply_sdg(qarg)?;
        self._left_multiply_h(qarg)
    }
}
