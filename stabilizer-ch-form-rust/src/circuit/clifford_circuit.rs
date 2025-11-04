use crate::circuit::CliffordGate;
use crate::circuit::parser;
use crate::circuit::random_clifford;
use crate::error::Result;
/// TOOD: Documentation
#[derive(Debug, Clone)]
pub struct CliffordCircuit {
    pub n_qubits: usize,
    pub gates: Vec<CliffordGate>,
}

impl CliffordCircuit {
    pub fn new(n_qubits: usize) -> Self {
        CliffordCircuit {
            n_qubits,
            gates: Vec::new(),
        }
    }
    pub fn add_gate(&mut self, gate: CliffordGate) {
        self.gates.push(gate);
    }

    pub fn add_gates(&mut self, gates: Vec<CliffordGate>) {
        for gate in gates {
            self.add_gate(gate);
        }
    }

    pub fn apply_h(&mut self, qarg: usize) {
        self.add_gate(CliffordGate::H(qarg));
    }

    pub fn apply_x(&mut self, qarg: usize) {
        self.add_gate(CliffordGate::X(qarg));
    }

    pub fn apply_s(&mut self, qarg: usize) {
        self.add_gate(CliffordGate::S(qarg));
    }

    pub fn apply_z(&mut self, qarg: usize) {
        self.add_gate(CliffordGate::Z(qarg));
    }

    pub fn apply_cx(&mut self, control: usize, target: usize) {
        self.add_gate(CliffordGate::CX(control, target));
    }

    pub fn apply_cz(&mut self, control: usize, target: usize) {
        self.add_gate(CliffordGate::CZ(control, target));
    }

    pub fn from_qasm_file(path: &str) -> Result<Self> {
        parser::_from_qasm_file(path)
    }

    pub fn from_qasm_str(qasm_str: &str) -> Result<Self> {
        parser::_from_qasm_str(qasm_str)
    }

    pub fn to_qasm_str(&self, reg_name: &str) -> String {
        parser::_to_qasm_str(self, reg_name)
    }

    pub fn to_qasm_file(&self, path: &str, reg_name: &str) -> Result<()> {
        parser::_to_qasm_file(self, path, reg_name)
    }

    /// Generates a random n-qubit Clifford circuit using the Bravyi-Maslov canonical form.
    ///
    /// This function implements the O(n^2) algorithm described in the paper to sample a Clifford operator uniformly at random from the n-qubit Clifford group.
    /// The resulting circuit is structured according to the canonical form U = F1 * H * S * F2. See the reference for details.
    ///
    /// ## Arguments
    /// * `n` - The number of qubits. Must be greater than 0.
    /// * `seed` - An optional seed for the random number generator for reproducibility.
    ///
    /// ## Returns
    /// A [`CliffordCircuit`] object representing the random Clifford operator.
    ///
    /// ## References
    /// - S. Bravyi and D. Maslov, "Hadamard-free circuits expose the structure of the Clifford group," arXiv:2003.09412v2 (2021).
    pub fn random_clifford(n_qubits: usize, seed: Option<u64>) -> Self {
        random_clifford::_random_clifford(n_qubits, seed)
    }
}
