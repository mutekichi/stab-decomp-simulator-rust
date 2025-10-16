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
