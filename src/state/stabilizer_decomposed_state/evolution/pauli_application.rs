use stabilizer_ch_form_rust::types::pauli::{PauliString, pauli_string::Pauli};

use crate::state::{Coefficient, StabilizerDecomposedState};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    pub(crate) fn _apply_pauli_string(&mut self, pauli_string: &PauliString) {
        match pauli_string {
            PauliString::Dense(ops) => {
                for (qubit, &op) in ops.iter().enumerate() {
                    match op {
                        Pauli::I => {}
                        Pauli::X => self.apply_x(qubit),
                        Pauli::Y => self.apply_y(qubit),
                        Pauli::Z => self.apply_z(qubit),
                    }
                }
            }
            PauliString::Sparse(terms) => {
                for term in terms {
                    match term.op {
                        Pauli::I => {}
                        Pauli::X => self.apply_x(term.qubit),
                        Pauli::Y => self.apply_y(term.qubit),
                        Pauli::Z => self.apply_z(term.qubit),
                    }
                }
            }
        }
    }
}
