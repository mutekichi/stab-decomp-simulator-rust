use crate::StabilizerCHForm;
use crate::core::internal::types::measurement::QubitState;

impl StabilizerCHForm {
    pub(crate) fn _get_qubit_state(&self, qarg: usize) -> QubitState {
        if qarg >= self.n {
            panic!("Qubit index out of bounds.");
        }
        if qarg >= self.n_qubits() {
            panic!("Qubit index ouWt of bounds.");
        }

        let g_row = self.mat_g.row(qarg);

        // Check for superposition: the state is a superposition if any v[i] is true
        // where the corresponding G_row[i] is also true.
        let is_determined = !g_row.iter().zip(&self.vec_v).any(|(&g, &v)| g && v);

        if is_determined {
            // If determined, the value is the parity of the inner product
            // of the g_row and the s vector.
            let value = g_row.iter().zip(&self.vec_s).fold(
                0,
                |acc, (&g, &s)| {
                    if g && s { acc + 1 } else { acc }
                },
            ) % 2
                == 1;
            QubitState::Determined(value)
        } else {
            QubitState::Superposition
        }
    }
}
