use crate::{circuit::QuantumCircuit, prelude::{compiler::{errors::CompileError, CircuitCompiler, StabDecompCompiler}, types::scalar::Scalar, SimulatorState}};

impl SimulatorState<Scalar> {
    /// Creates a new `SimulatorState` by compiling a `QuantumCircuit`.
    ///
    /// This function serves as the primary entry point for simulation. It takes a
    /// circuit blueprint and uses the default `StabDecompCompiler` to generate
    /// a computable state representation.
    ///
    /// # Arguments
    /// * `circuit` - A reference to the `QuantumCircuit` to be simulated.
    ///
    /// # Returns
    /// A `Result` containing the compiled `SimulatorState` or a `CompileError`.
    pub fn from_circuit(circuit: &QuantumCircuit) -> Result<Self, CompileError> {
        let compiler = StabDecompCompiler::new();
        compiler.compile(circuit)
    }
}