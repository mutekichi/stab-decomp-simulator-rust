use crate::circuit::CliffordGate;
use crate::circuit::parser;
use crate::circuit::random_clifford;
use crate::error::Result;
use std::fmt;

/// A struct representing a Clifford circuit composed of Clifford gates.
/// [`CliffordCircuit`] only stores the sequence of gates and does not calculate
/// the resulting stabilizer state.
///
/// ## Example usage:
///
/// ```rust
/// use stabilizer_ch_form_rust::circuit::CliffordCircuit;
/// use stabilizer_ch_form_rust::circuit::CliffordGate::{ H, CX };
///
/// let mut circuit = CliffordCircuit::new(2);
/// circuit.apply_h(0);
/// circuit.apply_cx(0, 1);
///
/// assert_eq!(circuit.gates[0], H(0));
/// assert_eq!(circuit.gates[1], CX(0, 1));
///
/// // `CliffordCircuit` is intended to be converted to `StabilizerCHForm` for simulation
/// use stabilizer_ch_form_rust::StabilizerCHForm;
/// let ch_form = StabilizerCHForm::from_clifford_circuit(&circuit).unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct CliffordCircuit {
    pub num_qubits: usize,
    pub gates: Vec<CliffordGate>,
}

impl CliffordCircuit {
    /// Creates a new Clifford circuit with the specified number of qubits.
    /// ## Arguments
    /// * `num_qubits` - The number of qubits in the circuit.
    pub fn new(num_qubits: usize) -> Self {
        CliffordCircuit {
            num_qubits,
            gates: Vec::new(),
        }
    }

    /// creates a new Clifford circuit by taking the tensor product of this circuit
    /// and another.
    /// Gates from `self` are applied to the first `self.num_qubits` qubits,
    /// and gates from `other` are applied to the next `other.num_qubits` qubits.
    ///
    /// ## Arguments
    /// * `other` - The other Clifford circuit to tensor with.
    ///
    /// ## Returns
    /// A new `CliffordCircuit` representing the tensor product.
    pub fn tensor(&self, other: &CliffordCircuit) -> Self {
        let mut new_circuit = CliffordCircuit::new(self.num_qubits + other.num_qubits);
        // Add gates from the first circuit
        for gate in &self.gates {
            new_circuit.gates.push(gate.clone());
        }
        // Add gates from the second circuit, shifting qubit indices
        for gate in &other.gates {
            new_circuit.gates.push(gate.shifted(self.num_qubits));
        }
        new_circuit
    }

    /// Appends the gates from another [`CliffordCircuit`] to this one.
    ///
    /// ## Arguments
    /// * `other` - The other Clifford circuit whose gates are to be appended.
    pub fn append(&mut self, other: &CliffordCircuit) {
        for gate in &other.gates {
            self.gates.push(gate.clone());
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

    /// Parses an OpenQASM 2.0 file into a [`CliffordCircuit`].
    ///
    /// ## Arguments
    /// * `path` - A path to the QASM file.
    ///
    /// ## Returns
    /// A [`Result`] containing the parsed [`CliffordCircuit`] or an [`Error`](crate::error::Error).
    pub fn from_qasm_file(path: &str) -> Result<Self> {
        parser::from_qasm_file(path)
    }

    /// Parses an OpenQASM 2.0 string into a [`CliffordCircuit`].
    ///
    /// ## Arguments
    /// * `qasm_str` - A string slice containing the OpenQASM 2.0 circuit description.
    ///
    /// ## Returns
    /// A [`Result`] containing the parsed [`CliffordCircuit`] or an [`Error`](crate::error::Error).
    pub fn from_qasm_str(qasm_str: &str) -> Result<Self> {
        parser::from_qasm_str(qasm_str)
    }

    /// Converts the circuit to an OpenQASM 2.0 string.
    ///
    /// ## Arguments
    /// * `reg_name` - The name of the quantum register (e.g., "q").
    ///
    /// ## Returns
    /// A [`String`] containing the OpenQASM 2.0 representation of the circuit.
    pub fn to_qasm_str(&self, reg_name: &str) -> String {
        parser::to_qasm_str(self, reg_name)
    }

    /// Writes the circuit to an OpenQASM 2.0 file.
    ///
    /// ## Arguments
    /// * `path` - The path to the output file.
    /// * `reg_name` - The name of the quantum register (e.g., "q").
    ///
    /// ## Returns
    /// A [`Result`] indicating success or failure.
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
    /// * `seed` - An optional seed for the random number generator to ensure reproducibility.
    ///   If `None`, a seed will be generated from system entropy.
    ///
    /// ## Returns
    /// A [`CliffordCircuit`] object representing the random Clifford operator.
    ///
    /// ## Reference
    /// - S. Bravyi and D. Maslov, "Hadamard-free circuits expose the structure of the Clifford
    ///   group," IEEE Trans. Inf. Theory 67, 5800 (2021).
    ///   <https://doi.org/10.1109/TIT.2021.3081415>
    pub fn random_clifford(num_qubits: usize, seed: Option<[u8; 32]>) -> Self {
        random_clifford::random_clifford(num_qubits, seed)
    }
}

impl fmt::Display for CliffordCircuit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CliffordCircuit(num_qubits={}) [", self.num_qubits)?;

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
        let mut circuit1 = CliffordCircuit::new(2);
        circuit1.apply_h(0);
        let mut circuit2 = CliffordCircuit::new(2);
        circuit2.apply_cx(0, 1);

        circuit1.append(&circuit2);

        assert_eq!(circuit1.gates.len(), 2);
        assert_eq!(circuit1.gates[0], CliffordGate::H(0));
        assert_eq!(circuit1.gates[1], CliffordGate::CX(0, 1));
    }

    #[test]
    fn test_tensor_circuit() {
        let mut circuit1 = CliffordCircuit::new(2);
        circuit1.apply_h(0);
        let mut circuit2 = CliffordCircuit::new(3);
        circuit2.apply_cx(0, 1);

        let tensor_circuit = circuit1.tensor(&circuit2);

        assert_eq!(tensor_circuit.num_qubits, 5);
        assert_eq!(tensor_circuit.gates.len(), 2);
        assert_eq!(tensor_circuit.gates[0], CliffordGate::H(0));
        assert_eq!(tensor_circuit.gates[1], CliffordGate::CX(2, 3));
    }

    #[test]
    fn test_clifford_circuit_display() {
        let mut circuit = CliffordCircuit::new(2);
        circuit.apply_h(0);
        circuit.apply_cx(0, 1);
        let display_str = format!("{}", circuit);
        assert_eq!(
            display_str,
            "CliffordCircuit(num_qubits=2) [H(0), CX(0, 1)]"
        );
    }
}
