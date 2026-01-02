use crate::{
    StabilizerCHForm,
    circuit::{CliffordCircuit, CliffordGate},
    error::Result,
    types::pauli::{Pauli, PauliString},
};

impl StabilizerCHForm {
    /// Applies the Hadamard gate to the qubit at index `qarg`.
    ///     
    /// Time complexity: O(n^2)
    ///
    /// ## Arguments
    /// * `qarg` - The index of the qubit to which the Hadamard gate is applied.
    ///
    /// ## Returns
    /// A [`Result`] indicating success or failure.
    pub fn apply_h(&mut self, qarg: usize) -> Result<()> {
        self.left_multiply_h(qarg)
    }

    /// Applies the Pauli-X gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(n)
    ///
    /// ## Arguments
    /// * `qarg` - The index of the qubit to which the Pauli-X gate is applied.
    ///
    /// ## Returns
    /// A [`Result`] indicating success or failure.
    pub fn apply_x(&mut self, qarg: usize) -> Result<()> {
        self.left_multiply_x(qarg)
    }

    /// Applies the Pauli-Y gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(n)
    ///
    /// ## Arguments
    /// * `qarg` - The index of the qubit to which the Pauli-Y gate is applied.
    ///
    /// ## Returns
    /// A [`Result`] indicating success or failure.
    pub fn apply_y(&mut self, qarg: usize) -> Result<()> {
        self.left_multiply_y(qarg)
    }

    /// Applies the Pauli-Z gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(1)
    ///
    /// ## Arguments
    /// * `qarg` - The index of the qubit to which the Pauli-Z gate is applied.
    ///
    /// ## Returns
    /// A [`Result`] indicating success or failure.
    pub fn apply_z(&mut self, qarg: usize) -> Result<()> {
        self.left_multiply_z(qarg)
    }

    /// Applies the Phase (S) gate to the qubit at index `qarg`.
    ///     
    /// Time complexity: O(n)
    ///
    /// ## Arguments
    /// * `qarg` - The index of the qubit to which the Phase (S) gate is applied.
    ///
    /// ## Returns
    /// A [`Result`] indicating success or failure.
    pub fn apply_s(&mut self, qarg: usize) -> Result<()> {
        self.left_multiply_s(qarg)
    }

    /// Applies the adjoint Phase (S†) gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(n)
    ///
    /// ## Arguments
    /// * `qarg` - The index of the qubit to which the adjoint Phase (S†) gate is applied.
    ///
    /// ## Returns
    /// A [`Result`] indicating success or failure.
    pub fn apply_sdg(&mut self, qarg: usize) -> Result<()> {
        self.left_multiply_sdg(qarg)
    }

    /// Applies the √X gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(n^2)
    ///
    /// ## Arguments
    /// * `qarg` - The index of the qubit to which the √X gate is applied.
    ///
    /// ## Returns
    /// A [`Result`] indicating success or failure.
    pub fn apply_sqrt_x(&mut self, qarg: usize) -> Result<()> {
        self.left_multiply_sqrt_x(qarg)
    }

    /// Applies the adjoint of the √X gate to the qubit at index `qarg`.
    ///
    /// Time complexity: O(n^2)
    ///
    /// ## Arguments
    /// * `qarg` - The index of the qubit to which the adjoint of the √X gate is applied.
    ///
    /// ## Returns
    /// A [`Result`] indicating success or failure.
    pub fn apply_sqrt_xdg(&mut self, qarg: usize) -> Result<()> {
        self.left_multiply_sqrt_xdg(qarg)
    }

    /// Applies the CNOT (CX) gate with control qubit at index `control` and target qubit at index `target`.
    ///
    /// Time complexity: O(n)
    ///
    /// ## Arguments
    /// * `control` - The index of the control qubit.
    /// * `target` - The index of the target qubit.
    ///
    /// ## Returns
    /// A [`Result`] indicating success or failure.
    pub fn apply_cx(&mut self, control: usize, target: usize) -> Result<()> {
        self.left_multiply_cx(control, target)
    }

    /// Applies the CZ gate between qubits at indices `qarg1` and `qarg2`.
    ///
    /// Time complexity: O(n)
    ///
    /// ## Arguments
    /// * `qarg1` - The index of the first qubit.
    /// * `qarg2` - The index of the second qubit.
    ///
    /// ## Returns
    /// A [`Result`] indicating success or failure.
    pub fn apply_cz(&mut self, qarg1: usize, qarg2: usize) -> Result<()> {
        self.left_multiply_cz(qarg1, qarg2)
    }

    /// Applies the SWAP gate between the qubits at indices `qarg1` and `qarg2`.
    ///
    /// Time complexity: O(n)
    ///
    /// ## Arguments
    /// * `qarg1` - The index of the first qubit.
    /// * `qarg2` - The index of the second qubit.
    ///
    /// ## Returns
    /// A [`Result`] indicating success or failure.
    pub fn apply_swap(&mut self, qarg1: usize, qarg2: usize) -> Result<()> {
        self.left_multiply_swap(qarg1, qarg2)
    }

    /// Applies a Clifford gate to the stabilizer state.
    ////
    /// ## Arguments
    /// * `gate` - The Clifford gate to apply.
    ///
    /// ## Returns
    /// A [`Result`] indicating success or failure.
    pub fn apply_gate(&mut self, gate: &CliffordGate) -> Result<()> {
        match gate {
            CliffordGate::H(qarg) => self.apply_h(*qarg)?,
            CliffordGate::X(qarg) => self.apply_x(*qarg)?,
            CliffordGate::Y(qarg) => self.apply_y(*qarg)?,
            CliffordGate::Z(qarg) => self.apply_z(*qarg)?,
            CliffordGate::S(qarg) => self.apply_s(*qarg)?,
            CliffordGate::Sdg(qarg) => self.apply_sdg(*qarg)?,
            CliffordGate::SqrtX(qarg) => self.apply_sqrt_x(*qarg)?,
            CliffordGate::SqrtXdg(qarg) => self.apply_sqrt_xdg(*qarg)?,
            CliffordGate::CX(control, target) => self.apply_cx(*control, *target)?,
            CliffordGate::CZ(control, target) => self.apply_cz(*control, *target)?,
            CliffordGate::Swap(q1, q2) => self.apply_swap(*q1, *q2)?,
        }
        Ok(())
    }

    /// Applies a Pauli string to the stabilizer state.
    ///
    /// ## Arguments
    /// * `pauli_string` - The Pauli string to apply.
    ///
    /// ## Returns
    /// A [`Result`] indicating success or failure.
    pub fn apply_pauli(&mut self, pauli_string: &PauliString) -> Result<()> {
        match pauli_string {
            PauliString::Dense(ops) => {
                for (qubit, &op) in ops.iter().enumerate() {
                    match op {
                        Pauli::I => {}
                        Pauli::X => self.apply_x(qubit)?,
                        Pauli::Y => self.apply_y(qubit)?,
                        Pauli::Z => self.apply_z(qubit)?,
                    }
                }
            }
            PauliString::Sparse(terms) => {
                for term in terms {
                    match term.op {
                        Pauli::I => {}
                        Pauli::X => self.apply_x(term.qubit)?,
                        Pauli::Y => self.apply_y(term.qubit)?,
                        Pauli::Z => self.apply_z(term.qubit)?,
                    }
                }
            }
        }
        Ok(())
    }

    /// Applies a Clifford circuit to the stabilizer state.
    ///
    /// ## Arguments
    /// * `circuit` - The Clifford circuit to apply.
    ///
    /// ## Returns
    /// A [`Result`] indicating success or failure.
    pub fn apply_circuit(&mut self, circuit: &CliffordCircuit) -> Result<()> {
        for gate in &circuit.gates {
            self.apply_gate(gate)?;
        }
        Ok(())
    }
}
