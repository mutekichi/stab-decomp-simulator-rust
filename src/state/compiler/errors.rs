#[derive(Debug)]
pub enum CompileError {
    /// An unsupported gate was found in the circuit.
    GateNotSupported(String),
    /// An error occurred during the compilation logic.
    InternalError(String),
}
