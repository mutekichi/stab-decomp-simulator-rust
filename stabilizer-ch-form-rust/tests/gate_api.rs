use stabilizer_ch_form_rust::StabilizerCHForm;
mod common;

#[test]
fn test_hadamard() {
    let mut ch_form = StabilizerCHForm::new(1).unwrap();
    ch_form.apply_h(0).unwrap();

    let statevec = ch_form.to_statevector().unwrap();
    println!("Statevector after H: {:?}", statevec);
    let expected = ndarray::array![
        num_complex::Complex64::new(1.0 / 2f64.sqrt(), 0.0),
        num_complex::Complex64::new(1.0 / 2f64.sqrt(), 0.0)
    ];
    common::assert_eq_complex_array1(&statevec, &expected);
}

#[test]
fn test_bell_state() {
    let mut ch_form = StabilizerCHForm::new(2).unwrap();
    ch_form.apply_h(0).unwrap();
    ch_form.apply_cx(0, 1).unwrap();

    let statevec = ch_form.to_statevector().unwrap();
    // ndarray::array![1.0, 0.0, 0.0, 1.0] / 2f64.sqrt();
    let expected = ndarray::array![
        num_complex::Complex64::new(1.0 / 2f64.sqrt(), 0.0),
        num_complex::Complex64::new(0.0, 0.0),
        num_complex::Complex64::new(0.0, 0.0),
        num_complex::Complex64::new(1.0 / 2f64.sqrt(), 0.0)
    ];
    common::assert_eq_complex_array1(&statevec, &expected);
}
