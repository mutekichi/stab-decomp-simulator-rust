#[derive(Debug, PartialEq, Eq)]
pub(crate) enum QubitState {
    Determined(bool),
    Superposition,
}
