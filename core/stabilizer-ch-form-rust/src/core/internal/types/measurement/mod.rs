#[derive(Debug, PartialEq, Eq)]
pub enum QubitState {
    Determined(bool),
    Superposition,
}
