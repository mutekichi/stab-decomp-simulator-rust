use crate::error::Result;
use crate::state::Coefficient;
use crate::state::StabilizerDecomposedState;

impl<T: Coefficient> StabilizerDecomposedState<T> {
    // Single-qubit gates
    pub fn _apply_x(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_x(qarg)?
        }
        Ok(())
    }

    pub fn _apply_y(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_y(qarg)?
        }
        Ok(())
    }

    pub fn _apply_z(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_z(qarg)?;
        }
        Ok(())
    }

    pub fn _apply_h(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_h(qarg)?;
        }
        Ok(())
    }

    pub fn _apply_s(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_s(qarg)?;
        }
        Ok(())
    }

    pub fn _apply_sdg(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_sdg(qarg)?;
        }
        Ok(())
    }

    pub fn _apply_sqrt_x(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_sqrt_x(qarg)?;
        }
        Ok(())
    }

    pub fn _apply_sqrt_xdg(&mut self, qarg: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_sqrt_xdg(qarg)?;
        }
        Ok(())
    }

    // Two-qubit gates
    pub fn _apply_cx(&mut self, control: usize, target: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_cx(control, target)?;
        }
        Ok(())
    }

    pub fn _apply_cz(&mut self, qarg1: usize, qarg2: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_cz(qarg1, qarg2)?;
        }
        Ok(())
    }

    pub fn _apply_swap(&mut self, qarg1: usize, qarg2: usize) -> Result<()> {
        for stab in self.stabilizers.iter_mut() {
            stab.apply_swap(qarg1, qarg2)?;
        }
        Ok(())
    }
}
