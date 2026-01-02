#[derive(Debug, PartialEq, Eq)]
/// Represents the state of a single qubit.
/// - `Determined(bool)`: The qubit is in a determined state, where `true` represents |1⟩
///   and `false` represents |0⟩.
/// - `Superposition`: The qubit is in a superposition state |+⟩ or |−⟩.
pub(crate) enum QubitState {
    Determined(bool),
    Superposition,
}
