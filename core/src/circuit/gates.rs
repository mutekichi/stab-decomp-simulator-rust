use crate::error::{Error, Result};
use stabilizer_ch_form_rust::api::CliffordGate;
use std::fmt;

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
    /// use stab_decomp_simulator_rust::circuit::QuantumGate;
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
    /// use stab_decomp_simulator_rust::circuit::QuantumGate;
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

    /// Checks if the gate is a T-type gate.
    /// Note that this checks for both T and T-dagger gates.
    /// ### Returns
    /// * `bool` - `true` if the gate is a T-type gate, otherwise `false`.
    pub fn is_t_type_gate(&self) -> bool {
        matches!(self, QuantumGate::T(_) | QuantumGate::Tdg(_))
    }

    /// Checks if the gate is a T gate.
    /// Note that this only checks for the T gate, not T-dagger.
    /// ### Returns
    /// * `bool` - `true` if the gate is a T gate, otherwise `false`.
    pub fn is_t_gate(&self) -> bool {
        matches!(self, QuantumGate::T(_))
    }

    /// Checks if the gate is a T-dagger gate.
    ///
    /// ### Returns
    /// * `bool` - `true` if the gate is a T-dagger gate, otherwise `false`.
    pub fn is_tdg_gate(&self) -> bool {
        matches!(self, QuantumGate::Tdg(_))
    }

    /// Returns the indices of the qubits this gate acts upon.
    ///
    /// The order of the indices is generally control qubits followed by target qubits,
    /// but it is not guaranteed for all gates.
    ///
    /// ### Returns
    /// * `Vec<usize>` - A vector containing the qubit indices.
    ///
    /// ### Examples
    /// ```rust
    /// use stab_decomp_simulator_rust::circuit::QuantumGate;
    ///
    /// let h_gate = QuantumGate::H(0);
    /// assert_eq!(h_gate.qubits(), vec![0]);
    ///
    /// let cx_gate = QuantumGate::CX(1, 3);
    /// assert_eq!(cx_gate.qubits(), vec![1, 3]);
    ///
    /// let ccx_gate = QuantumGate::CCX(0, 1, 2);
    /// assert_eq!(ccx_gate.qubits(), vec![0, 1, 2]);
    /// ```
    pub fn qubits(&self) -> Vec<usize> {
        match *self {
            // Single-qubit gates
            QuantumGate::H(q)
            | QuantumGate::X(q)
            | QuantumGate::Y(q)
            | QuantumGate::Z(q)
            | QuantumGate::S(q)
            | QuantumGate::Sdg(q)
            | QuantumGate::SqrtX(q)
            | QuantumGate::SqrtXdg(q)
            | QuantumGate::T(q)
            | QuantumGate::Tdg(q) => vec![q],

            // Two-qubit gates
            QuantumGate::CX(c, t) | QuantumGate::CZ(c, t) | QuantumGate::Swap(c, t) => vec![c, t],

            // Three-qubit gates
            QuantumGate::CCX(c1, c2, t) => vec![c1, c2, t],
        }
    }

    /// Display the gate name.
    /// ### Returns
    /// * `&'static str` - The name of the gate as a string slice.
    pub fn name(&self) -> &'static str {
        match self {
            QuantumGate::H(_) => "H",
            QuantumGate::X(_) => "X",
            QuantumGate::Y(_) => "Y",
            QuantumGate::Z(_) => "Z",
            QuantumGate::S(_) => "S",
            QuantumGate::Sdg(_) => "Sdg",
            QuantumGate::SqrtX(_) => "SqrtX",
            QuantumGate::SqrtXdg(_) => "SqrtXdg",
            QuantumGate::CX(_, _) => "CX",
            QuantumGate::CZ(_, _) => "CZ",
            QuantumGate::Swap(_, _) => "Swap",
            QuantumGate::T(_) => "T",
            QuantumGate::Tdg(_) => "Tdg",
            QuantumGate::CCX(_, _, _) => "CCX",
        }
    }

    // --- Crate internal use only ---
    pub(crate) fn shift_indices(&mut self, offset: usize) {
        match self {
            // Single-qubit gates
            QuantumGate::H(q)
            | QuantumGate::X(q)
            | QuantumGate::Y(q)
            | QuantumGate::Z(q)
            | QuantumGate::S(q)
            | QuantumGate::Sdg(q)
            | QuantumGate::SqrtX(q)
            | QuantumGate::SqrtXdg(q)
            | QuantumGate::T(q)
            | QuantumGate::Tdg(q) => {
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

    pub(crate) fn to_clifford_gate(&self) -> Result<CliffordGate> {
        match self {
            QuantumGate::H(q) => Ok(CliffordGate::H(*q)),
            QuantumGate::X(q) => Ok(CliffordGate::X(*q)),
            QuantumGate::Y(q) => Ok(CliffordGate::Y(*q)),
            QuantumGate::Z(q) => Ok(CliffordGate::Z(*q)),
            QuantumGate::S(q) => Ok(CliffordGate::S(*q)),
            QuantumGate::Sdg(q) => Ok(CliffordGate::Sdg(*q)),
            QuantumGate::SqrtX(q) => Ok(CliffordGate::SqrtX(*q)),
            QuantumGate::SqrtXdg(q) => Ok(CliffordGate::SqrtXdg(*q)),
            QuantumGate::CX(c, t) => Ok(CliffordGate::CX(*c, *t)),
            QuantumGate::CZ(c, t) => Ok(CliffordGate::CZ(*c, *t)),
            QuantumGate::Swap(q1, q2) => Ok(CliffordGate::Swap(*q1, *q2)),
            _ => Err(Error::GateNotClifford(self.name().to_string())),
        }
    }
}

impl fmt::Display for QuantumGate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuantumGate::H(q) => write!(f, "H({})", q),
            QuantumGate::X(q) => write!(f, "X({})", q),
            QuantumGate::Y(q) => write!(f, "Y({})", q),
            QuantumGate::Z(q) => write!(f, "Z({})", q),
            QuantumGate::S(q) => write!(f, "S({})", q),
            QuantumGate::Sdg(q) => write!(f, "Sdg({})", q),
            QuantumGate::SqrtX(q) => write!(f, "SqrtX({})", q),
            QuantumGate::SqrtXdg(q) => write!(f, "SqrtXdg({})", q),
            QuantumGate::T(q) => write!(f, "T({})", q),
            QuantumGate::Tdg(q) => write!(f, "Tdg({})", q),
            QuantumGate::CX(c, t) => write!(f, "CX({}, {})", c, t),
            QuantumGate::CZ(c, t) => write!(f, "CZ({}, {})", c, t),
            QuantumGate::Swap(q1, q2) => write!(f, "Swap({}, {})", q1, q2),
            QuantumGate::CCX(c1, c2, t) => write!(f, "CCX({}, {}, {})", c1, c2, t),
        }
    }
}
