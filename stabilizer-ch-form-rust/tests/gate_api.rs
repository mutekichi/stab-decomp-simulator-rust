use stabilizer_ch_form_rust::StabilizerCHForm;
use stabilizer_ch_form_rust::prelude::*;
mod common;

#[test]
fn test_hadamard() {
    let mut ch_form = StabilizerCHForm::new(1);
    ch_form.apply_h(0);

    let statevec = ch_form.to_statevector();
    println!("Statevector after H: {:?}", statevec);
    let expected = ndarray::array![
        num_complex::Complex64::new(1.0 / 2f64.sqrt(), 0.0),
        num_complex::Complex64::new(1.0 / 2f64.sqrt(), 0.0)
    ];
    common::assert_eq_complex_array1(&statevec, &expected);
}

#[test]
fn test_bell_state() {
    let mut ch_form = StabilizerCHForm::new(2);
    ch_form.apply_h(0);
    ch_form.apply_cx(0, 1);

    let statevec = ch_form.to_statevector();
    // ndarray::array![1.0, 0.0, 0.0, 1.0] / 2f64.sqrt();
    let expected = ndarray::array![
        num_complex::Complex64::new(1.0 / 2f64.sqrt(), 0.0),
        num_complex::Complex64::new(0.0, 0.0),
        num_complex::Complex64::new(0.0, 0.0),
        num_complex::Complex64::new(1.0 / 2f64.sqrt(), 0.0)
    ];
    common::assert_eq_complex_array1(&statevec, &expected);
}

#[test]
fn test_hadamard_behaviour() {
    let mut ch_form = StabilizerCHForm::new(3);
    ch_form.apply_h(0);
    ch_form.apply_s(0);
    ch_form.apply_cx(0, 1);
    ch_form.apply_cx(1, 2);
    ch_form.apply_cz(0, 2);
    ch_form.apply_h(2);
    ch_form.apply_s(1);
    ch_form.apply_z(0);
    ch_form.apply_cz(0, 1);
    ch_form.apply_h(0);

    let internal_state = ch_form.get_internal_state();
    internal_state.pretty_print();
}
