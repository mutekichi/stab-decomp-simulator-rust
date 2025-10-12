use crate::{
    StabilizerCHForm,
    error::ChFormError,
    types::pauli::{PauliString, pauli_string::Pauli},
};

impl StabilizerCHForm {
    pub fn apply_pauli(&mut self, pauli_string: &PauliString) -> Result<(), ChFormError> {
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
}
