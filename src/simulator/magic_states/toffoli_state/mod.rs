use stabilizer_ch_form_rust::prelude::*;

use crate::simulator::{types::scalar::Scalar, StabilizerDecomposedState};

/// Constructs the Toffoli state |Toffoli>:
/// |Toffoli> = (|000> + |100> + |010> + |111>) / 2
///           = (|0+0> + |1,Bell>) / sqrt(2)
/// as a StabilizerDecomposedState<Scalar>
pub(crate) fn _construct_toffoli_state() -> StabilizerDecomposedState<Scalar> {
    // |0+0> part
    let mut stab1 = StabilizerCHForm::new(3);
    stab1.apply_h(1);

    // |1,Bell> part
    let mut stab2 = StabilizerCHForm::new(3);
    stab2.apply_x(0);
    stab2.apply_h(1);
    stab2.apply_cx(1, 2);

    let coeffs = vec![Scalar::ONE_OVER_SQRT_2, Scalar::ONE_OVER_SQRT_2];

    StabilizerDecomposedState {
        num_qubits: 3,
        stabilizers: vec![stab1, stab2],
        coefficients: coeffs,
    }
}

pub(crate) fn _construct_toffoli_tensor_state(
    num_tensors: usize,
) -> StabilizerDecomposedState<Scalar> {
    match num_tensors {
        0 => panic!("Number of Toffoli states must be at least 1"),
        1 => _construct_toffoli_state(),
        _ => _construct_toffoli_tensor_state(num_tensors - 1).kron(&_construct_toffoli_state()),
    }
}
