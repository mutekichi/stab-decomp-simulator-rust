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

/// The primary interface for simulating and analyzing quantum states.
///
/// `QuantumState` represents a quantum state using the stabilizer decomposition method.
/// It acts as the central entity for state analysis, such as calculating expectation values
/// and inner products. Internally, a defined [`QuantumCircuit`] is automatically
/// compiled into a stabilizer decomposition representation upon creation.
///
/// ## Typical Workflow
///
/// 1. Construct a quantum circuit using [`QuantumCircuit`].
/// 2. Compile the circuit into a [`QuantumState`] using [`QuantumState::from_circuit`].
/// 3. Perform operations such as [`measure`](Self::measure), [`sample`](Self::sample), or
///    [`exp_value`](Self::exp_value) (expectation value calculation).
/// 4. If needed, additional Clifford gates can be applied directly to the state using methods like
///    [`apply_x`](Self::apply_x), [`apply_h`](Self::apply_h), etc.
///
/// ## Examples
///
/// ```rust
/// use necstar::prelude::{QuantumCircuit, QuantumState};
/// use necstar::types::PauliString;
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
/// // (optional) Apply a gate directly to the state
/// state.apply_x(0).unwrap();
///
/// // 3. Perform operations on the state
/// // - Sample measurement outcomes
/// let shots = 1024;
/// let samples = state.sample(&[0, 1], shots, None).unwrap();
/// println!("Measurement samples: {:?}", samples);
///
/// // - Calculate an expectation value
/// let pauli_z0 = PauliString::from_str("ZI").unwrap();
/// let exp_val = state.exp_value(&pauli_z0).unwrap();
/// println!("Expectation value of Z on qubit 0: {}", exp_val);
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
    /// Creates a new [`QuantumState`] by compiling a [`QuantumCircuit`].
    /// ## Examples
    /// ```rust
    /// use necstar::prelude::{QuantumCircuit, QuantumState};
    ///
    /// let mut circuit = QuantumCircuit::new(2);
    /// circuit.apply_h(0);
    /// circuit.apply_cx(0, 1);
    /// circuit.apply_t(0);
    ///
    /// let state = QuantumState::from_circuit(&circuit).unwrap();
    /// ```
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
    /// statevector of size `2^n`, which can be computationally expensive and memory-intensive for a
    /// large number of qubits (`n`). This approach deviates from the core strength of the
    /// stabilizer decomposition simulator, which is designed to efficiently handle systems with
    /// large `n` by avoiding this explicit statevector representation.
    ///
    /// ## Examples
    /// ```rust
    /// use necstar::prelude::{QuantumCircuit, QuantumState};
    /// use num_complex::Complex64;
    ///
    /// let mut circuit = QuantumCircuit::new(2);
    /// circuit.apply_h(0);
    /// circuit.apply_cx(0, 1);
    /// circuit.apply_t(0);
    ///
    /// let state = QuantumState::from_circuit(&circuit).unwrap();
    /// let statevector = state.to_statevector().unwrap();
    ///
    /// assert!((statevector[0] - num_complex::Complex64::new(0.70710678, 0.0)).norm() < 1e-6);
    /// assert!(statevector[1].norm() < 1e-6);
    /// assert!(statevector[2].norm() < 1e-6);
    /// assert!((statevector[3] - num_complex::Complex64::new(0.5, 0.5)).norm() < 1e-6);
    /// ```
    ///
    /// ## Returns
    /// A [`Result`] containing the statevector as an `Array1<Complex64>` or an
    /// [`Error`](crate::error::Error).
    pub fn to_statevector(&self) -> Result<Array1<num_complex::Complex64>> {
        match &self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.to_statevector(),
        }
    }

    /// Returns the inner product of the state and another state, i.e. ⟨self|other⟩.
    ///
    /// ## Examples
    /// ```rust
    /// use necstar::prelude::{QuantumCircuit, QuantumState};
    /// use num_complex::Complex64;
    ///
    /// let mut circuit1 = QuantumCircuit::new(1);
    /// circuit1.apply_h(0);
    /// let state1 = QuantumState::from_circuit(&circuit1).unwrap();
    ///
    /// let mut circuit2 = QuantumCircuit::new(1);
    /// circuit2.apply_x(0);
    /// let state2 = QuantumState::from_circuit(&circuit2).unwrap();
    ///
    /// let inner_prod = state1.inner_product(&state2).unwrap();
    /// assert!((inner_prod - Complex64::new(0.70710678, 0.0)).norm() < 1e-6);
    /// ```
    ///
    /// ## Arguments
    /// * `other` - A reference to another [`QuantumState`] to compute the inner product with.
    ///
    /// ## Returns
    /// A [`Result`] containing the inner product as `Complex64` or an
    /// [`Error`](crate::error::Error).
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
    /// ## Examples
    /// ```rust
    /// use necstar::prelude::{QuantumCircuit, QuantumState};
    ///
    /// let mut circuit = QuantumCircuit::new(2);
    /// circuit.apply_h(0);
    /// circuit.apply_cx(0, 1);
    /// let mut state = QuantumState::from_circuit(&circuit).unwrap(); // Bell state
    ///
    /// let result = state.measure(&[0, 1], Some([42; 32])).unwrap();
    /// // For the Bell state, the possible outcomes are |00> or |11>
    /// assert!(result == vec![false, false] || result == vec![true, true]);
    /// ```
    ///
    /// ## Arguments
    /// * `qargs` - A slice of qubit indices to measure.
    /// * `seed` - An optional seed for the random number generator to ensure reproducibility.
    ///   If `None` is provided, a seed will be generated from system entropy.
    ///
    /// ## Returns
    /// A [`Result`] containing a vector of boolean measurement results or an
    /// [`Error`](crate::error::Error). The length of the vector corresponds to `qargs.len()`.
    /// The `i`-th element in the vector corresponds to the result of the qubit specified by
    /// `qargs[i]`. `false` represents the `|0>` outcome, and `true` represents the `|1>` outcome.
    pub fn measure(&mut self, qargs: &[usize], seed: Option<[u8; 32]>) -> Result<Vec<bool>> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.measure(qargs, seed),
        }
    }

    /// Measure all qubits in the computational basis and return the measurement results.
    /// The state gets collapsed according to the measurement results.
    ///
    /// ## Examples
    /// ```rust
    /// use necstar::prelude::{QuantumCircuit, QuantumState};
    ///
    /// let mut circuit = QuantumCircuit::new(2);
    /// circuit.apply_h(0);
    /// circuit.apply_cx(0, 1);
    /// let mut state = QuantumState::from_circuit(&circuit).unwrap(); // Bell state
    ///
    /// let result = state.measure_all(Some([42; 32])).unwrap();
    /// // For the Bell state, the possible outcomes are |00> or |11>
    /// assert!(result == vec![false, false] || result == vec![true, true]);
    /// ```
    /// ## Arguments
    /// * `seed` - An optional seed for the random number generator to ensure reproducibility.
    ///   If `None` is provided, a seed will be generated from system entropy.
    ///
    /// ## Returns
    /// A [`Result`] containing a vector of boolean measurement results or an
    /// [`Error`](crate::error::Error). The length of the vector corresponds to the number of qubits
    /// in the state. The `i`-th element in the vector corresponds to the result of the qubit
    /// specified by index `i`. `false` represents the `|0>` outcome, and `true` represents the
    /// `|1>` outcome.
    pub fn measure_all(&mut self, seed: Option<[u8; 32]>) -> Result<Vec<bool>> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.measure_all(seed),
        }
    }

    /// Samples measurement outcomes for the specified qubits without collapsing the quantum state.
    ///
    /// ## Examples
    /// ```rust
    /// use necstar::prelude::{QuantumCircuit, QuantumState};
    ///
    /// let mut circuit = QuantumCircuit::new(2);
    /// circuit.apply_h(0);
    /// circuit.apply_cx(0, 1);
    /// let state = QuantumState::from_circuit(&circuit).unwrap();
    ///
    /// let shots = 1000;
    /// let samples = state.sample(&[0, 1], shots, Some([42; 32])).unwrap();
    /// // For the Bell state, the possible outcomes are |00> or |11>
    /// assert!(samples.iter().all(|(outcome, _count)|
    ///     outcome == &vec![false, false] || outcome == &vec![true, true]
    /// ));
    /// ```
    ///
    /// ## Arguments
    /// * `qargs` - A slice of qubit indices to sample.
    /// * `shots` - The number of measurement samples to generate.
    /// * `seed` - An optional seed for the random number generator to ensure reproducibility.
    ///   If `None` is provided, a seed will be generated from system entropy.
    ///
    /// ## Returns
    /// A [`Result`] containing a [`ShotCount`], which is a vector of tuples.
    /// Each tuple consists of:
    /// 1. `Vec<bool>`: A unique measurement outcome. The `i`-th element
    ///    corresponds to the qubit at `qargs[i]`, where `false` for `|0>` and `true` for `|1>`.
    /// 2. `usize`: The frequency (count) of this specific outcome across the total `shots`.
    ///
    /// The sum of all `usize` values in the returned vector equals `shots`. Note that it is not
    /// supported to sample more than 128 qubits at once due to internal representation limits.
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
    /// ## Examples
    /// ```rust
    /// use necstar::prelude::{QuantumCircuit, QuantumState};
    /// use necstar::types::PauliString;
    /// use std::str::FromStr;
    ///
    /// let mut circuit = QuantumCircuit::new(2);
    /// circuit.apply_h(0);
    /// circuit.apply_cx(0, 1);
    /// let state = QuantumState::from_circuit(&circuit).unwrap();
    ///
    /// let observable = PauliString::from_str("ZZ").unwrap();
    /// let exp_val = state.exp_value(&observable).unwrap();
    /// assert!((exp_val - 1.0).abs() < 1e-6);
    /// ```
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
    /// This operation is equivalent to a projective measurement in the Z-basis. The state is
    /// modified in place. If the projection is impossible (e.g., projecting a definite `|0>` state
    /// onto `|1>`), an error is returned. The resulting state after successful projection is
    /// normalized to have a total norm of 1. If the projection fails, the behavior of the state is
    /// undefined.
    ///
    /// ## Examples
    /// ```rust
    /// use necstar::prelude::{QuantumCircuit, QuantumState};
    ///
    /// let mut circuit = QuantumCircuit::new(2);
    /// circuit.apply_h(0);
    /// circuit.apply_cx(0, 1);
    /// let mut state = QuantumState::from_circuit(&circuit).unwrap(); // Bell state
    ///
    /// state.project_normalized(0, false).unwrap();
    /// assert!((state.norm().unwrap() - 1.0).abs() < 1e-6);
    ///
    /// let statevector = state.to_statevector().unwrap();
    /// assert!((statevector[0] - 1.0).norm() < 1e-6); // |00>
    /// ```
    ///
    /// ## Arguments
    /// * `qubit` - The index of the qubit to project.
    /// * `outcome` - The desired computational basis state to project onto: `false` for `|0>`
    ///   (the +1 eigenspace of Pauli Z) and `true` for `|1>` (the -1 eigenspace of Pauli Z).
    ///
    /// ## Returns
    /// A [`Result`] which is `Ok(())` on success, or an [`Error`](crate::error::Error) if the
    /// projection is impossible.
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
    /// will generally not equal 1. This method is useful for intermediate steps in algorithms
    /// like sampling, where the normalization can be deferred.
    ///
    /// ## Examples
    /// ```rust
    /// use necstar::prelude::{QuantumCircuit, QuantumState};
    ///
    /// let mut circuit = QuantumCircuit::new(2);
    /// circuit.apply_h(0);
    /// circuit.apply_cx(0, 1);
    /// let mut state = QuantumState::from_circuit(&circuit).unwrap(); // Bell state
    ///
    /// state.project_unnormalized(0, false).unwrap(); // Project qubit 0 onto |0>
    /// let statevector = state.to_statevector().unwrap();
    ///
    /// // The norm is not 1 after unnormalized projection
    /// assert!((state.norm().unwrap() - 0.70710678).abs() < 1e-6);
    ///
    /// // You can sample from the unnormalized state
    /// let shots = 1000;
    /// let samples = state.sample(&[0, 1], shots, Some([42; 32])).unwrap();
    /// assert!(samples.iter().all(|(outcome, _count)|
    ///    outcome == &vec![false, false]
    /// ));
    /// ```
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
    /// ## Argument
    /// * `qubit` - The index of the qubit to project.
    /// * `outcome` - The desired computational basis state to project onto: `false` for `|0>` and
    ///   `true` for `|1>`.
    ///
    /// ## Returns
    /// A [`Result`] which is `Ok(())` on success. Unlike
    /// [`project_normalized`](Self::project_normalized), this function will not return an error
    /// even if the projection results in a zero-norm state.
    pub fn project_unnormalized(&mut self, qubit: usize, outcome: bool) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => {
                state.project_unnormalized(qubit, outcome)
            }
        }
    }

    /// Removes a qubit from the quantum state, reducing the system size.
    ///
    /// This operation decreases the total number of qubits by one and modifies the
    /// state in place.
    ///
    /// ## Important
    ///
    /// This function **must** only be called on a qubit that has been projected to the `|0>` state
    /// and is disentangled from all other qubits. The behavior is undefined if this precondition is
    /// not met.
    ///
    /// For performance reasons, this function does not verify the qubit's state before discarding
    /// it. The caller is responsible for ensuring this precondition is met, for example, by using
    /// [`project_normalized`](Self::project_normalized) beforehand.
    ///
    /// ## Examples
    /// ```rust
    /// use necstar::prelude::{QuantumCircuit, QuantumState};
    ///
    /// let mut circuit = QuantumCircuit::new(2);
    /// circuit.apply_h(0);
    /// circuit.apply_cx(0, 1);
    /// circuit.apply_t(0);
    /// let mut state = QuantumState::from_circuit(&circuit).unwrap();
    ///
    /// state.project_normalized(0, false).unwrap(); // Project qubit 0 onto |0>
    /// state.discard(0).unwrap(); // Discard qubit 0
    ///
    /// assert_eq!(state.num_qubits(), 1);
    /// let statevector = state.to_statevector().unwrap();
    /// assert!((statevector[0] - 1.0).norm() < 1e-6); // |0>
    /// ```
    ///
    /// ## Arguments
    /// * `qubit` - The index of the qubit to discard.
    ///
    /// ## Returns
    /// A [`Result`] which is `Ok(())` on success, or an [`Error`](crate::error::Error).
    pub fn discard(&mut self, qubit: usize) -> Result<()> {
        match &mut self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.discard(qubit),
        }
    }

    // ===== Gate Applications =====

    /// Applies a [`QuantumGate`] to the quantum state.
    /// Note: Only Clifford gates are supported for direct application.
    ///
    /// ## Examples
    /// ```rust
    /// use necstar::prelude::{QuantumCircuit, QuantumState, QuantumGate};
    ///
    /// let mut circuit = QuantumCircuit::new(2);
    /// circuit.apply_h(0);
    ///
    /// let mut state = QuantumState::from_circuit(&circuit).unwrap();
    /// let gate = QuantumGate::CX(0, 1);
    /// state.apply_gate(&gate).unwrap();
    ///
    /// let statevector = state.to_statevector().unwrap();
    /// assert!((statevector[0] - 0.70710678).norm() < 1e-6);
    /// assert!(statevector[1].norm() < 1e-6);
    /// assert!(statevector[2].norm() < 1e-6);
    /// assert!((statevector[3] - 0.70710678).norm() < 1e-6);
    /// ```
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
    /// ## Examples
    /// ```rust
    /// use necstar::prelude::{QuantumCircuit, QuantumState, QuantumGate};
    /// use num_complex::Complex64;
    ///
    /// let mut circuit = QuantumCircuit::new(2);
    /// circuit.apply_h(0);
    ///
    /// let mut state = QuantumState::from_circuit(&circuit).unwrap();
    ///
    /// let gates = vec![
    ///     QuantumGate::CX(0, 1),
    ///     QuantumGate::S(1),
    /// ];
    /// state.apply_gates(&gates).unwrap();
    ///
    /// let statevector = state.to_statevector().unwrap();
    /// assert!((statevector[0] - Complex64::new(0.70710678, 0.0)).norm() < 1e-6);
    /// assert!(statevector[1].norm() < 1e-6);
    /// assert!(statevector[2].norm() < 1e-6);
    /// assert!((statevector[3] - Complex64::new(0.0, 0.70710678)).norm() < 1e-6);
    /// ```
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
    /// * `f64` - The norm of the state, which should be 1.0 for a valid normalized quantum state.
    pub fn norm(&self) -> Result<f64> {
        match &self.internal_state {
            InternalState::StabilizerDecomposedStateScalar(state) => state.norm(),
        }
    }
}
