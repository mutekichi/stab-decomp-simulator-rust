use crate::{circuit::QuantumCircuit, prelude::{compiler::{errors::CompileError, CircuitCompiler, StabDecompCompiler}, types::scalar::Scalar, SimulatorState}};

impl SimulatorState<Scalar> {
    /// Creates a new `SimulatorState` by compiling a `QuantumCircuit`.
    ///
    /// This function serves as the primary entry point for simulation. It takes a
    /// circuit blueprint and uses the default `StabDecompCompiler` to generate
    /// a computable state representation.
    ///
    /// ### Arguments
    /// * `circuit` - A reference to the `QuantumCircuit` to be simulated.
    ///
    /// ### Returns
    /// A `Result` containing the compiled `SimulatorState` or a `CompileError`.
    pub fn from_circuit(circuit: &QuantumCircuit) -> Result<Self, CompileError> {
        let compiler = StabDecompCompiler::new();
        compiler.compile(circuit)
    }

    /// Returns the statevector as a `Vec<Complex64>`.
    /// Note: This function is primarily for testing and debugging purposes. 
    /// 
    /// ### Returns
    /// `Array1<Complex64>` representing the statevector.
    pub fn to_statevector(&self) -> ndarray::Array1<num_complex::Complex64> {
        self.internal_state.to_statevector()
    }
}