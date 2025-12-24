use crate::circuit::QuantumCircuit;
use crate::circuit::QuantumGate;
use crate::error::{Error, Result};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

/// Parses an OpenQASM 2.0 string into a `QuantumCircuit`.
///
/// ### Arguments
/// * `qasm_str` - A string slice containing the OpenQASM 2.0 circuit description.
///
/// ### Returns
/// A `Result` containing the parsed `QuantumCircuit` or a `String` error message.
pub(crate) fn from_qasm_str(qasm_str: &str) -> Result<QuantumCircuit> {
    type Gate1Fn = fn(usize) -> QuantumGate;
    type Gate2Fn = fn(usize, usize) -> QuantumGate;
    type Gate3Fn = fn(usize, usize, usize) -> QuantumGate;

    lazy_static::lazy_static! {
        static ref QREG_RE: Regex = Regex::new(r"qreg\s+([a-zA-Z][a-zA-Z0-9_]*)\s*\[\s*(\d+)\s*\]\s*;").unwrap();
        static ref GATE1_RE: Regex = Regex::new(r"([a-z_]+)\s+([a-zA-Z][a-zA-Z0-9_]*)\[(\d+)\]\s*;").unwrap();
        static ref GATE2_RE: Regex = Regex::new(r"([a-z_]+)\s+([a-zA-Z][a-zA-Z0-9_]*)\[(\d+)\],\s*([a-zA-Z][a-zA-Z0-9_]*)\[(\d+)\]\s*;").unwrap();
        static ref GATE3_RE: Regex = Regex::new(r"([a-z_]+)\s+([a-zA-Z][a-zA-Z0-9_]*)\[(\d+)\],\s*([a-zA-Z][a-zA-Z0-9_]*)\[(\d+)\],\s*([a-zA-Z][a-zA-Z0-9_]*)\[(\d+)\]\s*;").unwrap();

        static ref SINGLE_QUBIT_GATES: HashMap<&'static str, Gate1Fn> = {
            let mut m = HashMap::new();
            m.insert("h", QuantumGate::H as fn(usize) -> QuantumGate);
            m.insert("x", QuantumGate::X as fn(usize) -> QuantumGate);
            m.insert("y", QuantumGate::Y as fn(usize) -> QuantumGate);
            m.insert("z", QuantumGate::Z as fn(usize) -> QuantumGate);
            m.insert("s", QuantumGate::S as fn(usize) -> QuantumGate);
            m.insert("sdg", QuantumGate::Sdg as fn(usize) -> QuantumGate);
            m.insert("sx", QuantumGate::SqrtX as fn(usize) -> QuantumGate);
            m.insert("sxdg", QuantumGate::SqrtXdg as fn(usize) -> QuantumGate);
            m.insert("t", QuantumGate::T as Gate1Fn);
            m.insert("tdg", QuantumGate::Tdg as fn(usize) -> QuantumGate);
            m
        };

        static ref TWO_QUBIT_GATES: HashMap<&'static str, Gate2Fn> = {
            let mut m = HashMap::new();
            m.insert("cx", QuantumGate::CX as fn(usize, usize) -> QuantumGate);
            m.insert("cz", QuantumGate::CZ as fn(usize, usize) -> QuantumGate);
            m.insert("swap", QuantumGate::Swap as fn(usize, usize) -> QuantumGate);
            m
        };

        static ref THREE_QUBIT_GATES: HashMap<&'static str, Gate3Fn> = {
            let mut m = HashMap::new();
            m.insert("ccx", QuantumGate::CCX as fn(usize, usize, usize) -> QuantumGate);
            m
        };
    }

    let mut num_qubits: Option<usize> = None;
    let mut gates = Vec::new();

    for (line_num, line_content) in qasm_str.lines().enumerate() {
        let line = line_content.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        if line.starts_with("OPENQASM") || line.starts_with("include") {
            continue;
        }

        if let Some(caps) = QREG_RE.captures(line) {
            if num_qubits.is_some() {
                return Err(Error::QasmParsingError(
                    "Multiple qreg declarations are not supported.".to_string(),
                ));
            }
            let size = caps[2].parse::<usize>().map_err(|e| {
                Error::QasmParsingError(format!("Invalid qreg size in line: '{}' ({})", line, e))
            })?;
            num_qubits = Some(size);
            continue;
        }

        if line.starts_with("measure") {
            eprintln!(
                "[Warning] Line {}: `measure` operation is ignored by the parser.",
                line_num + 1
            );
            continue;
        }

        let mut matched = false;

        // Check for 3-qubit gates first (most specific)
        if let Some(caps) = GATE3_RE.captures(line) {
            let gate_name = &caps[1];
            if let Some(gate_fn) = THREE_QUBIT_GATES.get(gate_name) {
                let q1 = caps[3].parse::<usize>().map_err(|e| {
                    Error::QasmParsingError(format!(
                        "Invalid qubit index in line: '{}' ({})",
                        line, e
                    ))
                })?;
                let q2 = caps[5].parse::<usize>().map_err(|e| {
                    Error::QasmParsingError(format!(
                        "Invalid qubit index in line: '{}' ({})",
                        line, e
                    ))
                })?;
                let q3 = caps[7].parse::<usize>().map_err(|e| {
                    Error::QasmParsingError(format!(
                        "Invalid qubit index in line: '{}' ({})",
                        line, e
                    ))
                })?;
                gates.push(gate_fn(q1, q2, q3));
                matched = true;
            }
        }

        // Check for 2-qubit gates if not matched
        if !matched && let Some(caps) = GATE2_RE.captures(line) {
            let gate_name = &caps[1];
            if let Some(gate_fn) = TWO_QUBIT_GATES.get(gate_name) {
                let q1 = caps[3].parse::<usize>().map_err(|e| {
                    Error::QasmParsingError(format!(
                        "Invalid qubit index in line: '{}' ({})",
                        line, e
                    ))
                })?;
                let q2 = caps[5].parse::<usize>().map_err(|e| {
                    Error::QasmParsingError(format!(
                        "Invalid qubit index in line: '{}' ({})",
                        line, e
                    ))
                })?;
                gates.push(gate_fn(q1, q2));
                matched = true;
            }
        }

        // Check for 1-qubit gates if not matched
        if !matched && let Some(caps) = GATE1_RE.captures(line) {
            let gate_name = &caps[1];
            if let Some(gate_fn) = SINGLE_QUBIT_GATES.get(gate_name) {
                let qarg = caps[3].parse::<usize>().map_err(|e| {
                    Error::QasmParsingError(format!(
                        "Invalid qubit index in line: '{}' ({})",
                        line, e
                    ))
                })?;
                gates.push(gate_fn(qarg));
                matched = true;
            }
        }

        if !matched {
            return Err(Error::QasmParsingError(format!(
                "Unrecognized or malformed line: {}",
                line
            )));
        }
    }

    if let Some(n) = num_qubits {
        Ok(QuantumCircuit {
            num_qubits: n,
            gates,
        })
    } else {
        Err(Error::QasmParsingError(
            "qreg declaration not found in QASM string.".to_string(),
        ))
    }
}

/// Parses an OpenQASM 2.0 file into a `QuantumCircuit`.
///
/// ### Arguments
/// * `path` - A reference to a path of the OpenQASM 2.0 file.
/// ### Returns
/// A `Result` containing the parsed `QuantumCircuit` or a `String` error message.
pub(crate) fn from_qasm_file<P: AsRef<Path>>(path: P) -> Result<QuantumCircuit> {
    let qasm_content = fs::read_to_string(path.as_ref()).map_err(|e| {
        Error::QasmParsingError(format!(
            "Failed to read file '{}': {}",
            path.as_ref().display(),
            e
        ))
    })?;

    from_qasm_str(&qasm_content)
}

pub(crate) fn to_qasm_str(circuit: &QuantumCircuit, reg_name: &str) -> String {
    let mut lines = Vec::new();
    lines.push("OPENQASM 2.0;".to_string());
    lines.push("include \"qelib1.inc\";".to_string());
    lines.push(format!("qreg {}[{}];", reg_name, circuit.num_qubits));

    for gate in &circuit.gates {
        lines.push(gate.to_qasm_str(reg_name));
    }

    lines.join("\n")
}

/// Writes the circuit to an OpenQASM 2.0 file.
///
/// # Arguments
/// * `path` - The path to the output file.
/// * `reg_name` - The name of the quantum register (e.g., "q").
pub(crate) fn to_qasm_file<P: AsRef<Path>>(
    circuit: &QuantumCircuit,
    path: P,
    reg_name: &str,
) -> Result<()> {
    let qasm_str = circuit.to_qasm_str(reg_name);
    let mut file = std::fs::File::create(path)?;
    file.write_all(qasm_str.as_bytes())?;
    Ok(())
}
