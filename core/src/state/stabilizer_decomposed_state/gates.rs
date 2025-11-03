use crate::circuit::QuantumGate;
use crate::error::Result;
use crate::state::Coefficient;
use crate::state::StabilizerDecomposedState;

impl<T: Coefficient> StabilizerDecomposedState<T> {
    pub fn _apply_gate(&mut self, gate: &QuantumGate) -> Result<()> {
        match gate {
            QuantumGate::H(q) => self._apply_h(*q),
            QuantumGate::X(q) => self._apply_x(*q),
            QuantumGate::Y(q) => self._apply_y(*q),
            QuantumGate::Z(q) => self._apply_z(*q),
            QuantumGate::S(q) => self._apply_s(*q),
            QuantumGate::Sdg(q) => self._apply_sdg(*q),
            QuantumGate::SqrtX(q) => self._apply_sqrt_x(*q),
            QuantumGate::SqrtXdg(q) => self._apply_sqrt_xdg(*q),
            QuantumGate::CX(c, t) => self._apply_cx(*c, *t),
            QuantumGate::CZ(q1, q2) => self._apply_cz(*q1, *q2),
            QuantumGate::Swap(q1, q2) => self._apply_swap(*q1, *q2),
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

    pub fn _apply_gates(&mut self, gates: &[QuantumGate]) -> Result<()> {
        for gate in gates {
            self._apply_gate(gate)?;
        }
        Ok(())
    }

    // Single-qubit gates
    pub fn _apply_x(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_x(qarg)?
        }
        Ok(())
    }

    pub fn _apply_y(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_y(qarg)?
        }
        Ok(())
    }

    pub fn _apply_z(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_z(qarg)?;
        }
        Ok(())
    }

    pub fn _apply_h(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_h(qarg)?;
        }
        Ok(())
    }

    pub fn _apply_s(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_s(qarg)?;
        }
        Ok(())
    }

    pub fn _apply_sdg(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_sdg(qarg)?;
        }
        Ok(())
    }

    pub fn _apply_sqrt_x(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_sqrt_x(qarg)?;
        }
        Ok(())
    }

    pub fn _apply_sqrt_xdg(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_sqrt_xdg(qarg)?;
        }
        Ok(())
    }

    // Two-qubit gates
    pub fn _apply_cx(&mut self, control: usize, target: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_cx(control, target)?;
        }
        Ok(())
    }

    pub fn _apply_cz(&mut self, qarg1: usize, qarg2: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_cz(qarg1, qarg2)?;
        }
        Ok(())
    }

    pub fn _apply_swap(&mut self, qarg1: usize, qarg2: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_swap(qarg1, qarg2)?;
        }
        Ok(())
    }
}
