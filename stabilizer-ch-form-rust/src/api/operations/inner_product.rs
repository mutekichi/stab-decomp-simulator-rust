use crate::{StabilizerCHForm, error::Result};

impl StabilizerCHForm {
    /// Computes the inner product 〈self|other〉.
    ///
    /// This method works by finding a sequence of Clifford operations that
    /// transforms the state |self> into the |0...0> state, and then applying
    /// the same sequence of operations to |other>. The inner product is then
    /// derived from the resulting state's amplitude at the |0...0> basis state.
    pub fn inner_product(&self, other: &StabilizerCHForm) -> Result<num_complex::Complex64> {
        self._inner_product(other)
    }
}
