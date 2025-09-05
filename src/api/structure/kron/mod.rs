use crate::prelude::{StabilizerDecomposedState, types::coefficient::Coefficient};

impl<T: Coefficient> StabilizerDecomposedState<T> {
    pub fn kron(&self, other: &Self) -> Self {
        let num_qubits = self.num_qubits + other.num_qubits;
        let mut stabilizers = Vec::new();
        let mut coefficients = Vec::new();

        for (stab1, coeff1) in self.stabilizers.iter().zip(self.coefficients.iter()) {
            for (stab2, coeff2) in other.stabilizers.iter().zip(other.coefficients.iter()) {
                let new_stab = stab1.kron(stab2);
                let new_coeff = *coeff1 * *coeff2;
                stabilizers.push(new_stab);
                coefficients.push(new_coeff);
            }
        }

        StabilizerDecomposedState {
            num_qubits,
            stabilizers,
            coefficients,
        }
    }
}
