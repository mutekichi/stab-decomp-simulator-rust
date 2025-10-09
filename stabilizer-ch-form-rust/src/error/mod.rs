use thiserror::Error;

/// Custom error type for StabilizerCHForm operations.
#[derive(Error, Debug)]
pub enum ChFormError {
    /// Error for invalid qubit index.
    #[error("Qubit index {0} is out of bounds for {1} qubits.")]
    InvalidQubitIndex(usize, usize), 
    /// Error for invalid number of qubits.
    #[error("Number of qubits must be greater than zero, got {0}.")]
    InvalidNumQubits(usize),
       
}