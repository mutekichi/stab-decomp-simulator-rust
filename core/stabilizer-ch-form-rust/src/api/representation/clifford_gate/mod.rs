#[derive(Debug, Clone, PartialEq)]
pub enum CliffordGate {
    H(usize),
    X(usize),
    Y(usize),
    Z(usize),
    S(usize),
    Sdg(usize),
    SqrtX(usize),
    SqrtXdg(usize),
    CX(usize, usize),
    CZ(usize, usize),
    Swap(usize, usize),
}
