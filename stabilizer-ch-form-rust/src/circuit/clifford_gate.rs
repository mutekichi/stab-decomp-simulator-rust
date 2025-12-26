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

    /// Returns a new `CliffordGate` with qubit indices shifted by the specified offset.
    pub(crate) fn shifted(&self, offset: usize) -> Self {
        let mut new_gate = self.clone();
        match &mut new_gate {
            CliffordGate::H(q)
            | CliffordGate::X(q)
            | CliffordGate::Y(q)
            | CliffordGate::Z(q)
            | CliffordGate::S(q)
            | CliffordGate::Sdg(q)
            | CliffordGate::SqrtX(q)
            | CliffordGate::SqrtXdg(q) => {
                *q += offset;
            }
            CliffordGate::CX(c, t) | CliffordGate::CZ(c, t) | CliffordGate::Swap(c, t) => {
                *c += offset;
                *t += offset;
            }
        }
        new_gate
    }
}

impl fmt::Display for CliffordGate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliffordGate::H(q) => write!(f, "H({})", q),
            CliffordGate::X(q) => write!(f, "X({})", q),
            CliffordGate::Y(q) => write!(f, "Y({})", q),
            CliffordGate::Z(q) => write!(f, "Z({})", q),
            CliffordGate::S(q) => write!(f, "S({})", q),
            CliffordGate::Sdg(q) => write!(f, "Sdg({})", q),
            CliffordGate::SqrtX(q) => write!(f, "SqrtX({})", q),
            CliffordGate::SqrtXdg(q) => write!(f, "SqrtXdg({})", q),
            CliffordGate::CX(c, t) => write!(f, "CX({}, {})", c, t),
            CliffordGate::CZ(q1, q2) => write!(f, "CZ({}, {})", q1, q2),
            CliffordGate::Swap(q1, q2) => write!(f, "Swap({}, {})", q1, q2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clifford_gate_display() {
        let h_gate = CliffordGate::H(0);
        assert_eq!(format!("{}", h_gate), "H(0)");
        let cx_gate = CliffordGate::CX(1, 2);
        assert_eq!(format!("{}", cx_gate), "CX(1, 2)");
    }

    #[test]
    fn test_clifford_gate_to_qasm_str() {
        let h_gate = CliffordGate::H(0);
        assert_eq!(h_gate.to_qasm_str("q"), "h q[0];");
        let cx_gate = CliffordGate::CX(1, 2);
        assert_eq!(cx_gate.to_qasm_str("q"), "cx q[1], q[2];");
    }
}
