use stabilizer_ch_form_rust::StabilizerCHForm;

use crate::prelude::types::coefficient::Coefficient;

#[derive(Clone, Debug)]
pub struct StabilizerDecomposedState<T: Coefficient> {
    pub num_qubits: usize,
    pub stabilizers: Vec<StabilizerCHForm>,
    pub coefficients: Vec<T>,
}
