pub mod comp_basis;
pub mod inner_product;
pub mod measurement;
pub mod sample;

use stabilizer_ch_form_rust::prelude::*;

use crate::state::Coefficient;

#[derive(Clone, Debug)]
pub struct StabilizerDecomposedState<T: Coefficient> {
    pub num_qubits: usize,
    pub stabilizers: Vec<StabilizerCHForm>,
    pub coefficients: Vec<T>,
}

impl<T: Coefficient> StabilizerDecomposedState<T> {
    /// Returns a new StabilizerDecomposedState representing the tensor product of self and other.
    pub fn kron(&self, other: &Self) -> Self {
        let mut new_stabilizers = Vec::new();
        let mut new_coefficients = Vec::new();

        for (stab1, coeff1) in self.stabilizers.iter().zip(self.coefficients.iter()) {
            for (stab2, coeff2) in other.stabilizers.iter().zip(other.coefficients.iter()) {
                new_stabilizers.push(stab1.kron(stab2));
                new_coefficients.push(*coeff1 * *coeff2);
            }
        }

        StabilizerDecomposedState {
            num_qubits: self.num_qubits + other.num_qubits,
            stabilizers: new_stabilizers,
            coefficients: new_coefficients,
        }
    }
}

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
