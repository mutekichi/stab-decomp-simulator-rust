use crate::api::QuantumCircuit;

impl QuantumCircuit {
    /// Appends the gates from another `QuantumCircuit` to this one.
    ///
    /// # Arguments
    /// - `other`: A reference to another `QuantumCircuit` whose gates will be appended.
    /// # Example
    /// ```rust
    /// let mut circuit1 = QuantumCircuit::new(2, None);
    /// circuit1.apply_h(0);
    /// let mut circuit2 = QuantumCircuit::new(2, None);
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
    /// let mut circuit1 = QuantumCircuit::new(1, None);
    /// circuit1.apply_h(0);
    /// let mut circuit2 = QuantumCircuit::new(1, None);
    /// circuit2.apply_x(0);
    /// let tensor_circuit = circuit1.tensor(&circuit2);
    /// ```
    pub fn tensor(&self, other: &QuantumCircuit) -> QuantumCircuit {
        let mut new_circuit = QuantumCircuit::new(
            self.num_qubits + other.num_qubits
        );

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