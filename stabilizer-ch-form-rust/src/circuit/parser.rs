use crate::circuit::{CliffordCircuit, CliffordGate};
use crate::error::{Error, Result};
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

pub(crate) fn from_qasm_str(qasm_str: &str) -> Result<CliffordCircuit> {
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

pub(crate) fn from_qasm_file<P: AsRef<Path>>(path: P) -> Result<CliffordCircuit> {
    let qasm_content = fs::read_to_string(path.as_ref()).map_err(|e| {
        Error::QasmParsingError(format!(
            "Failed to read file '{}': {}",
            path.as_ref().display(),
            e
        ))
    })?;

    from_qasm_str(&qasm_content)
}

pub(crate) fn to_qasm_str(circuit: &CliffordCircuit, reg_name: &str) -> String {
    let mut lines = Vec::new();
    lines.push("OPENQASM 2.0;".to_string());
    lines.push("include \"qelib1.inc\";".to_string());
    lines.push(format!("qreg {}[{}];", reg_name, circuit.n_qubits));

    for gate in &circuit.gates {
        lines.push(gate.to_qasm_str(reg_name))
    }

    lines.join("\n")
}

pub(crate) fn to_qasm_file<P: AsRef<std::path::Path>>(
    circuit: &CliffordCircuit,
    path: P,
    reg_name: &str,
) -> Result<()> {
    let qasm_str = to_qasm_str(circuit, reg_name);
    let mut file = std::fs::File::create(path)?;
    file.write_all(qasm_str.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_qasm_str() {
        let mut circuit = CliffordCircuit::new(2);
        circuit.apply_h(0);
        circuit.apply_cx(0, 1);

        let qasm_str = to_qasm_str(&circuit, "q");
        let expected_qasm = r#"OPENQASM 2.0;
include "qelib1.inc";
qreg q[2];
h q[0];
cx q[0], q[1];"#;
        assert_eq!(qasm_str, expected_qasm);
    }

    #[test]
    fn test_from_qasm_str() {
        let qasm_str = r#"OPENQASM 2.0;
include "qelib1.inc";
qreg q[2];
h q[0];
cx q[0], q[1];"#;
        let circuit = from_qasm_str(qasm_str).expect("QASM parsing failed");

        let mut expected_circuit = CliffordCircuit::new(2);
        expected_circuit.apply_h(0);
        expected_circuit.apply_cx(0, 1);

        assert_eq!(circuit.n_qubits, expected_circuit.n_qubits);
        assert_eq!(circuit.gates, expected_circuit.gates);
    }

    #[test]
    fn test_qasm_parser_roundtrip_str() {
        let n_qubits = 4;
        // Generate a random Clifford circuit
        let circuit1 = CliffordCircuit::random_clifford(n_qubits, Some([42; 32]));
        assert!(
            circuit1.gates.len() > 0,
            "Random circuit should not be empty"
        );

        // Convert to QASM string
        let qasm_str = circuit1.to_qasm_str("q");

        // Parse back from QASM string
        let circuit2 = CliffordCircuit::from_qasm_str(&qasm_str)
            .expect("QASM parsing from generated string failed");

        // Check that the circuits match
        assert_eq!(circuit1.n_qubits, circuit2.n_qubits);
        assert_eq!(
            circuit1.gates, circuit2.gates,
            "Gate sequences must match after roundtrip"
        );
    }

    #[test]
    fn test_qasm_parser_roundtrip_file() {
        // Create a temporary file path
        let mut temp_path = std::env::temp_dir();
        let file_name = format!("test_circuit_{}.qasm", std::process::id());
        temp_path.push(file_name);
        let temp_path_str = temp_path
            .to_str()
            .expect("Failed to create temp path string");

        // Generate a random circuit and write to QASM file
        let n_qubits = 3;
        let circuit1 = CliffordCircuit::random_clifford(n_qubits, Some([123; 32]));
        circuit1
            .to_qasm_file(temp_path_str, "qr")
            .expect("Failed to write QASM to file");

        // Read the circuit back from the QASM file
        let circuit2 =
            CliffordCircuit::from_qasm_file(temp_path_str).expect("Failed to read QASM from file");

        // Check that the circuits match
        assert_eq!(circuit1.n_qubits, circuit2.n_qubits);
        assert_eq!(circuit1.gates, circuit2.gates);

        // Clean up the temporary file
        fs::remove_file(temp_path).expect("Failed to remove temporary test file");
    }

    #[test]
    fn test_qasm_parser_errors() {
        // The parser should reject non-Clifford gates like T
        let qasm_t_gate = r#"
OPENQASM 2.0;
include "qelib1.inc";
qreg q[1];
t q[0];"#;

        let result = CliffordCircuit::from_qasm_str(qasm_t_gate);
        assert!(
            result.is_err(),
            "Parser should fail for non-Clifford T gate"
        );
        if let Err(e) = result {
            println!("Received expected error for T gate: {}", e);
            assert!(matches!(e, Error::QasmParsingError(_)));
        }

        // The parser should reject malformed QASM syntax (e.g., missing semicolon)
        let qasm_bad_syntax = r#"
OPENQASM 2.0;
qreg q[1]
h q[0];"#;
        assert!(
            CliffordCircuit::from_qasm_str(qasm_bad_syntax).is_err(),
            "Parser should fail on syntax error"
        );
    }
}
