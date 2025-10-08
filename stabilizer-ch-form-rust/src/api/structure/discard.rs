use crate::StabilizerCHForm;

impl StabilizerCHForm {
    pub fn discard(&mut self, qarg: usize) -> Result<(), &'static str> {
        self._discard(qarg)
    }
}
