use crate::types::pauli::pauli_string::Pauli;

/// Represents a single term in a sparse Pauli string.
/// ## Examples
/// ```rust
/// use stabilizer_ch_form_rust::types::pauli::{ Pauli, PauliString, PauliTerm };
/// 
/// let term = PauliTerm { op: Pauli::X, qubit: 2 };
/// assert_eq!(term.op, Pauli::X);
/// assert_eq!(term.qubit, 2);
/// 
/// let sparse: PauliString = "X1 Y3".parse().unwrap();
/// assert_eq!(sparse, PauliString::Sparse(vec![
///    PauliTerm { op: Pauli::X, qubit: 1 },
///    PauliTerm { op: Pauli::Y, qubit: 3 },
/// ]));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PauliTerm {
    pub op: Pauli,
    pub qubit: usize,
}
