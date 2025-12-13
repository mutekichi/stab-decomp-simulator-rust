use crate::circuit::{CliffordCircuit, CliffordGate};
use crate::error::{Error, Result};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

pub(crate) fn _from_qasm_str(qasm_str: &str) -> Result<CliffordCircuit> {
    lazy_static::lazy_static! {
        static ref QREG_RE: Regex = Regex::new(
            r"qreg\s+([a-zA-Z][a-zA-Z0-9_]*)\s*\[\s*(\d+)\s*\]\s*;"
        ).unwrap();
        static ref GATE1_RE: Regex = Regex::new(
            r"([a-z_]+)\s+([a-zA-Z][a-zA-Z0-9_]*)\[(\d+)\]\s*;"
        ).unwrap();
        static ref GATE2_RE: Regex = Regex::new(
            r"([a-z_]+)\s+([a-zA-Z][a-zA-Z0-9_]*)\[(\d+)\],\s*([a-zA-Z][a-zA-Z0-9_]*)\[(\d+)\]\s*;"
        ).unwrap();

        static ref SINGLE_QUBIT_GATES: HashMap<&'static str, fn(usize) -> CliffordGate> = {
            let mut m = HashMap::new();
            m.insert("h", CliffordGate::H as fn(usize) -> CliffordGate);
            m.insert("x", CliffordGate::X as fn(usize) -> CliffordGate);
            m.insert("y", CliffordGate::Y as fn(usize) -> CliffordGate);
            m.insert("z", CliffordGate::Z as fn(usize) -> CliffordGate);
            m.insert("s", CliffordGate::S as fn(usize) -> CliffordGate);
            m.insert("sdg", CliffordGate::Sdg as fn(usize) -> CliffordGate);
            m.insert("sx", CliffordGate::SqrtX as fn(usize) -> CliffordGate);
            m.insert("sxdg", CliffordGate::SqrtXdg as fn(usize) -> CliffordGate);
            m
        };

        static ref TWO_QUBIT_GATES: HashMap<&'static str, fn(usize, usize) -> CliffordGate> = {
            let mut m = HashMap::new();
            m.insert("cx", CliffordGate::CX as fn(usize, usize) -> CliffordGate);
            m.insert("cz", CliffordGate::CZ as fn(usize, usize) -> CliffordGate);
            m.insert("swap", CliffordGate::Swap as fn(usize, usize) -> CliffordGate);
            m
        };
    }

    let mut n_qubits: Option<usize> = None;
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
            if n_qubits.is_some() {
                return Err(Error::QasmParsingError(
                    "Multiple qreg declarations are not supported.".to_string(),
                ));
            }
            let size = caps[2].parse::<usize>().map_err(|_| {
                Error::QasmParsingError(format!("Invalid qreg size in line: {}", line))
            })?;
            n_qubits = Some(size);
            continue;
        }

        if line.starts_with("measure") {
            eprintln!(
                "[Warning] Line {}: `measure` operation is ignored by the parser.",
                line_num + 1
            );
            continue;
        }

        if let Some(caps) = GATE2_RE.captures(line) {
            let gate_name = &caps[1];
            if let Some(gate_fn) = TWO_QUBIT_GATES.get(gate_name) {
                let q1 = caps[3].parse::<usize>().map_err(|_| {
                    Error::QasmParsingError(format!("Invalid qubit index in line: {}", line))
                })?;
                let q2 = caps[5].parse::<usize>().map_err(|_| {
                    Error::QasmParsingError(format!("Invalid qubit index in line: {}", line))
                })?;
                gates.push(gate_fn(q1, q2));
                continue;
            }
        }

        if let Some(caps) = GATE1_RE.captures(line) {
            let gate_name = &caps[1];
            if let Some(gate_fn) = SINGLE_QUBIT_GATES.get(gate_name) {
                let qarg = caps[3].parse::<usize>().map_err(|_| {
                    Error::QasmParsingError(format!("Invalid qubit index in line: {}", line))
                })?;
                gates.push(gate_fn(qarg));
                continue;
            }
        }

        return Err(Error::QasmParsingError(format!(
            "Unrecognized or malformed line: {}",
            line
        )));
    }

    if let Some(n) = n_qubits {
        Ok(CliffordCircuit { n_qubits: n, gates })
    } else {
        Err(Error::QasmParsingError(
            "qreg declaration not found in QASM string.".to_string(),
        ))
    }
}

pub(crate) fn _from_qasm_file<P: AsRef<Path>>(path: P) -> Result<CliffordCircuit> {
    let qasm_content = fs::read_to_string(path.as_ref()).map_err(|e| {
        Error::QasmParsingError(format!(
            "Failed to read file '{}': {}",
            path.as_ref().display(),
            e
        ))
    })?;

    _from_qasm_str(&qasm_content)
}

pub(crate) fn _to_qasm_str(circuit: &CliffordCircuit, reg_name: &str) -> String {
    let mut lines = Vec::new();
    lines.push("OPENQASM 2.0;".to_string());
    lines.push("include \"qelib1.inc\";".to_string());
    lines.push(format!("qreg {}[{}];", reg_name, circuit.n_qubits));

    for gate in &circuit.gates {
        lines.push(gate.to_qasm_str(reg_name))
    }

    lines.join("\n")
}

pub(crate) fn _to_qasm_file<P: AsRef<std::path::Path>>(
    circuit: &CliffordCircuit,
    path: P,
    reg_name: &str,
) -> Result<()> {
    let qasm_str = _to_qasm_str(circuit, reg_name);
    let mut file = std::fs::File::create(path)?;
    file.write_all(qasm_str.as_bytes())?;
    Ok(())
}
