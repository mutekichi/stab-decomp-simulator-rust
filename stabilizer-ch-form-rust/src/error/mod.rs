use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// Custom error type for StabilizerCHForm operations.
#[derive(Error, Debug)]
pub enum Error {
    /// Error for invalid qubit index.
    #[error("Qubit index {0} is out of bounds for {1} qubits.")]
    QubitIndexOutOfBounds(usize, usize),

    /// Error for invalid number of qubits.
    #[error("Number of qubits must be greater than zero, got {0}.")]
    InvalidNumQubits(usize),

    /// Error for duplicate qubit indices in two-qubit gates.
    #[error("Two-qubit gate requires two different qubit indices, but got the same: {0}.")]
    DuplicateQubitIndices(usize),

    /// Error for impossible projection during measurement.
    #[error(
        "Impossible projection on qubit {qubit_index}: cannot project determined state |{}> onto |{}>.",
        if *desired { 0 } else { 1 },
        if *desired { 1 } else { 0 }
    )]
    ImpossibleProjection { qubit_index: usize, desired: bool },

    /// Error for mismatched qubit counts in operations involving two states (e.g., inner product).
    #[error(
        "The qubit counts of the two states must match for {}, got {} and {}.",
        operation,
        left,
        right
    )]
    QubitCountMismatch {
        operation: &'static str,
        left: usize,
        right: usize,
    },

    /// Error for invalid permutation length.
    #[error("The length of the permutation ({0}) must match the number of qubits ({1}).")]
    InvalidPermutationLength(usize, usize),

    /// Error for QASM parsing issues.
    #[error("QASM parsing error: {0}")]
    QasmParsingError(String),

    /// Error for Pauli string parsing issues.
    #[error("Pauli string parsing error: {0}")]
    PauliStringParsingError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
