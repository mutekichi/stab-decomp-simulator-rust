/// Represents a quantum circuit as a sequence of quantum gates.
///
/// A [`QuantumCircuit`] acts as a blueprint for a quantum computation. It holds the number of qubits
/// and an ordered list of [`QuantumGate`] operations to be applied.
///
/// This struct is the primary entry point for defining a computation. Once built, it is typically
/// compiled into a [`QuantumState`](crate::state::QuantumState) via [`QuantumState::from_circuit`](crate::state::QuantumState::from_circuit) to be simulated.
///
/// ## Examples
///
/// ```rust
/// use stab_decomp_simulator_rust::prelude::{QuantumCircuit, QuantumState};
/// use stab_decomp_simulator_rust::types::PauliString;
/// use std::str::FromStr;
///
/// // Create a circuit
/// let mut circuit = QuantumCircuit::new(2);
/// circuit.apply_h(0);
/// circuit.apply_t(0);
/// circuit.apply_cx(0, 1);
///
/// // Compile the circuit to a state for simulation
/// let state = QuantumState::from_circuit(&bell_circuit).unwrap();
///
/// // Sample measurement outcomes
/// let shots = 1024;
/// let qargs = vec![0, 1];
/// let seed = None;
/// let shot_count = state.sample(&qargs, shots, seed).unwrap();
/// for (outcome, count) in shot_count.iter() {
///     println!("{:?}: {}", outcome, count);
/// }
///
/// // Calculate an expectation value
/// let pauli_str = PauliString::from_str("ZI").unwrap();
/// let expectation = state.expectation_value(&pauli_str).unwrap();
/// println!("Expectation value of {}: {}", pauli_str, expectation);
///
/// // Apply a Clifford gate directly to the state
/// state.apply_h(1).unwrap();
///
/// // Get the stabilizer rank χ
/// println!("Stabilizer rank: {}", state.stabilizer_rank());
/// ```
pub struct QuantumCircuit {
    pub num_qubits: usize,
    pub gates: Vec<QuantumGate>,
}

impl QuantumCircuit {
    /// Creates a new quantum circuit
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            gates: Vec::new(),
        }
    }

    // Gate application methods

    /// Apply a quantum gate to the circuit.
    ///
    /// ### Arguments
    /// * `gate` - The quantum gate to apply.
    /// ### Example
    /// ```rust
    /// use stab_decomp_simulator_rust::circuit::QuantumGate;
    /// use stab_decomp_simulator_rust::prelude::QuantumCircuit;
    /// let mut circuit = QuantumCircuit::new(2);
    /// circuit.apply_gate(QuantumGate::H(0));
    /// circuit.apply_gate(QuantumGate::CX(0, 1));
    /// ```
    pub fn apply_gate(&mut self, gate: QuantumGate) {
        self.gates.push(gate);
    }

    /// Apply a sequence of quantum gates to the circuit.
    ///
    /// ### Arguments
    /// * `gates` - A slice of quantum gates to apply.
    /// ### Example
    /// ```rust
    /// use stab_decomp_simulator_rust::circuit::QuantumGate;
    /// use stab_decomp_simulator_rust::prelude::QuantumCircuit;
    /// let mut circuit = QuantumCircuit::new(2);
    /// let gates = vec![QuantumGate::H(0), QuantumGate::CX(0, 1)];
    /// circuit.apply_gates(&gates);
    /// ```
    pub fn apply_gates(&mut self, gates: &[QuantumGate]) {
        self.gates.extend_from_slice(gates);
    }

    // *** Single-Qubit Clifford Gates ***

    /// Apply a Hadamard gate to the target qubit.
    /// ### Arguments
    /// * `target` - The target qubit index.
    pub fn apply_h(&mut self, target: usize) {
        self.apply_gate(QuantumGate::H(target));
    }

    /// Apply a Pauli-X gate to the target qubit.
    /// ### Arguments
    /// * `target` - The target qubit index.
    pub fn apply_x(&mut self, target: usize) {
        self.apply_gate(QuantumGate::X(target));
    }

    /// Apply a Pauli-Y gate to the target qubit.
    /// ### Arguments
    /// * `target` - The target qubit index.
    pub fn apply_y(&mut self, target: usize) {
        self.apply_gate(QuantumGate::Y(target));
    }

    /// Apply a Pauli-Z gate to the target qubit.
    /// ### Arguments
    /// * `target` - The target qubit index.
    pub fn apply_z(&mut self, target: usize) {
        self.apply_gate(QuantumGate::Z(target));
    }

    /// Apply an S gate to the target qubit.
    /// ### Arguments
    /// * `target` - The target qubit index.
    pub fn apply_s(&mut self, target: usize) {
        self.apply_gate(QuantumGate::S(target));
    }

    /// Apply an S-dagger gate to the target qubit.
    /// ### Arguments
    /// * `target` - The target qubit index.
    pub fn apply_sdg(&mut self, target: usize) {
        self.apply_gate(QuantumGate::Sdg(target));
    }

    /// Apply a square root of X gate to the target qubit.
    /// ### Arguments
    /// * `target` - The target qubit index.
    pub fn apply_sqrt_x(&mut self, target: usize) {
        self.apply_gate(QuantumGate::SqrtX(target));
    }

    /// Apply a square root of X-dagger gate to the target qubit.
    /// ### Arguments
    /// * `target` - The target qubit index.
    pub fn apply_sqrt_xdg(&mut self, target: usize) {
        self.apply_gate(QuantumGate::SqrtXdg(target));
    }

    // *** Two-Qubit Clifford Gates ***

    /// Apply a CNOT gate with the specified control and target qubits.
    /// ### Arguments
    /// * `control` - The control qubit index.
    /// * `target` - The target qubit index.
    pub fn apply_cx(&mut self, control: usize, target: usize) {
        self.apply_gate(QuantumGate::CX(control, target));
    }

    /// Apply a CZ gate with the specified qubits.
    /// ### Arguments
    /// * `qarg1` - The first qubit index.
    /// * `qarg2` - The second qubit index.
    pub fn apply_cz(&mut self, qarg1: usize, qarg2: usize) {
        self.apply_gate(QuantumGate::CZ(qarg1, qarg2));
    }

    /// Apply a SWAP gate with the specified qubits.
    /// ### Arguments
    /// * `qarg1` - The first qubit index.
    /// * `qarg2` - The second qubit index.
    pub fn apply_swap(&mut self, qarg1: usize, qarg2: usize) {
        self.apply_gate(QuantumGate::Swap(qarg1, qarg2));
    }

    // *** Single-Qubit Non-Clifford Gates ***

    /// Apply a T gate to the target qubit.
    /// ### Arguments
    /// * `target` - The target qubit index.
    pub fn apply_t(&mut self, target: usize) {
        self.apply_gate(QuantumGate::T(target));
    }

    /// Apply a T-dagger gate to the target qubit.
    /// ### Arguments
    /// * `target` - The target qubit index.
    pub fn apply_tdg(&mut self, target: usize) {
        self.apply_gate(QuantumGate::Tdg(target));
    }

    // *** Multi-Qubit Non-Clifford Gates ***
    /// Apply a Toffoli (CCX) gate with the specified control and target qubits.
    /// ### Arguments
    /// * `control1` - The first control qubit index.
    /// * `control2` - The second control qubit index.
    /// * `target` - The target qubit index.
    pub fn apply_ccx(&mut self, control1: usize, control2: usize, target: usize) {
        self.apply_gate(QuantumGate::CCX(control1, control2, target));
    }

    /// Appends the gates from another `QuantumCircuit` to this one.
    ///
    /// # Arguments
    /// - `other`: A reference to another `QuantumCircuit` whose gates will be appended.
    /// # Example
    /// ```rust
    /// use stab_decomp_simulator_rust::prelude::QuantumCircuit;
    /// let mut circuit1 = QuantumCircuit::new(2);
    /// circuit1.apply_h(0);
    /// let mut circuit2 = QuantumCircuit::new(2);
    /// circuit2.apply_cx(0, 1);
    /// circuit1.append(&circuit2);
    /// ```
    pub fn append(&mut self, other: &QuantumCircuit) {
        self.gates.extend_from_slice(&other.gates);
    }

    /// Creates a new circuit by taking the tensor product of this circuit and another.
    ///
    /// The new circuit will have `self.num_qubits() + other.num_qubits()` qubits.
    /// Gates from `self` are applied to the first qubits, and gates from `other`
    /// are applied to the subsequent qubits.
    /// # Arguments
    /// - `other`: A reference to another `QuantumCircuit` to tensor with.  
    /// # Example
    /// ```rust
    /// use stab_decomp_simulator_rust::prelude::QuantumCircuit;
    /// let mut circuit1 = QuantumCircuit::new(1);
    /// circuit1.apply_h(0);
    /// let mut circuit2 = QuantumCircuit::new(1);
    /// circuit2.apply_x(0);
    /// let tensor_circuit = circuit1.tensor(&circuit2);
    /// ```
    pub fn tensor(&self, other: &QuantumCircuit) -> QuantumCircuit {
        let mut new_circuit = QuantumCircuit::new(self.num_qubits + other.num_qubits);

        // Add gates from the first circuit
        for gate in &self.gates {
            new_circuit.gates.push(gate.clone());
        }

        // Add gates from the second circuit, adjusting qubit indices
        let offset = self.num_qubits;
        for gate in &other.gates {
            new_circuit.gates.push(gate.clone().shifted(offset));
        }

        new_circuit
    }
}

pub mod gates;
pub mod parser;

pub use gates::QuantumGate;
pub use parser::{from_qasm_file, from_qasm_str};
