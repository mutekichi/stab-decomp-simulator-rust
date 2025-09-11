use std::{fmt, str::FromStr};
use lazy_static::lazy_static;
use regex::Regex;
use crate::types::pauli::{pauli_string::Pauli, pauli_term::PauliTerm};

pub mod pauli_string;
pub mod pauli_term;

/// TODO: Documentation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PauliString {
    Dense(Vec<Pauli>),
    Sparse(Vec<PauliTerm>),
}
/// Parses a dense Pauli string like "IXYZ".
fn parse_dense(s: &str) -> Result<PauliString, String> {
    let mut ops = Vec::with_capacity(s.len());
    for (i, char) in s.chars().enumerate() {
        match char {
            'I' => ops.push(Pauli::I),
            'X' => ops.push(Pauli::X),
            'Y' => ops.push(Pauli::Y),
            'Z' => ops.push(Pauli::Z),
            _ => return Err(format!("invalid Pauli character '{}' at position {}", char, i)),
        }
    }
    Ok(PauliString::Dense(ops))
}

/// Parses a sparse Pauli string like "X1 Y3".
fn parse_sparse(s: &str) -> Result<PauliString, String> {
    lazy_static! {
        static ref SPARSE_RE: Regex = Regex::new(r"(?i)\s*([XYZ])\s*(\d+)\s*").unwrap();
    }

    let mut terms = Vec::new();
    for cap in SPARSE_RE.captures_iter(s) {
        let op_char = cap.get(1).unwrap().as_str();
        let index_str = cap.get(2).unwrap().as_str();

        let op = match op_char.to_uppercase().as_str() {
            "X" => Pauli::X,
            "Y" => Pauli::Y,
            "Z" => Pauli::Z,
            _ => unreachable!(), // Regex ensures this
        };
        let qubit = index_str.parse::<usize>().map_err(|_| format!("invalid qubit index: {}", index_str))?;
        terms.push(PauliTerm { op, qubit });
    }

    // Check if the entire string was parsed successfully.
    let parsed_len: usize = SPARSE_RE.find_iter(s).map(|m| m.as_str().len()).sum();
    // Also consider the length of surrounding whitespace that is not part of any match
    let total_trimmed_len = s.trim_start().trim_end().len();
    if parsed_len != total_trimmed_len {
        return Err(format!("failed to fully parse sparse PauliString: '{}'", s));
    }

    Ok(PauliString::Sparse(terms))
}

/// Implements FromStr for PauliString to allow parsing from strings.
impl FromStr for PauliString {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed = s.trim();

        // Handle empty string or `"I"` (case insensitive) as identity.
        if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("i") {
            return Ok(PauliString::identity());
        }

        // Heuristic to decide format: check for digits.
        // This is more robust than checking for whitespace.
        let has_digits = trimmed.chars().any(|c| c.is_ascii_digit());

        if has_digits {
            parse_sparse(trimmed)
        } else {
            parse_dense(trimmed)
        }
    }
}

impl PauliString {
    /// Generates the identity Pauli string for any number of qubits.
    /// Here, we treat `Sparse(vec![])` as the canonical identity representation.
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
                let s: String = ops.iter().map(|op| match op {
                    Pauli::I => 'I',
                    Pauli::X => 'X',
                    Pauli::Y => 'Y',
                    Pauli::Z => 'Z',
                }).collect();
                write!(f, "{}", s)
            }
            PauliString::Sparse(terms) => {
                let s: String = terms
                    .iter()
                    .map(|term| {
                        let op_char = match term.op {
                            // Sparse format should not contain I, but we handle it just in case.
                            Pauli::I => 'I',
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