use stabilizer_ch_form_rust::error::Error as ChFormError;
use thiserror::Error;

/// A specialized Result type for compiler operations.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
/// Errors that can occur in compiler operations.
pub enum Error {
    /// Error for unsupported gates.
    #[error("Gate {0} is not supported.")]
    GateNotSupported(String),

    #[error(transparent)]
    ChForm(#[from] ChFormError),
}
