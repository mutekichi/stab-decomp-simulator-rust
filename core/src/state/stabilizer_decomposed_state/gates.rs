use crate::circuit::QuantumGate;
use crate::error::Result;
use crate::state::Coefficient;
use crate::state::StabilizerDecomposedState;

impl<T: Coefficient> StabilizerDecomposedState<T> {
    pub(crate) fn apply_gate(&mut self, gate: &QuantumGate) -> Result<()> {
        match gate {
            QuantumGate::H(q) => self.apply_h(*q),
            QuantumGate::X(q) => self.apply_x(*q),
            QuantumGate::Y(q) => self.apply_y(*q),
            QuantumGate::Z(q) => self.apply_z(*q),
            QuantumGate::S(q) => self.apply_s(*q),
            QuantumGate::Sdg(q) => self.apply_sdg(*q),
            QuantumGate::SqrtX(q) => self.apply_sqrt_x(*q),
            QuantumGate::SqrtXdg(q) => self.apply_sqrt_xdg(*q),
            QuantumGate::CX(c, t) => self.apply_cx(*c, *t),
            QuantumGate::CZ(q1, q2) => self.apply_cz(*q1, *q2),
            QuantumGate::Swap(q1, q2) => self.apply_swap(*q1, *q2),
            QuantumGate::T(_) => Err(crate::error::Error::NonCliffordGateApplication(
                gate.name().to_string(),
            )),
            QuantumGate::Tdg(_) => Err(crate::error::Error::NonCliffordGateApplication(
                gate.name().to_string(),
            )),
            QuantumGate::CCX(_, _, _) => Err(crate::error::Error::NonCliffordGateApplication(
                gate.name().to_string(),
            )),
        }
    }

    pub(crate) fn apply_gates(&mut self, gates: &[QuantumGate]) -> Result<()> {
        for gate in gates {
            self.apply_gate(gate)?;
        }
        Ok(())
    }

    // Single-qubit gates
    pub(crate) fn apply_x(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_x(qarg)?
        }
        Ok(())
    }

    pub(crate) fn apply_y(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_y(qarg)?
        }
        Ok(())
    }

    pub(crate) fn apply_z(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_z(qarg)?;
        }
        Ok(())
    }

    pub(crate) fn apply_h(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_h(qarg)?;
        }
        Ok(())
    }

    pub(crate) fn apply_s(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_s(qarg)?;
        }
        Ok(())
    }

    pub(crate) fn apply_sdg(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_sdg(qarg)?;
        }
        Ok(())
    }

    pub(crate) fn apply_sqrt_x(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_sqrt_x(qarg)?;
        }
        Ok(())
    }

    pub(crate) fn apply_sqrt_xdg(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_sqrt_xdg(qarg)?;
        }
        Ok(())
    }

    // Two-qubit gates
    pub(crate) fn apply_cx(&mut self, control: usize, target: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_cx(control, target)?;
        }
        Ok(())
    }

    pub(crate) fn apply_cz(&mut self, qarg1: usize, qarg2: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_cz(qarg1, qarg2)?;
        }
        Ok(())
    }

    pub(crate) fn apply_swap(&mut self, qarg1: usize, qarg2: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_swap(qarg1, qarg2)?;
        }
        Ok(())
    }
}
// WIP: Add tests
