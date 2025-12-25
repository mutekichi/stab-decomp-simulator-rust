#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InternalGate {
    H(usize),
    // S(usize), // Internal S gate is not used
    Sdg(usize),
    X(usize),
    CX(usize, usize),
    CZ(usize, usize),
}
