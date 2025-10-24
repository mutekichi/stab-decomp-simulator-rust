use crate::types::pauli::pauli_string::Pauli;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

/// Represents a single term in a sparse Pauli string, e.g., "X1" or "Y3".
pub struct PauliTerm {
    pub op: Pauli,
    pub qubit: usize,
}
