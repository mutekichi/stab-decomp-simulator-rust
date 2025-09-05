pub mod gate;
pub use gate::*;
pub struct QuantumCircuit {
    pub num_qubits: usize,
    pub num_cbits: Option<usize>,
    pub gates: Vec<QuantumGate>,
}

impl QuantumCircuit {
    pub fn new(num_qubits: usize, num_cbits: Option<usize>) -> Self {
        Self {
            num_qubits,
            num_cbits,
            gates: Vec::new(),
        }
    }
}
