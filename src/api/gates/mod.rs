use crate::prelude::{types::coefficient::Coefficient, StabilizerDecomposedState};
use stabilizer_ch_form_rust::prelude::*;

/// Implements single-qubit gate methods on StabilizerDecomposedState
/// by forwarding the call to each stabilizer in the decomposition.
macro_rules! impl_single_qubit_gate_on_state {
    ( $( pub fn $method_name:ident(&mut self, qarg: usize) );* ) => {
        impl<T: Coefficient> StabilizerDecomposedState<T> {
            $(
                /// Applies the corresponding gate to a specific qubit.
                pub fn $method_name(&mut self, qarg: usize) {
                    for stab in self.stabilizers.iter_mut() {
                        stab.$method_name(qarg);
                    }
                }
            )*
        }
    };
}

// Use the macro to generate the methods.
impl_single_qubit_gate_on_state! {
    pub fn apply_x(&mut self, qarg: usize);
    pub fn apply_y(&mut self, qarg: usize);
    pub fn apply_z(&mut self, qarg: usize);
    pub fn apply_h(&mut self, qarg: usize);
    pub fn apply_s(&mut self, qarg: usize);
    pub fn apply_sdg(&mut self, qarg: usize);
    pub fn apply_sqrt_x(&mut self, qarg: usize)
}


macro_rules! impl_two_qubit_gate_on_state {
    ( $( pub fn $method_name:ident(&mut self, $($param:ident: $param_type:ty),*) );* ) => {
        impl<T: Coefficient> StabilizerDecomposedState<T> {
            $(
                pub fn $method_name(&mut self, $($param: $param_type),*) {
                    for stab in self.stabilizers.iter_mut() {
                        stab.$method_name($($param),*);
                    }
                }
            )*
        }
    };
}

impl_two_qubit_gate_on_state! {
    pub fn apply_cx(&mut self, control: usize, target: usize);
    pub fn apply_cz(&mut self, qarg1: usize, qarg2: usize);
    pub fn apply_swap(&mut self, qarg1: usize, qarg2: usize)
}