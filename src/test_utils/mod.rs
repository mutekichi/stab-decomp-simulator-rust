use crate::circuit::{QuantumCircuit, QuantumGate};
use ndarray::{Array1, Array2};
use num_complex::Complex64;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[allow(dead_code)]
pub fn assert_eq_complex(a: Complex64, b: Complex64) {
    let diff = (a - b).norm();
    assert!(
        diff <= 1e-8,
        "Complex numbers differ: |{} - {}| = {} > {}",
        a,
        b,
        diff,
        1e-8
    );
}

#[allow(dead_code)]
pub fn assert_eq_complex_array1(a: &Array1<Complex64>, b: &Array1<Complex64>) {
    assert_eq!(a.len(), b.len(), "Arrays have different lengths.");
    for (i, (x, y)) in a.iter().zip(b.iter()).enumerate() {
        let diff = (x - y).norm();
        assert!(
            diff <= 1e-8,
            "Arrays differ at index {}: |{} - {}| = {} > {}",
            i,
            x,
            y,
            diff,
            1e-8
        );
    }
}

#[allow(dead_code)]
pub fn load_statevector_from_file<P: AsRef<Path>>(
    path: P,
) -> Result<Array1<Complex64>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut vec_data = Vec::new();

    for line in reader.lines() {
        let line_content = line?;
        let parts: Vec<&str> = line_content.split(',').collect();
        if parts.len() == 2 {
            let real = parts[0]
                .trim()
                .parse::<f64>()
                .expect("Failed to parse real part");
            let imag = parts[1]
                .trim()
                .parse::<f64>()
                .expect("Failed to parse imag part");
            vec_data.push(Complex64::new(real, imag));
        }
    }
    Ok(Array1::from(vec_data))
}

/// Prints a boolean vector (`Array1<bool>`) to the console in a readable format (e.g., [0, 1, 0]).
#[allow(dead_code)]
pub fn pretty_print_bool_vec(name: &str, vec: &Array1<bool>) {
    let s: String = vec
        .iter()
        .map(|&b| if b { '1' } else { '0' })
        .collect::<String>();
    println!("{}: [{}]", name, s);
}

/// Prints a boolean matrix (`Array2<bool>`) to the console in a readable format.
#[allow(dead_code)] // This is a debug utility, so allow it to be unused in some tests
pub fn pretty_print_bool_mat(name: &str, mat: &Array2<bool>) {
    println!("{}: [", name);
    for row in mat.rows() {
        let s: String = row.iter().map(|&b| if b { '1' } else { '0' }).collect();
        println!("  {}", s);
    }
    println!("]");
}

/// Prints a complex vector (`Array1<Complex64>`) to the console in a readable format.
#[allow(dead_code)]
pub fn pretty_print_complex_vec(name: &str, vec: &Array1<Complex64>) {
    let elements: Vec<String> = vec
        .iter()
        .map(|c| format!("{:.4}", c)) // Format each complex number to 4 decimal places
        .collect();

    let formatted_vec = elements.join(", ");
    println!("{}: [{}]", name, formatted_vec);
}

/// Generates a random quantum circuit with the specified number of qubits and gates.
#[allow(dead_code)]
pub fn random_circuit_with_t_gate(
    n_qubits: usize,
    clifford_gate_count: usize,
    t_type_gate_count: usize,
    seed: Option<u64>,
) -> QuantumCircuit {
    let mut circuit = QuantumCircuit::new(n_qubits);
    let mut rng = match seed {
        Some(s) => StdRng::seed_from_u64(s),
        None => StdRng::from_entropy(),
    };

    // An enum to represent the category of gate to be generated.
    #[derive(Clone, Copy)]
    enum GateCategory {
        Clifford,
        TType,
    }

    // Create a pool of gate categories to be generated.
    let total_gates = clifford_gate_count + t_type_gate_count;
    let mut gate_categories: Vec<GateCategory> = Vec::with_capacity(total_gates);
    gate_categories.extend(std::iter::repeat(GateCategory::Clifford).take(clifford_gate_count));
    gate_categories.extend(std::iter::repeat(GateCategory::TType).take(t_type_gate_count));

    // Shuffle the pool to ensure random ordering of Clifford and T-type gates.
    gate_categories.shuffle(&mut rng);

    for category in gate_categories {
        let gate = match category {
            GateCategory::Clifford => {
                // Define the number of available Clifford gates of each arity.
                const NUM_1Q_CLIFFORDS: u32 = 8; // H, X, Y, Z, S, Sdg, SqrtX, SqrtXdg
                const NUM_2Q_CLIFFORDS: u32 = 3; // CX, CZ, Swap

                // Determine the range of possible gates based on the number of qubits.
                let max_gate_idx = if n_qubits < 2 {
                    NUM_1Q_CLIFFORDS
                } else {
                    NUM_1Q_CLIFFORDS + NUM_2Q_CLIFFORDS
                };

                let gate_idx = rng.gen_range(0..max_gate_idx);

                if gate_idx < NUM_1Q_CLIFFORDS {
                    // Generate a 1-qubit Clifford gate.
                    let q = rng.gen_range(0..n_qubits);
                    match gate_idx {
                        0 => QuantumGate::H(q),
                        1 => QuantumGate::X(q),
                        2 => QuantumGate::Y(q),
                        3 => QuantumGate::Z(q),
                        4 => QuantumGate::S(q),
                        5 => QuantumGate::Sdg(q),
                        6 => QuantumGate::SqrtX(q),
                        7 => QuantumGate::SqrtXdg(q),
                        _ => unreachable!(),
                    }
                } else {
                    // Generate a 2-qubit Clifford gate.
                    let q1 = rng.gen_range(0..n_qubits);
                    let mut q2 = rng.gen_range(0..n_qubits);
                    while q1 == q2 {
                        q2 = rng.gen_range(0..n_qubits);
                    }
                    match gate_idx - NUM_1Q_CLIFFORDS {
                        0 => QuantumGate::CX(q1, q2),
                        1 => QuantumGate::CZ(q1, q2),
                        2 => QuantumGate::Swap(q1, q2),
                        _ => unreachable!(),
                    }
                }
            }
            GateCategory::TType => {
                // Generate a T or Tdg gate.
                let q = rng.gen_range(0..n_qubits);
                if rng.gen_bool(0.5) {
                    QuantumGate::T(q)
                } else {
                    QuantumGate::Tdg(q)
                }
            }
        };
        circuit.apply_gate(gate);
    }

    circuit
}

#[allow(dead_code)]
pub fn _norm_squared(v: &Array1<Complex64>) -> f64 {
    v.iter().map(|c| c.norm_sqr()).sum()
}
