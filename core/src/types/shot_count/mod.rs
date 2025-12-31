use std::fmt::Debug;

/// Represents the result of sampling shots from a quantum state.
/// Each entry in the vector contains a tuple of:
/// 1. `Vec<bool>`: The measurement outcome for each qubit (false for `|0>`, true for `|1>`).
/// 2. `usize`: The frequency (count) of this specific outcome.
pub type ShotCount = Vec<(Vec<bool>, usize)>;

/// Trait for representing measurement outcomes (e.g. [false, false, true])
/// as integer types: u32, u64, u128 (e.g. 0b001 for the previous example).
pub(crate) trait OutcomeInteger: Copy + Sized + Debug {
    fn zero() -> Self;
    fn set_bit(self, index: usize) -> Self;
    fn to_vec_bool(self, len: usize) -> Vec<bool>;
}

macro_rules! impl_outcome_integer {
    ($type:ty) => {
        impl OutcomeInteger for $type {
            #[inline(always)]
            fn zero() -> Self {
                0
            }
            #[inline(always)]
            fn set_bit(self, index: usize) -> Self {
                self | (1 as $type) << index
            }
            #[inline(always)]
            fn to_vec_bool(self, len: usize) -> Vec<bool> {
                let mut vec = Vec::with_capacity(len);
                for i in 0..len {
                    vec.push((self >> i) & 1 == 1);
                }
                vec
            }
        }
    };
}

// We use u32, u64, or u128 to represent measurement outcomes as integers.
impl_outcome_integer!(u32);
impl_outcome_integer!(u64);
impl_outcome_integer!(u128);

pub(crate) enum SamplingBuffer {
    U32(Vec<(u32, usize)>),
    U64(Vec<(u64, usize)>),
    U128(Vec<(u128, usize)>),
}

impl SamplingBuffer {
    /// Converts the internal sampling result into the final `ShotCount` format.
    pub(crate) fn finalize(self, num_qubits: usize) -> ShotCount {
        match self {
            SamplingBuffer::U32(data) => data
                .into_iter()
                .map(|(bits, count)| (bits.to_vec_bool(num_qubits), count))
                .collect(),
            SamplingBuffer::U64(data) => data
                .into_iter()
                .map(|(bits, count)| (bits.to_vec_bool(num_qubits), count))
                .collect(),
            SamplingBuffer::U128(data) => data
                .into_iter()
                .map(|(bits, count)| (bits.to_vec_bool(num_qubits), count))
                .collect(),
        }
    }
}
