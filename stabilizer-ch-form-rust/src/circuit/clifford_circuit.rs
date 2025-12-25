use crate::circuit::CliffordGate;
use crate::circuit::parser;
use crate::circuit::random_clifford;
use crate::error::Result;

/// A struct representing a Clifford circuit composed of Clifford gates.
/// `CliffordCircuit` only stores the sequence of gates and does not calculate
/// the resulting stabilizer state.
#[derive(Debug, Clone)]
pub struct CliffordCircuit {
    pub n_qubits: usize,
    pub gates: Vec<CliffordGate>,
}

impl CliffordCircuit {
    /// Creates a new Clifford circuit with the specified number of qubits.
    /// ## Arguments
    /// * `n_qubits` - The number of qubits in the circuit.
    pub fn new(n_qubits: usize) -> Self {
        CliffordCircuit {
            n_qubits,
            gates: Vec::new(),
        }
    }

    /// Adds a Clifford gate to the circuit.
    /// ## Arguments
    /// * `gate` - The Clifford gate to add.
    pub fn add_gate(&mut self, gate: CliffordGate) {
        self.gates.push(gate);
    }

    /// Adds multiple Clifford gates to the circuit.
    /// ## Arguments
    /// * `gates` - A vector of Clifford gates to add.
    pub fn add_gates(&mut self, gates: Vec<CliffordGate>) {
        for gate in gates {
            self.add_gate(gate);
        }
    }

    /// Applies a Hadamard gate to the specified qubit.
    /// ## Arguments
    /// * `qarg` - The index of the qubit to apply the gate to.
    pub fn apply_h(&mut self, qarg: usize) {
        self.add_gate(CliffordGate::H(qarg));
    }

    /// Applies a Pauli-X gate to the specified qubit.
    /// ## Arguments
    /// * `qarg` - The index of the qubit to apply the gate to.
    pub fn apply_x(&mut self, qarg: usize) {
        self.add_gate(CliffordGate::X(qarg));
    }

    /// Applies a Pauli-Y gate to the specified qubit.
    /// ## Arguments
    /// * `qarg` - The index of the qubit to apply the gate to.
    pub fn apply_y(&mut self, qarg: usize) {
        self.add_gate(CliffordGate::Y(qarg));
    }

    /// Applies a Pauli-Z gate to the specified qubit.
    /// ## Arguments
    /// * `qarg` - The index of the qubit to apply the gate to.
    pub fn apply_z(&mut self, qarg: usize) {
        self.add_gate(CliffordGate::Z(qarg));
    }

    /// Applies a Phase (S) gate to the specified qubit.
    /// ## Arguments
    /// * `qarg` - The index of the qubit to apply the gate to.
    pub fn apply_s(&mut self, qarg: usize) {
        self.add_gate(CliffordGate::S(qarg));
    }

    /// Applies a conjugate Phase (Sdg) gate to the specified qubit.
    /// ## Arguments
    /// * `qarg` - The index of the qubit to apply the gate to.
    pub fn apply_sdg(&mut self, qarg: usize) {
        self.add_gate(CliffordGate::Sdg(qarg));
    }

    /// Applies a square root of X (SqrtX) gate to the specified qubit.
    /// ## Arguments
    /// * `qarg` - The index of the qubit to apply the gate to.
    pub fn apply_sqrt_x(&mut self, qarg: usize) {
        self.add_gate(CliffordGate::SqrtX(qarg));
    }

    /// Applies a conjugate square root of X (SqrtXdg) gate to the specified qubit.
    /// ## Arguments
    /// * `qarg` - The index of the qubit to apply the gate to.
    pub fn apply_sqrt_xdg(&mut self, qarg: usize) {
        self.add_gate(CliffordGate::SqrtXdg(qarg));
    }

    /// Applies a controlled-X (CX) gate between the specified control and target qubits.
    /// ## Arguments
    /// * `control` - The index of the control qubit.
    /// * `target` - The index of the target qubit.
    pub fn apply_cx(&mut self, control: usize, target: usize) {
        self.add_gate(CliffordGate::CX(control, target));
    }

    /// Applies a controlled-Z (CZ) gate between the specified qubits.
    /// ## Arguments
    /// * `qarg1` - The index of the first qubit.
    /// * `qarg2` - The index of the second qubit.
    pub fn apply_cz(&mut self, qarg1: usize, qarg2: usize) {
        self.add_gate(CliffordGate::CZ(qarg1, qarg2));
    }

    /// Applies a SWAP gate between the specified qubits.
    /// ## Arguments
    /// * `qarg1` - The index of the first qubit.
    /// * `qarg2` - The index of the second qubit.
    pub fn apply_swap(&mut self, qarg1: usize, qarg2: usize) {
        self.add_gate(CliffordGate::Swap(qarg1, qarg2));
    }

    /// Parses an OpenQASM 2.0 file into a `CliffordCircuit`.
    ///
    /// ## Arguments
    /// * `path` - A path to the QASM file.
    ///
    /// ## Returns
    /// A `Result` containing the parsed `CliffordCircuit` or a `String` error message.
    pub fn from_qasm_file(path: &str) -> Result<Self> {
        parser::from_qasm_file(path)
    }

    /// Parses an OpenQASM 2.0 string into a `CliffordCircuit`.
    ///
    /// ## Arguments
    /// * `qasm_str` - A string slice containing the OpenQASM 2.0 circuit description.
    ///
    /// ## Returns
    /// A `Result` containing the parsed `CliffordCircuit` or a `String` error message.
    pub fn from_qasm_str(qasm_str: &str) -> Result<Self> {
        parser::from_qasm_str(qasm_str)
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
    /// # Arguments
    /// * `path` - The path to the output file.
    /// * `reg_name` - The name of the quantum register (e.g., "q").
    pub fn to_qasm_file(&self, path: &str, reg_name: &str) -> Result<()> {
        parser::to_qasm_file(self, path, reg_name)
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
    ///
    /// ## Returns
    /// A [`CliffordCircuit`] object representing the random Clifford operator.
    ///
    /// ## References
    /// - S. Bravyi and D. Maslov, "Hadamard-free circuits expose the structure of the Clifford
    ///   group," arXiv:2003.09412v2 (2021).
    pub fn random_clifford(n_qubits: usize, seed: Option<u64>) -> Self {
        random_clifford::random_clifford(n_qubits, seed)
    }
}
