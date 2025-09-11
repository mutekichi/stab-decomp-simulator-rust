use crate::circuit::QuantumCircuit;
use crate::circuit::QuantumGate;

impl QuantumCircuit {
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
}
