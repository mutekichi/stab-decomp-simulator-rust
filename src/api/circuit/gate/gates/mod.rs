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
