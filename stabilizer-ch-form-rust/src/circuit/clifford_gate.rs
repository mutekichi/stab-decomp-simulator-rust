use std::fmt;

/// Represents a Clifford gate in a quantum circuit.
#[derive(Debug, Clone, PartialEq)]
pub enum CliffordGate {
    H(usize),
    X(usize),
    Y(usize),
    Z(usize),
    S(usize),
    Sdg(usize),
    SqrtX(usize),
    SqrtXdg(usize),
    CX(usize, usize),
    CZ(usize, usize),
    Swap(usize, usize),
}

impl CliffordGate {
    /// Returns the QASM 2.0 string representation for this gate.
    pub fn to_qasm_str(&self, reg_name: &str) -> String {
        match self {
            CliffordGate::H(q) => format!("h {}[{}];", reg_name, q),
            CliffordGate::X(q) => format!("x {}[{}];", reg_name, q),
            CliffordGate::Y(q) => format!("y {}[{}];", reg_name, q),
            CliffordGate::Z(q) => format!("z {}[{}];", reg_name, q),
            CliffordGate::S(q) => format!("s {}[{}];", reg_name, q),
            CliffordGate::Sdg(q) => format!("sdg {}[{}];", reg_name, q),
            CliffordGate::SqrtX(q) => format!("sx {}[{}];", reg_name, q),
            CliffordGate::SqrtXdg(q) => format!("sxdg {}[{}];", reg_name, q),
            CliffordGate::CX(c, t) => format!("cx {}[{}], {}[{}];", reg_name, c, reg_name, t),
            CliffordGate::CZ(q1, q2) => format!("cz {}[{}], {}[{}];", reg_name, q1, reg_name, q2),
            CliffordGate::Swap(q1, q2) => {
                format!("swap {}[{}], {}[{}];", reg_name, q1, reg_name, q2)
            }
        }
    }
}

impl fmt::Display for CliffordGate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_qasm_str("q"))
    }
}
