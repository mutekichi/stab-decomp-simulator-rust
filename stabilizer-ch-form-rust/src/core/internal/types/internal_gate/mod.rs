use crate::StabilizerCHForm;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Should be changed to pub(crate)
pub enum InternalGate {
    H(usize),
    // S(usize), // Internal S gate is not used
    Sdg(usize),
    X(usize),
    CX(usize, usize),
    CZ(usize, usize),
}

impl StabilizerCHForm {
    pub(crate) fn _get_ops_applied_state(&self, ops: &[InternalGate]) -> StabilizerCHForm {
        let mut new_state = self.clone();
        for op in ops {
            match op {
                InternalGate::H(q) => new_state._left_multiply_h(*q),
                // InternalGate::S(q) => new_state._left_multiply_s(*q),
                InternalGate::Sdg(q) => new_state._left_multiply_sdg(*q),
                InternalGate::X(q) => new_state._left_multiply_x(*q),
                InternalGate::CX(c, t) => new_state._left_multiply_cx(*c, *t),
                InternalGate::CZ(c, t) => new_state._left_multiply_cz(*c, *t),
            }
        }
        new_state
    }
}
