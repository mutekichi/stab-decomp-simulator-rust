pub mod gate_applications;
pub mod gates;
pub mod manipulations;
pub mod parser;

pub use gates::*;
pub use parser::*;

pub struct QuantumCircuit {
    pub num_qubits: usize,
    // pub num_cbits: Option<usize>,
    pub gates: Vec<QuantumGate>,
}

impl QuantumCircuit {
    // pub fn new(num_qubits: usize, num_cbits: Option<usize>) -> Self {
    pub fn new(num_qubits: usize) -> Self {
        Self {
            num_qubits,
            // num_cbits,
            gates: Vec::new(),
        }
    }
}
