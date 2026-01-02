use ndarray::Array1;
use num_complex::Complex64;

pub(crate) fn tensor_statevectors(
    a: &Array1<Complex64>,
    b: &Array1<Complex64>,
) -> Array1<Complex64> {
    let dim_a = a.len();
    let dim_b = b.len();
    let mut res = Array1::zeros(dim_a * dim_b);

    for i in 0..dim_a {
        for j in 0..dim_b {
            res[j * dim_a + i] = a[i] * b[j];
        }
    }
    res
}
#[allow(dead_code)]
pub(crate) fn assert_eq_complex_array1(a: &Array1<Complex64>, b: &Array1<Complex64>) {
    assert_eq!(a.len(), b.len(), "Arrays have different lengths.");
    for (i, (x, y)) in a.iter().zip(b.iter()).enumerate() {
        let diff = (x - y).norm();
        assert!(
            diff <= 1e-8,
            "Arrays differ at index {}: |{} - {}| = {} > {}",
            i,
            x,
            y,
            diff,
            1e-8
        );
    }
}
