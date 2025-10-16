use stabilizer_ch_form_rust::error::Error as ChFormError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Gate {0} is not supported.")]
    GateNotSupported(String),

    #[error(transparent)]
    ChForm(#[from] ChFormError),
}
