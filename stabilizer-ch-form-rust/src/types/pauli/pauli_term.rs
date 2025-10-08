use crate::types::pauli::pauli_string::Pauli;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct PauliTerm {
    pub op: Pauli,
    pub qubit: usize,
}
