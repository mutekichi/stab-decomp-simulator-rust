pub(crate) mod compiler;
pub(crate) mod magic_states;
pub(crate) mod stabilizer_decomposed_state;
pub(crate) mod types;

use ndarray::Array1;
use stabilizer_ch_form_rust::types::pauli::PauliString;
pub(crate) use stabilizer_decomposed_state::StabilizerDecomposedState;
pub(crate) use types::coefficient::Coefficient;

use crate::{
    circuit::QuantumCircuit,
    error::Result,
    state::{
        compiler::{CircuitCompiler, StabDecompCompiler},
        types::scalar::Scalar,
    },
    types::shot_count::ShotCount,
};

/// TODO: Add documentation for QuantumState
pub struct QuantumState {
    internal_state: InternalState,
}

pub(crate) enum InternalState {
    StabilizerDecomposedStateScalar(StabilizerDecomposedState<Scalar>),
}

impl QuantumState {
    // ===== Primary APIs =====

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
    pub fn from_circuit(circuit: &QuantumCircuit) -> Result<Self> {
        let compiler = StabDecompCompiler::new();
        let internal_state = compiler._compile(circuit)?;
        Ok(Self { internal_state })
    }

    /// Returns the statevector as a `Vec<Complex64>`.
    /// Note: This function is primarily for testing and debugging purposes.
    ///
    /// ### Returns
    /// `Array1<Complex64>` representing the statevector.
    pub fn to_statevector(&self) -> Result<Array1<num_complex::Complex64>> {
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
    pub fn inner_product(&self, other: &Self) -> Result<num_complex::Complex64> {
        match (&self.internal_state, &other.internal_state) {
            (
                InternalState::StabilizerDecomposedStateScalar(state1),
                InternalState::StabilizerDecomposedStateScalar(state2),
            ) => state1._inner_product(state2),
        }
    }

    /// Measure the specified qubits in the computational basis and return the measurement results.
    /// The state gets collapsed according to the measurement results.
    ///
    /// ### Arguments
    /// * `qargs` - A slice of qubit indices to measure.
    /// * `seed` - An optional seed for the random number generator to ensure reproducibility.
    ///
    /// ### Returns
    /// A `Result` containing a vector of boolean measurement results or an `Error`.
    pub fn measure(&mut self, qargs: &[usize], seed: Option<[u8; 32]>) -> Result<Vec<bool>> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state._measure(qargs, seed),
        }
    }

    /// Measure the all qubits in the computational basis and return the measurement results.
    /// The state gets collapsed according to the measurement results.
    ///
    /// ### Returns
    /// A `Result` containing a vector of boolean measurement results or an `Error`.
    pub fn measure_all(&mut self, seed: Option<[u8; 32]>) -> Result<Vec<bool>> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state._measure_all(seed),
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
    pub fn sample(
        &self,
        qargs: &[usize],
        shots: usize,
        seed: Option<[u8; 32]>,
    ) -> Result<ShotCount> {
        match &self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => {
                state._sample(qargs, shots, seed)
            }
        }
    }

    /// Returns the expectation value of a given observable represented as a pauli string.
    ///
    /// ### Arguments
    /// * `pauli_string` - A reference to a `PauliString` representing the observable.
    ///
    /// ### Returns
    /// A `Result` containing the expectation value as `Complex64` or an `Error`.
    pub fn exp_value(&self, pauli_string: &PauliString) -> Result<num_complex::Complex64> {
        match &self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state._exp_value(pauli_string),
        }
    }

    /// Project the state onto the ±1 eigenspace of the Pauli Z operator on the specified qubit with normalization.
    /// The state is modified in place.
    ///
    /// ### Arguments
    /// * `qubit` - The index of the qubit to project.
    /// * `outcome` - The measurement outcome (true for +1, false for -1).
    ///
    /// ### Returns
    /// A `Result` indicating success or an `Error`.
    pub fn project_normalized(&mut self, qubit: usize, outcome: bool) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => {
                state._project_normalized(qubit, outcome)
            }
        }
    }

    /// Project the state onto the ±1 eigenspace of the Pauli Z operator on the specified qubit without normalization.
    ///
    /// The state is internally represented as a stabilizer decomposed state:
    /// $$|\phi\rangle = \sum_i c_i |\psi_i\rangle$$ and the projected state is given by:
    /// $$
    /// \Pi_{Z_j = \pm 1} |\phi\rangle = \sum_i c_i \right(I + (-1)^{o} Z_j\left)/2 |\psi_i\rangle,
    /// which is generally unnormalized.
    ///
    /// The state is modified in place.
    ///
    /// ### Arguments
    /// * `qubit` - The index of the qubit to project.
    /// * `outcome` - The measurement outcome (true for +1, false for -1).
    ///
    /// ### Returns
    /// A `Result` indicating success or an `Error`.
    pub fn project_unnormalized(&mut self, qubit: usize, outcome: bool) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => {
                state._project_unnormalized(qubit, outcome)
            }
        }
    }

    /// Discard the specified qubit from the quantum state.
    /// The state is modified in place.
    ///
    /// NOTE: Make sure that the qubit to be discarded is projected to |0> before calling this function.
    ///       Discarding a qubit that is not in |0> may lead to incorrect results.
    ///       This function does not check if the qubit is in |0> for performance reasons.
    ///
    /// ## Arguments
    /// * `qubit` - The index of the qubit to discard.
    ///
    /// ## Returns
    /// A `Result` indicating success or an `Error`.
    pub fn discard(&mut self, qubit: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state._discard(qubit),
        }
    }

    // ===== Gate Applications =====

    /// Applies a Pauli-X gate to the specified qubit.
    ///
    /// ### Arguments
    /// * `qubit` - The index of the qubit to apply the gate to.
    ////
    /// ### Returns
    /// Nothing. The state is modified in place.
    pub fn apply_x(&mut self, qubit: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state._apply_x(qubit),
        }
    }

    /// Applies a Pauli-Y gate to the specified qubit.
    pub fn apply_y(&mut self, qubit: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state._apply_y(qubit),
        }
    }

    /// Applies a Pauli-Z gate to the specified qubit.
    pub fn apply_z(&mut self, qubit: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state._apply_z(qubit),
        }
    }

    /// Applies a Hadamard gate to the specified qubit.
    pub fn apply_h(&mut self, qubit: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state._apply_h(qubit),
        }
    }

    /// Applies an S gate to the specified qubit.
    pub fn apply_s(&mut self, qubit: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state._apply_s(qubit),
        }
    }

    /// Applies an Sdg gate to the specified qubit.
    pub fn apply_sdg(&mut self, qubit: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state._apply_sdg(qubit),
        }
    }

    /// Applies a SqrtX gate to the specified qubit.
    pub fn apply_sqrt_x(&mut self, qubit: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state._apply_sqrt_x(qubit),
        }
    }

    /// Applies a SqrtXdg gate to the specified qubit.
    pub fn apply_sqrt_xdg(&mut self, qubit: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state._apply_sqrt_xdg(qubit),
        }
    }

    /// Applies a CX (CNOT) gate.
    pub fn apply_cx(&mut self, control: usize, target: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => {
                state._apply_cx(control, target)
            }
        }
    }

    /// Applies a CZ gate.
    pub fn apply_cz(&mut self, qarg1: usize, qarg2: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state._apply_cz(qarg1, qarg2),
        }
    }

    /// Applies a SWAP gate.
    pub fn apply_swap(&mut self, qarg1: usize, qarg2: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => {
                state._apply_swap(qarg1, qarg2)
            }
        }
    }

    // ===== Properties =====

    /// Returns the number of qubits in the quantum state.
    ///
    /// ### Returns
    /// * `usize` - The number of qubits.
    pub fn num_qubits(&self) -> usize {
        match &self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.num_qubits,
        }
    }

    /// Returns the stabilizer rank (the number of stabilizer states in the decomposition)
    /// of the internal stabilizer decomposed state.
    ///
    /// ### Returns
    /// * `usize` - The stabilizer rank.
    pub fn stabilizer_rank(&self) -> usize {
        match &self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.stabilizers.len(),
        }
    }

    /// Returns the norm of the state.
    ///
    /// ### Returns
    /// * `f64` - The norm of the state, which should be 1.0 for a valid quantum state.
    pub fn norm(&self) -> Result<f64> {
        match &self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state._norm(),
        }
    }
}
