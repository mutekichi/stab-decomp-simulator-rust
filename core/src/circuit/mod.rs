mod gates;
mod parser;
mod random_clifford;

pub use gates::QuantumGate;

use crate::error::Result;
use std::{fmt, path::Path};

/// Represents a quantum circuit as a sequence of quantum gates.
///
/// A [`QuantumCircuit`] acts as a blueprint for a quantum computation.
///
/// This struct is the primary entry point for defining a computation. Once built, it is typically
/// compiled into a [`QuantumState`](crate::state::QuantumState) via [`QuantumState::from_circuit`](crate::state::QuantumState::from_circuit) to be simulated.
///
/// ## Examples
///
/// ```rust
/// use necstar::prelude::{QuantumCircuit, QuantumState};
/// use necstar::types::PauliString;
/// use std::str::FromStr;
///
/// // Create a circuit
/// let mut circuit = QuantumCircuit::new(2);
/// circuit.apply_h(0);
/// circuit.apply_t(0);
/// circuit.apply_cx(0, 1);
///
/// // Compile the circuit to a state for simulation
/// let mut state = QuantumState::from_circuit(&circuit).unwrap();
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
/// let expectation = state.exp_value(&pauli_str).unwrap();
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
    /// ## Arguments
    /// * `gate` - The quantum gate to apply.
    /// ## Example
    /// ```rust
    /// use necstar::circuit::QuantumGate;
    /// use necstar::prelude::QuantumCircuit;
    /// let mut circuit = QuantumCircuit::new(2);
    /// circuit.apply_gate(QuantumGate::H(0));
    /// circuit.apply_gate(QuantumGate::CX(0, 1));
    /// assert_eq!(circuit.gates[0], QuantumGate::H(0));
    /// assert_eq!(circuit.gates[1], QuantumGate::CX(0, 1));
    /// ```
    pub fn apply_gate(&mut self, gate: QuantumGate) {
        self.gates.push(gate);
    }

    /// Apply a sequence of quantum gates to the circuit.
    ///
    /// ## Arguments
    /// * `gates` - A slice of quantum gates to apply.
    /// ## Example
    /// ```rust
    /// use necstar::circuit::QuantumGate;
    /// use necstar::prelude::QuantumCircuit;
    /// let mut circuit = QuantumCircuit::new(2);
    /// let gates = vec![QuantumGate::H(0), QuantumGate::CX(0, 1)];
    /// circuit.apply_gates(&gates);
    /// assert_eq!(circuit.gates[0], QuantumGate::H(0));
    /// assert_eq!(circuit.gates[1], QuantumGate::CX(0, 1));
    /// ```
    pub fn apply_gates(&mut self, gates: &[QuantumGate]) {
        self.gates.extend_from_slice(gates);
    }

    /// Apply a Hadamard gate to the target qubit.
    /// ## Arguments
    /// * `target` - The target qubit index.
    pub fn apply_h(&mut self, target: usize) {
        self.apply_gate(QuantumGate::H(target));
    }

    /// Apply a Pauli-X gate to the target qubit.
    /// ## Arguments
    /// * `target` - The target qubit index.
    pub fn apply_x(&mut self, target: usize) {
        self.apply_gate(QuantumGate::X(target));
    }

    /// Apply a Pauli-Y gate to the target qubit.
    /// ## Arguments
    /// * `target` - The target qubit index.
    pub fn apply_y(&mut self, target: usize) {
        self.apply_gate(QuantumGate::Y(target));
    }

    /// Apply a Pauli-Z gate to the target qubit.
    /// ## Arguments
    /// * `target` - The target qubit index.
    pub fn apply_z(&mut self, target: usize) {
        self.apply_gate(QuantumGate::Z(target));
    }

    /// Apply an S gate to the target qubit.
    /// ## Arguments
    /// * `target` - The target qubit index.
    pub fn apply_s(&mut self, target: usize) {
        self.apply_gate(QuantumGate::S(target));
    }

    /// Apply an S-dagger gate to the target qubit.
    /// ## Arguments
    /// * `target` - The target qubit index.
    pub fn apply_sdg(&mut self, target: usize) {
        self.apply_gate(QuantumGate::Sdg(target));
    }

    /// Apply a square root of X gate to the target qubit.
    /// ## Arguments
    /// * `target` - The target qubit index.
    pub fn apply_sqrt_x(&mut self, target: usize) {
        self.apply_gate(QuantumGate::SqrtX(target));
    }

    /// Apply a square root of X-dagger gate to the target qubit.
    /// ## Arguments
    /// * `target` - The target qubit index.
    pub fn apply_sqrt_xdg(&mut self, target: usize) {
        self.apply_gate(QuantumGate::SqrtXdg(target));
    }

    /// Apply a CNOT gate with the specified control and target qubits.
    /// ## Arguments
    /// * `control` - The control qubit index.
    /// * `target` - The target qubit index.
    pub fn apply_cx(&mut self, control: usize, target: usize) {
        self.apply_gate(QuantumGate::CX(control, target));
    }

    /// Apply a CZ gate with the specified qubits.
    /// ## Arguments
    /// * `qarg1` - The first qubit index.
    /// * `qarg2` - The second qubit index.
    pub fn apply_cz(&mut self, qarg1: usize, qarg2: usize) {
        self.apply_gate(QuantumGate::CZ(qarg1, qarg2));
    }

    /// Apply a SWAP gate with the specified qubits.
    /// ## Arguments
    /// * `qarg1` - The first qubit index.
    /// * `qarg2` - The second qubit index.
    pub fn apply_swap(&mut self, qarg1: usize, qarg2: usize) {
        self.apply_gate(QuantumGate::Swap(qarg1, qarg2));
    }

    // *** Single-Qubit Non-Clifford Gates ***

    /// Apply a T gate to the target qubit.
    /// ## Arguments
    /// * `target` - The target qubit index.
    pub fn apply_t(&mut self, target: usize) {
        self.apply_gate(QuantumGate::T(target));
    }

    /// Apply a T-dagger gate to the target qubit.
    /// ## Arguments
    /// * `target` - The target qubit index.
    pub fn apply_tdg(&mut self, target: usize) {
        self.apply_gate(QuantumGate::Tdg(target));
    }

    /// Apply a Toffoli (CCX) gate with the specified control and target qubits.
    /// ## Arguments
    /// * `control1` - The first control qubit index.
    /// * `control2` - The second control qubit index.
    /// * `target` - The target qubit index.
    pub fn apply_ccx(&mut self, control1: usize, control2: usize, target: usize) {
        self.apply_gate(QuantumGate::CCX(control1, control2, target));
    }

    /// Appends the gates from another [`QuantumCircuit`] to this one.
    ///
    /// ## Arguments
    /// - `other`: A reference to another [`QuantumCircuit`] whose gates will be appended.
    /// ## Example
    /// ```rust
    /// use necstar::prelude::QuantumCircuit;
    /// use necstar::circuit::QuantumGate;
    /// let mut circuit1 = QuantumCircuit::new(2);
    /// circuit1.apply_h(0);
    /// let mut circuit2 = QuantumCircuit::new(2);
    /// circuit2.apply_cx(0, 1);
    /// circuit1.append(&circuit2);
    /// assert_eq!(circuit1.num_qubits, 2);
    /// assert_eq!(circuit1.gates.len(), 2);
    /// assert_eq!(circuit1.gates[0], QuantumGate::H(0));
    /// assert_eq!(circuit1.gates[1], QuantumGate::CX(0, 1));
    /// ```
    pub fn append(&mut self, other: &QuantumCircuit) {
        self.gates.extend_from_slice(&other.gates);
    }

    /// Creates a new circuit by taking the tensor product of this circuit and another.
    ///
    /// The new circuit will have `self.num_qubits() + other.num_qubits()` qubits.
    /// Gates from `self` are applied to the first qubits, and gates from `other`
    /// are applied to the subsequent qubits.
    /// ## Arguments
    /// - `other`: A reference to another [`QuantumCircuit`] to tensor with.  
    /// ## Example
    /// ```rust
    /// use necstar::prelude::QuantumCircuit;
    /// use necstar::circuit::QuantumGate;
    /// let mut circuit1 = QuantumCircuit::new(1);
    /// circuit1.apply_h(0);
    /// let mut circuit2 = QuantumCircuit::new(2);
    /// circuit2.apply_cx(0, 1);
    /// let tensor_circuit = circuit1.tensor(&circuit2);
    /// assert_eq!(tensor_circuit.num_qubits, 3);
    /// assert_eq!(tensor_circuit.gates.len(), 2);
    /// assert_eq!(tensor_circuit.gates[0], QuantumGate::H(0));
    /// assert_eq!(tensor_circuit.gates[1], QuantumGate::CX(1, 2));
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

    /// Generates a uniformly random n-qubit Clifford circuit.
    ///
    /// This function implements the O(n^2) algorithm described in the paper to sample a Clifford
    /// operator uniformly at random from the n-qubit Clifford group.
    /// The resulting circuit is structured according to the canonical form U = F1 * H * S * F2.
    /// See the reference for details.
    ///
    /// ## Arguments
    /// * `n` - The number of qubits. Must be greater than 0.
    /// * `seed` - An optional seed for the random number generator for reproducibility.
    ///   If `None` is provided, a seed will be generated from system entropy.
    ///
    /// ## Returns
    /// A [`QuantumCircuit`] object representing the random Clifford operator.
    ///
    /// ## Reference
    /// - S. Bravyi and D. Maslov, "Hadamard-free circuits expose the structure of the Clifford
    ///   group," IEEE Trans. Inf. Theory 67, 5800 (2021).
    ///   <https://doi.org/10.1109/TIT.2021.3081415>
    pub fn random_clifford(n: usize, seed: Option<[u8; 32]>) -> QuantumCircuit {
        random_clifford::random_clifford(n, seed)
    }

    /// Parses an OpenQASM 2.0 string into a [`QuantumCircuit`].
    ///
    /// ## Arguments
    /// * `qasm_str` - A string slice containing the OpenQASM 2.0 circuit description.
    pub fn from_qasm_str(qasm_str: &str) -> Result<Self> {
        parser::from_qasm_str(qasm_str)
    }

    /// Parses an OpenQASM 2.0 file into a [`QuantumCircuit`]
    ///
    /// ## Arguments
    /// * `path` - A path to the QASM file.
    pub fn from_qasm_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        parser::from_qasm_file(path)
    }

    /// Converts the circuit to an OpenQASM 2.0 string.
    ///
    /// ## Arguments
    /// * `reg_name` - The name of the quantum register (e.g., "q").
    pub fn to_qasm_str(&self, reg_name: &str) -> String {
        parser::to_qasm_str(self, reg_name)
    }

    /// Writes the circuit to an OpenQASM 2.0 file.
    ///
    /// ## Arguments
    /// * `path` - The path to the output file.
    /// * `reg_name` - The name of the quantum register (e.g., "q").
    pub fn to_qasm_file<P: AsRef<Path>>(&self, path: P, reg_name: &str) -> Result<()> {
        parser::to_qasm_file(self, path, reg_name)
    }
}

impl fmt::Display for QuantumCircuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "QuantumCircuit(num_qubits={}) [", self.num_qubits)?;

        for (i, gate) in self.gates.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", gate)?;
        }

        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_circuit() {
        let mut circuit1 = QuantumCircuit::new(2);
        circuit1.apply_h(0);
        let mut circuit2 = QuantumCircuit::new(2);
        circuit2.apply_cx(0, 1);
        circuit2.apply_t(0);

        circuit1.append(&circuit2);

        assert_eq!(circuit1.num_qubits, 2);
        assert_eq!(circuit1.gates.len(), 3);
        assert_eq!(circuit1.gates[0], QuantumGate::H(0));
        assert_eq!(circuit1.gates[1], QuantumGate::CX(0, 1));
        assert_eq!(circuit1.gates[2], QuantumGate::T(0));
    }

    #[test]
    fn test_tensor_circuit() {
        let mut circuit1 = QuantumCircuit::new(2);
        circuit1.apply_h(0);
        let mut circuit2 = QuantumCircuit::new(3);
        circuit2.apply_cx(0, 1);
        circuit2.apply_t(2);

        let tensor_circuit = circuit1.tensor(&circuit2);

        assert_eq!(tensor_circuit.num_qubits, 5);
        assert_eq!(tensor_circuit.gates.len(), 3);
        assert_eq!(tensor_circuit.gates[0], QuantumGate::H(0));
        assert_eq!(tensor_circuit.gates[1], QuantumGate::CX(2, 3));
        assert_eq!(tensor_circuit.gates[2], QuantumGate::T(4));
    }

    #[test]
    fn test_quantum_circuit_display() {
        let mut circuit = QuantumCircuit::new(2);
        circuit.apply_x(0);
        circuit.apply_cz(0, 1);
        circuit.apply_tdg(1);

        let display_str = format!("{}", circuit);
        let expected_str = "QuantumCircuit(num_qubits=2) [X(0), CZ(0, 1), Tdg(1)]";
        assert_eq!(display_str, expected_str);
    }
}
