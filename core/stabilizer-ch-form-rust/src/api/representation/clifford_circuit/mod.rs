use crate::api::clifford_gate::CliffordGate;

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

    pub fn add_multiple_gates(&mut self, gates: Vec<CliffordGate>) {
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
}
