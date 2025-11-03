use stabilizer_ch_form_rust::types::pauli::{Pauli, PauliString};

use crate::error::Result;
use crate::state::{Coefficient, StabilizerDecomposedState};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    pub(crate) fn _apply_pauli_string(&mut self, pauli_string: &PauliString) -> Result<()> {
        match pauli_string {
            PauliString::Dense(ops) => {
                for (qubit, &op) in ops.iter().enumerate() {
                    match op {
                        Pauli::I => {}
                        Pauli::X => self._apply_x(qubit)?,
                        Pauli::Y => self._apply_y(qubit)?,
                        Pauli::Z => self._apply_z(qubit)?,
                    }
                }
            }
            PauliString::Sparse(terms) => {
                for term in terms {
                    match term.op {
                        Pauli::I => {}
                        Pauli::X => self._apply_x(term.qubit)?,
                        Pauli::Y => self._apply_y(term.qubit)?,
                        Pauli::Z => self._apply_z(term.qubit)?,
                    }
                }
            }
        }
        Ok(())
    }
}
