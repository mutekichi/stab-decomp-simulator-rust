use crate::state::Coefficient;
use crate::state::StabilizerDecomposedState;
use stabilizer_ch_form_rust::prelude::*;

impl<T: Coefficient> StabilizerDecomposedState<T> {
    // Single-qubit gates
    pub fn _apply_x(&mut self, qarg: usize) {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_x(qarg);
        }
    }

    pub fn _apply_y(&mut self, qarg: usize) {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_y(qarg);
        }
    }

    pub fn _apply_z(&mut self, qarg: usize) {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_z(qarg);
        }
    }

    pub fn _apply_h(&mut self, qarg: usize) {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_h(qarg);
        }
    }

    pub fn _apply_s(&mut self, qarg: usize) {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_s(qarg);
        }
    }

    pub fn _apply_sdg(&mut self, qarg: usize) {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_sdg(qarg);
        }
    }

    pub fn _apply_sqrt_x(&mut self, qarg: usize) {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_sqrt_x(qarg);
        }
    }

    pub fn _apply_sqrt_xdg(&mut self, qarg: usize) {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_sqrt_xdg(qarg);
        }
    }

    // Two-qubit gates
    pub fn _apply_cx(&mut self, control: usize, target: usize) {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_cx(control, target);
        }
    }

    pub fn _apply_cz(&mut self, qarg1: usize, qarg2: usize) {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_cz(qarg1, qarg2);
        }
    }

    pub fn _apply_swap(&mut self, qarg1: usize, qarg2: usize) {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_swap(qarg1, qarg2);
        }
    }
}
