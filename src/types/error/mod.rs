#[derive(Debug)]
pub enum Error {
    Compilation(String),
    Measurement(String),
    Projection(String),
    Sampling(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Compilation(msg) => write!(f, "Compilation Error: {}", msg),
            Error::Measurement(msg) => write!(f, "Measurement Error: {}", msg),
            Error::Projection(msg) => write!(f, "Projection Error: {}", msg),
            Error::Sampling(msg) => write!(f, "Sampling Error: {}", msg),
        }
    }
}
