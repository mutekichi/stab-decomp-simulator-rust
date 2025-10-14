use crate::state::compiler::error::Error as CompileError;
use stabilizer_ch_form_rust::error::Error as ChFormError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    /// Error for invalid qubit index.
    #[error("Qubit index {0} is out of bounds for {1} qubits.")]
    QubitIndexOutOfBounds(usize, usize),

    /// Error for invalid number of qubits.
    #[error("Number of qubits must be greater than zero, got {0}.")]
    InvalidNumQubits(usize),

    #[error("Attempted to convert a non-Clifford gate to a Clifford gate: {0}")]
    GateNotClifford(String),

    #[error("Calculating the statevector for a state with {0} qubits is not feasible.")]
    StatevectorTooLarge(usize),

    #[error("Not implemented: {0}")]
    NotImplemented(String),

    #[error(
        "Impossible projection on qubit {qubit_index}: cannot project determined state |{}> onto |{}>.",
        if *desired { 0 } else { 1 },
        if *desired { 1 } else { 0 }
    )]
    ImpossibleProjection { qubit_index: usize, desired: bool },

    #[error(transparent)]
    Compile(#[from] CompileError),

    #[error(transparent)]
    Binomial(#[from] rand_distr::BinomialError),

    #[error(transparent)]
    ChForm(#[from] ChFormError),
}
