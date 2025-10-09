pub mod gate_applications;
pub mod gates;
pub mod manipulations;
pub mod parser;

pub use gates::*;
pub use parser::*;

/// Represents a quantum circuit as a sequence of quantum gates.
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
}
