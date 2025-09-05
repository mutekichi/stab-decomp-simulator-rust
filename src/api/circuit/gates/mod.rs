#[derive(Debug, Clone, PartialEq)]
pub enum QuantumGate {
    // Clifford gates
    // - Single-qubit Cliffords
    /// Hadamard gate
    H(usize),
    /// Pauli-X gate
    X(usize),
    /// Pauli-Y gate
    Y(usize),
    /// Pauli-Z gate
    Z(usize),
    /// S gate
    S(usize),
    /// S-dagger gate
    Sdg(usize),
    /// Square root of X gate
    SqrtX(usize),
    /// Square root of X-dagger gate
    SqrtXdg(usize),
    // - Two-qubit Cliffords
    /// Controlled-NOT (CNOT) gate
    CX(usize, usize),
    /// Controlled-Z (CZ) gate
    CZ(usize, usize),
    /// SWAP gate
    Swap(usize, usize),
    // Non-Clifford gates
    // - Single-qubit Non-Cliffords
    /// T gate
    T(usize),
    /// T-dagger gate
    Tdg(usize),
    // - Multi-qubit Non-Cliffords
    /// Toffoli (CCX) gate
    CCX(usize, usize, usize), // (control1, control2, target)
}

impl QuantumGate {
    /// Checks if the gate is a single-qubit gate.
    /// ### Returns
    /// * `bool` - `true` if the gate is a single-qubit gate, otherwise `false`.
    /// ### Examples
    /// ```rust
    /// let gate = QuantumGate::H(0);
    /// println!("{}", gate.is_single_qubit_gate()); // true
    /// let gate = QuantumGate::CX(0, 1);
    /// println!("{}", gate.is_single_qubit_gate()); // false
    /// ```
    pub fn is_single_qubit_gate(&self) -> bool {
        matches!(
            self,
            QuantumGate::H(_)
                | QuantumGate::X(_)
                | QuantumGate::Y(_)
                | QuantumGate::Z(_)
                | QuantumGate::S(_)
                | QuantumGate::Sdg(_)
                | QuantumGate::SqrtX(_)
                | QuantumGate::SqrtXdg(_)
                | QuantumGate::T(_)
                | QuantumGate::Tdg(_)
        )
    }
    
    /// Checks if the gate is a Clifford gate.
    /// ### Returns
    /// * `bool` - `true` if the gate is a Clifford gate, otherwise `false`.
    /// ### Examples
    /// ```rust
    /// let gate = QuantumGate::H(0);
    /// println!("{}", gate.is_clifford()); // true
    /// let gate = QuantumGate::T(0);
    /// println!("{}", gate.is_clifford()); // false
    /// ```
    pub fn is_clifford(&self) -> bool {
        matches!(
            self,
            QuantumGate::H(_)
                | QuantumGate::X(_)
                | QuantumGate::Y(_)
                | QuantumGate::Z(_)
                | QuantumGate::S(_)
                | QuantumGate::Sdg(_)
                | QuantumGate::SqrtX(_)
                | QuantumGate::SqrtXdg(_)
                | QuantumGate::CX(_, _)
                | QuantumGate::CZ(_, _)
                | QuantumGate::Swap(_, _)
        )
    }

    // --- Crate internal use only ---
    pub(crate) fn shift_indices(&mut self, offset: usize) {
        match self {
            // Single-qubit gates
            QuantumGate::H(q) | QuantumGate::X(q) | QuantumGate::Y(q) |
            QuantumGate::Z(q) | QuantumGate::S(q) | QuantumGate::Sdg(q) |
            QuantumGate::SqrtX(q) | QuantumGate::SqrtXdg(q) |
            QuantumGate::T(q) | QuantumGate::Tdg(q) => {
                *q += offset;
            }
            // Two-qubit gates
            QuantumGate::CX(c, t) | QuantumGate::CZ(c, t) | QuantumGate::Swap(c, t) => {
                *c += offset;
                *t += offset;
            }
            // Three-qubit gates
            QuantumGate::CCX(c1, c2, t) => {
                *c1 += offset;
                *c2 += offset;
                *t += offset;
            }
        }
    }

    pub(crate) fn shifted(&self, offset: usize) -> Self {
        let mut new_gate = self.clone();
        new_gate.shift_indices(offset);
        new_gate
    }
}
