#[derive(Debug, Clone, Copy, PartialEq, Eq)]

/// Represents a single-qubit Pauli operator.
pub enum Pauli {
    I,
    X,
    Y,
    Z,
}
