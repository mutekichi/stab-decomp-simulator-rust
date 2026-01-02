use rand::prelude::*;
use stabilizer_ch_form_rust::prelude::*;

mod common;

#[test]
fn test_random_circuit_vs_matrix_reference() {
    let mut rng = rand::thread_rng();

    // Number of qubits for the test.
    // Kept small (e.g., 5) to ensure the matrix reference simulation runs in reasonable time (O(2^N)).
    const N: usize = 10;

    // Define gate counts based on the requested ratios
    let count_1q = 10 * N;
    let count_h = 20 * N;
    let count_2q = N * N * 2;

    let mut gates = Vec::new();

    // 1. Generate random single-qubit gates (excluding Hadamard)
    // Available 1-qubit gates: X, Y, Z, S, Sdg, SqrtX, SqrtXdg
    let single_qubit_ops = [
        |q| CliffordGate::X(q),
        |q| CliffordGate::Y(q),
        |q| CliffordGate::Z(q),
        |q| CliffordGate::S(q),
        |q| CliffordGate::Sdg(q),
        |q| CliffordGate::SqrtX(q),
        |q| CliffordGate::SqrtXdg(q),
    ];

    for _ in 0..count_1q {
        for op in &single_qubit_ops {
            let q = rng.gen_range(0..N);
            gates.push(op(q));
        }
    }

    // 2. Generate random Hadamard gates
    for _ in 0..count_h {
        let q = rng.gen_range(0..N);
        gates.push(CliffordGate::H(q));
    }

    // 3. Generate random two-qubit gates
    // Available 2-qubit gates: CX, CZ, Swap
    let two_qubit_ops = [
        |c, t| CliffordGate::CX(c, t),
        |c, t| CliffordGate::CZ(c, t),
        |c, t| CliffordGate::Swap(c, t),
    ];

    for _ in 0..count_2q {
        let q1 = rng.gen_range(0..N);
        let mut q2 = rng.gen_range(0..N);
        // Ensure control and target are different
        while q1 == q2 {
            q2 = rng.gen_range(0..N);
        }
        for op in &two_qubit_ops {
            gates.push(op(q1, q2));
        }
    }

    // Shuffle all gates to randomly interleave different gate types
    gates.shuffle(&mut rng);

    // Build the circuit
    let mut circuit = CliffordCircuit::new(N);
    circuit.add_gates(gates);

    println!(
        "Testing random circuit with {} qubits and {} gates...",
        N,
        circuit.gates.len()
    );

    // Run Simulator (CH-Form)
    let ch_form = StabilizerCHForm::from_clifford_circuit(&circuit)
        .expect("Failed to simulate circuit with CH-Form");
    let sim_sv = ch_form
        .to_statevector()
        .expect("Failed to generate statevector from CH-Form");

    // Run Reference (Matrix Multiplication)
    let ref_sv = common::get_reference_statevector(&circuit);

    // Assert consistency between simulator and reference
    common::assert_eq_complex_array1(&sim_sv, &ref_sv);
}
