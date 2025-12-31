use crate::state::compiler::error::Error as CompileError;
use stabilizer_ch_form_rust::error::Error as ChFormError;
use thiserror::Error;

/// A specialized `Result` type for NECSTAR operations.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
/// Errors that can occur in NECSTAR operations.
pub enum Error {
    /// Error for invalid qubit index.
    #[error("Qubit index {0} is out of bounds for {1} qubits.")]
    QubitIndexOutOfBounds(usize, usize),

    /// Error for invalid number of qubits.
    #[error("Number of qubits must be greater than zero, got {0}.")]
    InvalidNumQubits(usize),

    /// Error for attempting to convert a non-Clifford gate to a Clifford gate.
    #[error("Attempted to convert a non-Clifford gate to a Clifford gate: {0}")]
    GateNotClifford(String),

    /// Error for statevector calculations that exceed feasible limits.
    #[error("Calculating the statevector for a state with {0} qubits is not feasible.")]
    StatevectorTooLarge(usize),

    /// Error for direct application of non-Clifford gates.
    #[error("Direct application of non-Clifford gate {0} is not supported.")]
    NonCliffordGateApplication(String),

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

    /// Error for measurements exceeding supported qubit limits.
    #[error("Measurements of more than 128 qubits are not supoprted.")]
    MeasurementTooManyQubits,

    /// Error for sampling more than supported qubit limits.
    #[error("Sampling more than 128 qubits is not supported.")]
    SamplingTooManyQubits,

    /// Error for impossible projections.
    #[error(
        "Impossible projection on qubit {qubit_index}: cannot project determined state |{}> onto |{}>.",
        if *desired { 0 } else { 1 },
        if *desired { 1 } else { 0 }
    )]
    ImpossibleProjection { qubit_index: usize, desired: bool },

    /// Error for duplicate qubit indices in a argument list.
    #[error("Duplicate qubit index found: {0}.")]
    DuplicateQubitIndex(usize),

    /// Error for invalid Pauli string length.
    #[error("Invalid Pauli string length: expected {expected}, found {found}.")]
    InvalidPauliStringLength { expected: usize, found: usize },

    /// Error for empty qubit index list.
    #[error("Qubit index list is empty.")]
    EmptyQubitIndices,

    /// Error for QASM parsing issues.
    #[error("QASM parsing error: {0}")]
    QasmParsingError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Compile(#[from] CompileError),

    #[error(transparent)]
    Binomial(#[from] rand_distr::BinomialError),

    #[error(transparent)]
    ChForm(#[from] ChFormError),

    /// Error for unimplemented features.
    #[error("Not implemented: {0}")]
    NotImplemented(String),
}
// DONE
