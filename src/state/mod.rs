pub(crate) mod compiler;
pub(crate) mod magic_states;
pub(crate) mod stabilizer_decomposed_state;
pub(crate) mod types;

use stabilizer_ch_form_rust::types::pauli::PauliString;
pub(crate) use stabilizer_decomposed_state::StabilizerDecomposedState;
pub(crate) use types::coefficient::Coefficient;

use crate::{
    circuit::QuantumCircuit,
    state::{
        compiler::{errors::CompileError, CircuitCompiler, StabDecompCompiler},
        types::scalar::Scalar,
    }, types::{error::Error, result::shot_count::ShotCount}
};

/// TODO: Add documentation for QuantumState
pub struct QuantumState {
    internal_state: InternalState,
}

// impl QuantumState {
//     pub fn new(internal_state: InternalState) -> Self {
//         Self { internal_state }
//     }
// }

pub(crate) enum InternalState {
    StabilizerDecomposedStateScalar(StabilizerDecomposedState<Scalar>),
}

impl QuantumState {
    /// Creates a new `QuantumState` by compiling a `QuantumCircuit`.
    ///
    /// This function serves as the primary entry point for simulation. It takes a
    /// circuit blueprint and uses the default `StabDecompCompiler` to generate
    /// a computable state representation.
    ///
    /// ### Arguments
    /// * `circuit` - A reference to the `QuantumCircuit` to be simulated.
    ///
    /// ### Returns
    /// A `Result` containing the compiled `QuantumState` or a `CompileError`.
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
    /// * `other` - A reference to another `QuantumState` to compute the inner product with.
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

    /// Returns the number of qubits in the quantum state.
    ///
    /// ### Returns
    /// * `usize` - The number of qubits.
    pub fn num_qubits(&self) -> usize {
        match &self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.num_qubits,
        }
    }

    /// Measure the specified qubits and return the measurement results.
    /// The state gets collapsed according to the measurement results.
    ///
    /// ### Arguments
    /// * `qargs` - A slice of qubit indices to measure.
    ///
    /// ### Returns
    /// A `Result` containing a vector of boolean measurement results or an `Error`.
    pub fn measure(&mut self, qargs: &[usize]) -> Result<Vec<bool>, Error> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state._measure(qargs),
        }
    }

    /// Sample the specified qubits and return the measurement results.
    /// The state does not get collapsed.
    ///
    /// ### Arguments
    /// * `qargs` - A slice of qubit indices to sample.
    /// * `shots` - The number of samples to draw.
    ///
    /// ### Returns
    /// A `Result` containing a vector of boolean measurement results or an `Error`.
    pub fn sample(&self, qargs: &[usize], shots: usize) -> Result<ShotCount, Error> {
        match &self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state._sample(qargs, shots),
        }
    }

    /// Returns the expectation value of a given observable represented as a pauli string.
    ///
    /// ### Arguments
    /// * `pauli_string` - A reference to a `PauliString` representing the observable.
    ///
    /// ### Returns
    /// A `Result` containing the expectation value as `Complex64` or an `Error`.
    pub fn exp_value(&self, pauli_string: &PauliString)-> Result<num_complex::Complex64, Error> {
        match &self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state._exp_value(pauli_string),
        }
    }
}
