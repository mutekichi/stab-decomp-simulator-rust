use num_complex::Complex64;

use crate::state::{Coefficient, StabilizerDecomposedState};



impl<T: Coefficient> StabilizerDecomposedState<T> {
    /// calculates the norm of the state
    /// NOTE: This ignores the global factor
    pub(crate) fn _norm_squared(&self) -> f64 {
        let mut sum = Complex64::new(0.0, 0.0);
        for i in 0..self.coefficients.len() - 1{ 
            for j in i+1..self.coefficients.len(){
                let inner_prod = self.stabilizers[i].inner_product(&self.stabilizers[j]);
                sum += (self.coefficients[i].conj() * self.coefficients[j]).into() * inner_prod;
            }
        }
        // assert that the result is real
        assert!(sum.im.abs() < 1e-10);
        sum.re
    }

    /// calculates the norm of the state
    pub(crate) fn _norm(&self) -> f64 {
        self._norm_squared().sqrt()
    }
}