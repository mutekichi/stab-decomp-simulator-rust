use crate::state::Coefficient;
use crate::state::StabilizerDecomposedState;
use stabilizer_ch_form_rust::prelude::*;

#[allow(dead_code)]
macro_rules! impl_single_qubit_gate {
    ( $( pub fn $method_name:ident(&mut self, qarg: usize) );* ) => {
        impl<T: Coefficient> StabilizerDecomposedState<T> {
            $(
                pub fn $method_name(&mut self, qarg: usize) {
                    for stab in self.stabilizers.iter_mut() {
                        stab.$method_name(qarg);
                    }
                }
            )*
        }
    };
}

#[allow(dead_code)]
macro_rules! impl_two_qubit_gate {
    ( $( pub fn $method_name:ident(&mut self, $arg1:ident: usize, $arg2:ident: usize) );* ) => {
        impl<T: Coefficient> StabilizerDecomposedState<T> {
            $(
                pub fn $method_name(&mut self, $arg1: usize, $arg2: usize) {
                    for stab in self.stabilizers.iter_mut() {
                        stab.$method_name($arg1, $arg2);
                    }
                }
            )*
        }
    };
}

impl_single_qubit_gate! {
    pub fn apply_x(&mut self, qarg: usize);
    pub fn apply_y(&mut self, qarg: usize);
    pub fn apply_z(&mut self, qarg: usize);
    pub fn apply_h(&mut self, qarg: usize);
    pub fn apply_s(&mut self, qarg: usize);
    pub fn apply_sdg(&mut self, qarg: usize);
    pub fn apply_sqrt_x(&mut self, qarg: usize);
    pub fn apply_sqrt_xdg(&mut self, qarg: usize)
}

impl_two_qubit_gate! {
    pub fn apply_cx(&mut self, control: usize, target: usize);
    pub fn apply_cz(&mut self, qarg1: usize, qarg2: usize);
    pub fn apply_swap(&mut self, qarg1: usize, qarg2: usize)
}
