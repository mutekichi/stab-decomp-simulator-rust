use stabilizer_ch_form_rust::prelude::*;

#[test]

fn test_project() {
    let mut ch_form = StabilizerCHForm::new(3);
    ch_form.apply_h(0);
    ch_form.apply_cx(0, 1);
    ch_form.apply_cx(1, 2);

    dbg!(&ch_form.to_statevector());
    // Project qubit 0 onto |0>
    ch_form.project(0, false).unwrap();

    dbg!(&ch_form.to_statevector());

    ch_form.apply_h(0);
    ch_form.apply_cx(0, 1);
    ch_form.apply_cx(1, 2);
    ch_form.apply_x(2);

    ch_form.project(2, false).unwrap();

    // Now the state should be |110>
    // Discard the last qubit
    ch_form.discard(2).unwrap();

    dbg!(&ch_form.to_statevector());
}
