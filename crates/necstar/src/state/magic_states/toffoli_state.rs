#![allow(dead_code)]
use crate::error::{Error, Result};
use crate::state::{StabilizerDecomposedState, types::scalar::Scalar};
use stabilizer_ch_form_rust::prelude::*;

/// Constructs the Toffoli state |Toffoli>:
/// |Toffoli> = (|000> + |100> + |010> + |111>) / 2
///           = (|0+0> + |1,Bell>) / sqrt(2)
/// as a `StabilizerDecomposedState<Scalar>`
///
/// Note: Toffoli state injection is not implemented yet.
pub(crate) fn construct_toffoli_state() -> Result<StabilizerDecomposedState<Scalar>> {
    // |0+0> part
    let mut stab1 = StabilizerCHForm::new(3)?;
    stab1.apply_h(1)?;

    // |1,Bell> part
    let mut stab2 = StabilizerCHForm::new(3)?;
    stab2.apply_x(0)?;
    stab2.apply_h(1)?;
    stab2.apply_cx(1, 2)?;

    let coeffs = vec![Scalar::ONE_OVER_SQRT_2, Scalar::ONE_OVER_SQRT_2];

    Ok(StabilizerDecomposedState::new(
        3,
        vec![stab1, stab2],
        coeffs,
    ))
}

pub(crate) fn construct_toffoli_tensor_state(
    num_tensors: usize,
) -> Result<StabilizerDecomposedState<Scalar>> {
    match num_tensors {
        0 => Err(Error::InvalidNumQubits(num_tensors)),
        1 => construct_toffoli_state(),
        _ => Ok(construct_toffoli_tensor_state(num_tensors - 1)?.kron(&construct_toffoli_state()?)?),
    }
}
