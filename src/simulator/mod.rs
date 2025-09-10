pub mod api;
pub(crate) mod compiler;
pub(crate) mod magic_states;
pub(crate) mod stabilizer_decomposed_state;
pub(crate) mod types;

pub(crate) use stabilizer_decomposed_state::StabilizerDecomposedState;
pub(crate) use types::coefficient::Coefficient;

/// TODO: Add documentation for SimulatorState
#[derive(Debug, Clone)]
pub struct SimulatorState<T: Coefficient> {
    /// TODO: Add documentation for internal_state
    pub(crate) internal_state: StabilizerDecomposedState<T>,
}

impl<T: Coefficient> SimulatorState<T> {
    pub fn new(internal_state: StabilizerDecomposedState<T>) -> Self {
        Self { internal_state }
    }
}

