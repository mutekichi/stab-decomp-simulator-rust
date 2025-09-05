use num_complex::Complex64;
use stabilizer_ch_form_rust::StabilizerCHForm;

#[derive(Clone, Debug)]
pub struct StabilizerDecomposedState {
    pub num_qubits: usize,
    pub stabilizers: Vec<StabilizerCHForm>,
    pub coefficients: Vec<Complex64>,
}
