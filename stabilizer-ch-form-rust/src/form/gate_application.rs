use crate::{
    StabilizerCHForm,
    circuit::{CliffordCircuit, CliffordGate},
    error::Result,
    types::pauli::{Pauli, PauliString},
};

impl StabilizerCHForm {
    pub(crate) fn _apply_gate(&mut self, gate: &CliffordGate) -> Result<()> {
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

    pub(crate) fn _apply_pauli(&mut self, pauli_string: &PauliString) -> Result<()> {
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

    pub(crate) fn _apply_circuit(&mut self, circuit: &CliffordCircuit) -> Result<()> {
        for gate in &circuit.gates {
            self.apply_gate(gate)?;
        }
        Ok(())
    }
}
