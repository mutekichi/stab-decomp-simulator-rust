use stabilizer_ch_form_rust::StabilizerCHForm;
use stabilizer_ch_form_rust::error::Error;
mod common;
use common::*;

#[test]
fn test_hadamard() {
    let mut ch_form = StabilizerCHForm::new(1).unwrap();
    ch_form.apply_h(0).unwrap();

    let statevec = ch_form.to_statevector().unwrap();

    let expected = ndarray::array![
        num_complex::Complex64::new(1.0 / 2f64.sqrt(), 0.0),
        num_complex::Complex64::new(1.0 / 2f64.sqrt(), 0.0)
    ];
    assert_eq_complex_array1(&statevec, &expected);
}

#[test]
fn test_bell_state() {
    let mut ch_form = StabilizerCHForm::new(2).unwrap();
    ch_form.apply_h(0).unwrap();
    ch_form.apply_cx(0, 1).unwrap();

    let statevec = ch_form.to_statevector().unwrap();

    let expected = ndarray::array![
        num_complex::Complex64::new(1.0 / 2f64.sqrt(), 0.0),
        num_complex::Complex64::new(0.0, 0.0),
        num_complex::Complex64::new(0.0, 0.0),
        num_complex::Complex64::new(1.0 / 2f64.sqrt(), 0.0)
    ];
    assert_eq_complex_array1(&statevec, &expected);
}

#[test]
fn test_gate_api_qubit_index_out_of_bounds() {
    let mut ch_form = StabilizerCHForm::new(2).unwrap();
    let result_h = ch_form.apply_h(2);
    assert!(matches!(
        result_h,
        Err(Error::QubitIndexOutOfBounds(index, total)) if index == 2 && total == 2
    ));

    let result_cx = ch_form.apply_cx(0, 2);
    assert!(matches!(
        result_cx,
        Err(Error::QubitIndexOutOfBounds(index, total)) if index == 2 && total == 2
    ));
}

#[test]
fn test_gate_api_duplicate_qubit_indices() {
    let mut ch_form = StabilizerCHForm::new(2).unwrap();
    let result_cx = ch_form.apply_cx(1, 1);
    assert!(matches!(
        result_cx,
        Err(Error::DuplicateQubitIndices(index)) if index == 1
    ));
}
