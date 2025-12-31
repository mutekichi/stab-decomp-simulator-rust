use crate::error::Error;
use crate::error::Result;
use lazy_static::lazy_static;
use regex::Regex;
use std::{fmt, str::FromStr};

mod pauli_string;
mod pauli_term;

pub use pauli_string::Pauli;
pub use pauli_term::PauliTerm;

/// Represents a multi-qubit Pauli operator.
/// You can represent it in either dense or sparse format:
/// - Dense: `"IXYZ"` ( Q0=Z, Q1=Y, Q2=X, Q3=I )
/// - Sparse: `"X1 Y3"` ( Q1=X, Q3=Y, others=I )
///
/// Note that the dense format follows the little-endian convention (Qiskit-style), where the
/// rightmost character corresponds to the 0-th qubit.
///
/// ## Examples
/// ```rust
/// use stabilizer_ch_form_rust::types::pauli::{ Pauli, PauliTerm, PauliString };
///
/// // Parsing dense Pauli string
/// let dense: PauliString = "IXYZ".parse().unwrap();
/// assert_eq!(dense, PauliString::Dense(vec![Pauli::Z, Pauli::Y, Pauli::X, Pauli::I]));
///
/// // Parsing sparse Pauli string
/// let sparse: PauliString = "X1 Y3".parse().unwrap();
/// assert_eq!(sparse, PauliString::Sparse(vec![
///     PauliTerm { op: Pauli::X, qubit: 1 },
///     PauliTerm { op: Pauli::Y, qubit: 3 },
/// ]));
///
/// // Identity Pauli string
/// let identity_sparse: PauliString = "".parse().unwrap();
/// let identity_dense: PauliString = "IIII".parse().unwrap();
/// assert!(identity_sparse.is_identity());
/// assert!(identity_dense.is_identity());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PauliString {
    Dense(Vec<Pauli>),
    Sparse(Vec<PauliTerm>),
}

/// Parses a dense Pauli string like "IXYZ".
/// The string is assumed to be in little-endian format (Q0 is rightmost),
/// matching Qiskit's Pauli string convention.
fn parse_dense(s: &str) -> Result<PauliString> {
    let mut ops = Vec::with_capacity(s.len());
    for (i, char) in s.chars().enumerate() {
        match char {
            'I' => ops.push(Pauli::I),
            'X' => ops.push(Pauli::X),
            'Y' => ops.push(Pauli::Y),
            'Z' => ops.push(Pauli::Z),
            _ => {
                return Err(Error::PauliStringParsingError(format!(
                    "invalid character '{}' at position {} in dense PauliString",
                    char, i
                )));
            }
        }
    }

    // Reverse the vector to store in little-endian (index 0 = Qubit 0)
    // "IX" (Q1=I, Q0=X) -> parsed as [I, X] -> reversed to [X, I]
    ops.reverse();

    Ok(PauliString::Dense(ops))
}

/// Parses a sparse Pauli string like "X1 Y3".
fn parse_sparse(s: &str) -> Result<PauliString> {
    lazy_static! {
        // Regex to match a single term EXACTLY (e.g., "X12", "Y3")
        static ref TERM_RE: Regex = Regex::new(r"^(?i)([XYZ])(\d+)$").unwrap();
    }

    let mut terms = Vec::new();

    if s.is_empty() {
        return Ok(PauliString::Sparse(vec![]));
    }

    let mut seen_qubits = std::collections::BTreeSet::new();
    for term_str in s.split_whitespace() {
        if let Some(cap) = TERM_RE.captures(term_str) {
            let op_char = cap.get(1).unwrap().as_str();
            let index_str = cap.get(2).unwrap().as_str();

            let op = match op_char.to_uppercase().as_str() {
                "X" => Pauli::X,
                "Y" => Pauli::Y,
                "Z" => Pauli::Z,
                _ => unreachable!(), // Regex ensures this
            };
            let qubit = index_str.parse::<usize>().map_err(|_| {
                Error::PauliStringParsingError(format!("invalid qubit index: {}", index_str))
            })?;
            if seen_qubits.contains(&qubit) {
                return Err(Error::PauliStringParsingError(format!(
                    "duplicate Pauli operator on qubit {} in string: '{}'",
                    qubit, s
                )));
            }
            terms.push(PauliTerm { op, qubit });
            seen_qubits.insert(qubit);
        } else {
            return Err(Error::PauliStringParsingError(format!(
                "invalid sparse Pauli term: '{}' in string: '{}'",
                term_str, s
            )));
        }
    }

    Ok(PauliString::Sparse(terms))
}

/// Implements `FromStr` for PauliString to allow parsing from strings.
impl FromStr for PauliString {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let trimmed = s.trim();

        // Handle empty string or "I" (case insensitive) as identity.
        if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("i") {
            return Ok(PauliString::identity());
        }

        // Heuristic to decide format: check for digits.
        let has_digits = trimmed.chars().any(|c| c.is_ascii_digit());

        if has_digits {
            parse_sparse(trimmed)
        } else {
            parse_dense(trimmed)
        }
    }
}

impl PauliString {
    /// Generates the identity Pauli string.
    pub fn identity() -> Self {
        PauliString::Sparse(vec![])
    }

    /// Checks if the Pauli string is the identity operator.
    pub fn is_identity(&self) -> bool {
        match self {
            PauliString::Sparse(terms) => terms.is_empty(),
            PauliString::Dense(ops) => ops.iter().all(|&op| op == Pauli::I),
        }
    }
}

impl fmt::Display for PauliString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PauliString::Dense(ops) => {
                // ops is little-endian (Q0 at index 0)
                let s: String = ops
                    .iter()
                    .rev() // Reverse to little-endian string (Q0 at rightmost)
                    .map(|op| match op {
                        Pauli::I => 'I',
                        Pauli::X => 'X',
                        Pauli::Y => 'Y',
                        Pauli::Z => 'Z',
                    })
                    .collect();

                if s.is_empty() {
                    // Handle 0-qubit case
                    write!(f, "I")
                } else {
                    write!(f, "{}", s)
                }
            }
            PauliString::Sparse(terms) => {
                if terms.is_empty() {
                    return write!(f, "I");
                }
                let s: String = terms
                    .iter()
                    .map(|term| {
                        let op_char = match term.op {
                            Pauli::I => 'I', // Should not happen in sparse
                            Pauli::X => 'X',
                            Pauli::Y => 'Y',
                            Pauli::Z => 'Z',
                        };
                        format!("{}{}", op_char, term.qubit)
                    })
                    .collect::<Vec<String>>()
                    .join(" ");
                write!(f, "{}", s)
            }
        }
    }
}
