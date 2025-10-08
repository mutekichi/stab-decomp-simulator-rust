use crate::StabilizerCHForm;

impl StabilizerCHForm {
    pub fn measure(&mut self, qarg: usize) -> bool {
        self._measure(qarg)
    }
}
