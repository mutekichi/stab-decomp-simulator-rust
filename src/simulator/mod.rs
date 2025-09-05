pub mod types;
pub mod stabilizer_decomposed_state;
pub mod magic_states;

pub use stabilizer_decomposed_state::StabilizerDecomposedState;
pub use types::coefficient::Coefficient;

/// TODO: Add documentation for SimulatorState
#[derive(Debug, Clone)]
pub struct SimulatorState<T: Coefficient> {
    /// TODO: Add documentation for internal_state
    pub(crate) internal_state: StabilizerDecomposedState<T>,
}