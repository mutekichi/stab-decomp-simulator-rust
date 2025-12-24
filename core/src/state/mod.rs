pub(crate) mod compiler;
pub(crate) mod magic_states;
pub(crate) mod stabilizer_decomposed_state;
pub(crate) mod types;

use ndarray::Array1;
use stabilizer_ch_form_rust::types::pauli::PauliString;
pub(crate) use stabilizer_decomposed_state::StabilizerDecomposedState;
pub(crate) use types::coefficient::Coefficient;

use crate::{
    circuit::{QuantumCircuit, QuantumGate},
    error::Result,
    state::{
        compiler::{CircuitCompiler, StabDecompCompiler},
        types::scalar::Scalar,
    },
    types::shot_count::ShotCount,
};

/// Represents a simulated quantum state, providing the primary interface for quantum computation simulation.
///
/// `QuantumState` is the central struct for manipulating the results of a quantum circuit simulation.
/// It encapsulates the internal state representation (based on the stabilizer decomposition method)
/// and provides a high-level API for performing various quantum operations and analyses.
///
/// ## Features
///
/// This simulator utilizes the stabilizer decomposition method, which offers high performance,
/// especially for circuits dominated by Clifford gates.
/// Non-Clifford gates, such as the T-gate, are handled using gate teleportation.
/// The computational cost is measured by the number of stabilizer states: χ (`stabilizer_rank`) held internally.
///
/// ## Typical Workflow
///
/// 1. Construct a quantum circuit using [`QuantumCircuit`].
/// 2. Compile the circuit into a [`QuantumState`] using [`QuantumState::from_circuit`].
/// 3. Perform operations such as [`measure`](Self::measure), [`sample`](Self::sample), or [`exp_value`](Self::exp_value) (expectation value calculation).
/// 4. If needed, additional clifford gates can be applied directly to the state using methods like
///    [`apply_x`](Self::apply_x), [`apply_h`](Self::apply_h), etc.
///
/// # Examples
///
/// ```rust
/// use stab_decomp_simulator_rust::prelude::{QuantumCircuit, QuantumState};
/// use stab_decomp_simulator_rust::types::PauliString;
/// use std::str::FromStr;
///
/// // 1. Build a quantum circuit
/// let mut circuit = QuantumCircuit::new(2);
/// circuit.apply_h(0);
/// circuit.apply_cx(0, 1);
/// circuit.apply_t(1); // A non-Clifford gate
///
/// // 2. Compile the circuit into a QuantumState
/// let mut state = QuantumState::from_circuit(&circuit).unwrap();
///
/// // 3. Perform operations on the state
/// // Sample measurement outcomes
/// let shots = 1024;
/// let samples = state.sample(&[0, 1], shots, None).unwrap();
/// println!("Measurement samples: {:?}", samples);
///
/// // Calculate an expectation value
/// let pauli_z0 = PauliString::from_str("ZI").unwrap();
/// let exp_val = state.exp_value(&pauli_z0).unwrap();
/// println!("Expectation value of Z on qubit 0: {}", exp_val);
///
/// // 4. Apply a gate directly to the state
/// state.apply_x(0).unwrap();
///
/// // Get the stabilizer rank χ
/// println!("Stabilizer rank: {}", state.stabilizer_rank());
/// ```
pub struct QuantumState {
    internal_state: InternalState,
}

/// Internal representation of the quantum state.
/// Currently, only `StabilizerDecomposedState<Scalar>` is supported.
/// Future extensions may include other types like `StabilizerDecomposedState<Complex64>`
/// for Clifford gates other than T-gates.
pub(crate) enum InternalState {
    StabilizerDecomposedStateScalar(StabilizerDecomposedState<Scalar>),
}

impl QuantumState {
    // ===== Primary APIs =====

    /// Creates a new [`QuantumState`] by compiling a [`QuantumCircuit`].
    ///
    /// ## Arguments
    /// * `circuit` - A reference to the [`QuantumCircuit`] to be simulated.
    ///
    /// ## Returns
    /// A [`Result`] containing the compiled [`QuantumState`] or a [`Error`](crate::error::Error).
    pub fn from_circuit(circuit: &QuantumCircuit) -> Result<Self> {
        let compiler = StabDecompCompiler::new();
        let internal_state = compiler.compile(circuit)?;
        Ok(Self { internal_state })
    }

    /// Returns the statevector as an `Array1<Complex64>`.
    ///
    /// This function is primarily for testing and debugging purposes. It computes the full, dense
    /// statevector of size 2^n, which can be computationally expensive and memory-intensive for a large
    /// number of qubits (`n`). This approach deviates from the core strength of the stabilizer
    /// decomposition simulator, which is designed to efficiently handle systems with large `n`
    /// by avoiding this explicit statevector representation.
    ///
    /// ## Returns
    /// A [`Result`] containing the statevector as an `Array1<Complex64>` or an [`Error`](crate::error::Error).
    pub fn to_statevector(&self) -> Result<Array1<num_complex::Complex64>> {
        match &self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.to_statevector(),
        }
    }

    /// Returns the inner product of the state and another state, i.e. ⟨self|other⟩.
    ///
    /// ## Arguments
    /// * `other` - A reference to another [`QuantumState`] to compute the inner product with.
    ///
    /// ## Returns
    /// A [`Result`] containing the inner product as `Complex64` or an [`Error`](crate::error::Error).
    pub fn inner_product(&self, other: &Self) -> Result<num_complex::Complex64> {
        match (&self.internal_state, &other.internal_state) {
            (
                InternalState::StabilizerDecomposedStateScalar(state1),
                InternalState::StabilizerDecomposedStateScalar(state2),
            ) => state1.inner_product(state2),
        }
    }

    /// Measure the specified qubits in the computational basis and return the measurement results.
    /// The state gets collapsed according to the measurement results.
    ///
    /// ## Arguments
    /// * `qargs` - A slice of qubit indices to measure.
    /// * `seed` - An optional seed for the random number generator to ensure reproducibility.
    ///
    /// ## Returns
    /// A [`Result`] containing a vector of boolean measurement results or an [`Error`](crate::error::Error).
    pub fn measure(&mut self, qargs: &[usize], seed: Option<[u8; 32]>) -> Result<Vec<bool>> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.measure(qargs, seed),
        }
    }

    /// Measure the all qubits in the computational basis and return the measurement results.
    /// The state gets collapsed according to the measurement results.
    ///
    /// ## Arguments
    /// * `seed` - An optional seed for the random number generator to ensure reproducibility.
    ///
    /// ### Returns
    /// A [`Result`] containing a vector of boolean measurement results or an [`Error`](crate::error::Error).
    pub fn measure_all(&mut self, seed: Option<[u8; 32]>) -> Result<Vec<bool>> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.measure_all(seed),
        }
    }

    /// Samples measurement outcomes for the specified qubits without collapsing the quantum state.
    ///
    /// This method is designed for efficiently gathering measurement statistics. Unlike [`measure`](Self::measure),
    /// the internal state of the [`QuantumState`] is not modified by this operation, making it suitable
    /// for repeated analysis of the same state.
    ///
    /// The sampling process is performed recursively, applying the chain rule of probability one qubit at a time.
    /// For a given number of shots, a binomial distribution is used to statistically determine the outcomes for each qubit,
    /// avoiding the need to simulate each shot individually. This makes the process highly efficient, especially for a large number of shots.
    ///
    /// ## Arguments
    ///
    /// * `qargs` - A slice of qubit indices to sample.
    /// * `shots` - The number of measurement samples to generate.
    /// * `seed` - An optional seed for the random number generator to ensure reproducible results. If `None`, the generator is seeded from system entropy.
    ///
    /// ## Returns
    ///
    /// A [`Result`] containing a [`ShotCount`] or an [`Error`](crate::error::Error).
    pub fn sample(
        &self,
        qargs: &[usize],
        shots: usize,
        seed: Option<[u8; 32]>,
    ) -> Result<ShotCount> {
        match &self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => {
                state.sample(qargs, shots, seed)
            }
        }
    }

    /// Returns the expectation value of a given observable represented as a [`PauliString`].
    ///
    /// ## Arguments
    /// * `pauli_string` - A reference to a [`PauliString`] representing the observable.
    ///
    /// ## Returns
    /// A [`Result`] containing the expectation value as `f64` or an [`Error`](crate::error::Error).
    pub fn exp_value(&self, pauli_string: &PauliString) -> Result<f64> {
        match &self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.exp_value(pauli_string),
        }
    }

    /// Projects the state onto a computational basis state (`|0>` or `|1>`) for a specific qubit,
    /// then normalizes the entire quantum state.
    ///
    /// This operation is equivalent to a projective measurement in the Z-basis. The state is modified in place.
    /// If the projection is impossible (e.g., projecting a definite `|0>` state onto `|1>`), an error is returned.
    ///
    /// ## Arguments
    ///
    /// * `qubit` - The index of the qubit to project.
    /// * `outcome` - The desired computational basis state to project onto: `false` for `|0>` (the +1 eigenspace of Pauli Z)
    ///   and `true` for `|1>` (the -1 eigenspace of Pauli Z).
    ///
    /// ## Returns
    ///
    /// A [`Result`] which is `Ok(())` on success, or an [`Error`](crate::error::Error) if the projection is impossible.
    pub fn project_normalized(&mut self, qubit: usize, outcome: bool) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => {
                state.project_normalized(qubit, outcome)
            }
        }
    }

    #[cfg_attr(doc, katexit::katexit)]
    /// Projects the state onto a computational basis state (`|0>` or `|1>`) for a specific qubit,
    /// without normalizing the resulting state.
    ///
    /// The state is modified in place. After this operation, the total norm of the quantum state
    /// will generally not be 1. This method is useful for intermediate steps in algorithms
    /// like sampling, where the normalization can be deferred.
    ///
    /// The operation applies a projection operator `Π` to each stabilizer component `|ψ_i>`
    /// of the state `|φ> = Σ_i c_i |ψ_i>`. The projector for qubit `j` and outcome `o ∈ {0, 1}` is:
    /// $$
    /// \Pi_j^{(o)} = \frac{I + (-1)^o Z_j}{2}
    /// $$
    /// The resulting unnormalized state is:
    /// $$
    /// \Pi_j^{(o)}|\phi\rangle = \sum_i c_i (\Pi_j^{(o)}|\psi_i\rangle)
    /// $$
    ///
    /// ## Arguments
    ///
    /// * `qubit` - The index of the qubit to project.
    /// * `outcome` - The desired computational basis state to project onto: `false` for `|0>` and `true` for `|1>`.
    ///
    /// ## Returns
    ///
    /// A [`Result`] which is `Ok(())` on success. Unlike [`project_normalized`](Self::project_normalized),
    /// this function will not return an error even if the projection results in a zero-norm state.
    pub fn project_unnormalized(&mut self, qubit: usize, outcome: bool) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => {
                state.project_unnormalized(qubit, outcome)
            }
        }
    }

    /// Discards a qubit from the quantum state by tracing it out.
    ///
    /// This operation reduces the total number of qubits in the simulation by one and modifies the state in place.
    ///
    /// ## Important
    ///
    /// This function **must** only be called on a qubit that has been projected to the `|0>` state and is
    /// disentangled from all other qubits. Discarding a qubit that does not meet this condition will lead
    /// to incorrect simulation results.
    ///
    /// For performance reasons, this function does not verify the qubit's state before discarding it.
    /// The caller is responsible for ensuring this precondition is met, for example, by using
    /// [`project_normalized`](Self::project_normalized) beforehand.
    ///
    /// ## Arguments
    /// * `qubit` - The index of the qubit to discard.
    ///
    /// ## Returns
    /// A [`Result`] which is `Ok(())` on success, or an [`Error`](crate::error::Error) if the qubit index is out of bounds.
    pub fn discard(&mut self, qubit: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.discard(qubit),
        }
    }

    // ===== Gate Applications =====

    /// Applies a [`QuantumGate`] to the quantum state.
    /// Note: Only Clifford gates are supported for direct application.
    ///
    /// ## Arguments
    /// * `gate` - A reference to the [`QuantumGate`] to apply.
    ///
    /// ## Returns
    /// A [`Result`] which is `Ok(())` on success, or an [`Error`](crate::error::Error).
    pub fn apply_gate(&mut self, gate: &QuantumGate) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.apply_gate(gate),
        }
    }

    /// Applies a sequence of [`QuantumGate`]s to the quantum state.
    /// Note: Only Clifford gates are supported for direct application.
    ///
    /// ## Arguments
    /// * `gates` - A slice of [`QuantumGate`]s to apply.
    ///
    /// ## Returns
    /// A [`Result`] which is `Ok(())` on success, or an [`Error`](crate::error::Error).
    pub fn apply_gates(&mut self, gates: &[QuantumGate]) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.apply_gates(gates),
        }
    }

    /// Applies a Pauli-X gate to the specified qubit.
    /// Time complexity: `O(χn)`
    ///
    /// ## Arguments
    /// * `qubit` - The index of the qubit to apply the gate to.
    ///
    /// ## Returns
    /// A [`Result`] which is `Ok(())` on success, or an [`Error`](crate::error::Error).
    pub fn apply_x(&mut self, qubit: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.apply_x(qubit),
        }
    }

    /// Applies a Pauli-Y gate to the specified qubit.
    /// Time complexity: `O(χn)`
    ///
    /// ## Arguments
    /// * `qubit` - The index of the qubit to apply the gate to.
    /// ## Returns
    /// A [`Result`] which is `Ok(())` on success, or an [`Error`](crate::error::Error).
    pub fn apply_y(&mut self, qubit: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.apply_y(qubit),
        }
    }

    /// Applies a Pauli-Z gate to the specified qubit.
    /// Time complexity: `O(χ)`
    ///
    /// ## Arguments
    /// * `qubit` - The index of the qubit to apply the gate to.
    /// ## Returns
    /// A [`Result`] which is `Ok(())` on success, or an [`Error`](crate::error::Error).
    pub fn apply_z(&mut self, qubit: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.apply_z(qubit),
        }
    }

    /// Applies a Hadamard gate to the specified qubit.
    /// Time complexity: `O(χn^2)`
    ///
    /// ## Arguments
    /// * `qubit` - The index of the qubit to apply the gate to.
    /// ## Returns
    /// A [`Result`] which is `Ok(())` on success, or an [`Error`](crate::error::Error).
    pub fn apply_h(&mut self, qubit: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.apply_h(qubit),
        }
    }

    /// Applies an S gate to the specified qubit.
    /// Time complexity: `O(χn)`
    ///
    /// ## Arguments
    /// * `qubit` - The index of the qubit to apply the gate to.
    /// ## Returns
    /// A [`Result`] which is `Ok(())` on success, or an [`Error`](crate::error::Error).
    pub fn apply_s(&mut self, qubit: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.apply_s(qubit),
        }
    }

    /// Applies an Sdg gate to the specified qubit.
    /// Time complexity: `O(χn)`
    ///
    /// ## Arguments
    /// * `qubit` - The index of the qubit to apply the gate to.
    /// ## Returns
    /// A [`Result`] which is `Ok(())` on success, or an [`Error`](crate::error::Error).
    pub fn apply_sdg(&mut self, qubit: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.apply_sdg(qubit),
        }
    }

    /// Applies a SqrtX gate to the specified qubit.
    /// Time complexity: `O(χn^2)`
    ///
    /// ## Arguments
    /// * `qubit` - The index of the qubit to apply the gate to.
    /// ## Returns
    /// A [`Result`] which is `Ok(())` on success, or an [`Error`](crate::error::Error).
    pub fn apply_sqrt_x(&mut self, qubit: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.apply_sqrt_x(qubit),
        }
    }

    /// Applies a SqrtXdg gate to the specified qubit.
    /// Time complexity: `O(χn^2)`
    ///
    /// ## Arguments
    /// * `qubit` - The index of the qubit to apply the gate to.
    /// ## Returns
    /// A [`Result`] which is `Ok(())` on success, or an [`Error`](crate::error::Error).
    pub fn apply_sqrt_xdg(&mut self, qubit: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.apply_sqrt_xdg(qubit),
        }
    }

    /// Applies a CX (CNOT) gate.
    /// Time complexity: `O(χn)`
    ///
    /// ## Arguments
    /// * `control` - The index of the control qubit.
    /// * `target` - The index of the target qubit.
    /// ## Returns
    /// A [`Result`] which is `Ok(())` on success, or an [`Error`](crate::error::Error).
    pub fn apply_cx(&mut self, control: usize, target: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => {
                state.apply_cx(control, target)
            }
        }
    }

    /// Applies a CZ gate.
    /// Time complexity: `O(χn)`
    ///
    /// ## Arguments
    /// * `qarg1` - The index of the first qubit.
    /// * `qarg2` - The index of the second qubit.
    /// ## Returns
    /// A [`Result`] which is `Ok(())` on success, or an [`Error`](crate::error::Error).
    pub fn apply_cz(&mut self, qarg1: usize, qarg2: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.apply_cz(qarg1, qarg2),
        }
    }

    /// Applies a SWAP gate.
    /// Time complexity: `O(χn)`
    ///
    /// ## Arguments
    /// * `qarg1` - The index of the first qubit.
    /// * `qarg2` - The index of the second qubit.
    /// ## Returns
    /// A [`Result`] which is `Ok(())` on success, or an [`Error`](crate::error::Error).
    pub fn apply_swap(&mut self, qarg1: usize, qarg2: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.apply_swap(qarg1, qarg2),
        }
    }

    // ===== Properties =====

    /// Returns the number of qubits in the quantum state.
    ///
    /// ## Returns
    /// * `usize` - The number of qubits.
    pub fn num_qubits(&self) -> usize {
        match &self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.num_qubits,
        }
    }

    /// Returns the stabilizer rank χ (the number of stabilizer states in the decomposition)
    /// of the internal stabilizer decomposed state.
    ///
    /// ## Returns
    /// * `usize` - The stabilizer rank.
    pub fn stabilizer_rank(&self) -> usize {
        match &self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.stabilizers.len(),
        }
    }

    /// Returns the norm of the state.
    ///
    /// ## Returns
    /// * `f64` - The norm of the state, which should be 1.0 for a valid quantum state.
    pub fn norm(&self) -> Result<f64> {
        match &self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.norm(),
        }
    }
}
