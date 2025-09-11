pub(crate) mod compiler;
pub(crate) mod magic_states;
pub(crate) mod stabilizer_decomposed_state;
pub(crate) mod types;

pub(crate) use stabilizer_decomposed_state::StabilizerDecomposedState;
pub(crate) use types::coefficient::Coefficient;

use crate::{circuit::QuantumCircuit, state::{compiler::{errors::CompileError, CircuitCompiler, StabDecompCompiler}, types::scalar::Scalar}};

/// TODO: Add documentation for SimulatorState
pub struct QuantumState {
    internal_state: InternalState,
}

impl QuantumState {
    pub fn new(internal_state: InternalState) -> Self {
        Self { internal_state }
    }
}

enum InternalState {
    StabilizerDecomposedStateScalar(StabilizerDecomposedState<Scalar>),
}



impl QuantumState {
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
        let internal_state = compiler._compile(circuit)?;
        Ok(Self { internal_state })
    }

    /// Returns the statevector as a `Vec<Complex64>`.
    /// Note: This function is primarily for testing and debugging purposes.
    ///
    /// ### Returns
    /// `Array1<Complex64>` representing the statevector.
    pub fn to_statevector(&self) -> ndarray::Array1<num_complex::Complex64> {
        match &self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state._to_statevector(),
        }
    }

    /// Returns the inner product of the state and another state.
    ///
    /// ### Arguments
    /// * `other` - A reference to another `SimulatorState` to compute the inner product with.
    ///
    /// ### Returns
    /// A `Complex64` representing the inner product.
    pub fn inner_product(&self, other: &Self) -> num_complex::Complex64 {
        match (&self.internal_state, &other.internal_state) {
            (
                InternalState::StabilizerDecomposedStateScalar(state1),
                InternalState::StabilizerDecomposedStateScalar(state2),
            ) => state1._inner_product(state2),
        }
    }
}
