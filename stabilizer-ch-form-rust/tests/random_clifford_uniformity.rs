use ndarray::Array1;
use num_complex::Complex64;
use stabilizer_ch_form_rust::prelude::*;
use std::collections::HashMap;

// --- Helper Structs and Functions for Uniformity Test ---

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
/// Discrete representation of a complex amplitude for stabilizer states.
/// r: exponent for 2^(-r/2) magnitude
/// phase: 0 (1), 1 (i), 2 (-1), 3 (-i)
struct DiscreteAmp {
    r: u32,
    phase: u8, // 0: 1, 1: i, 2: -1, 3: -i
}

/// Calculate the total number of unique stabilizer states for n qubits (up to global phase).
/// Formula: 2^n * ∏_{k=0}^{n-1} (2^{n-k} + 1)
fn calculate_num_stabilizer_states(n_qubits: usize) -> u64 {
    let term1 = 1u64 << n_qubits;
    let product: u64 = (0..n_qubits)
        .map(|k| (1u64 << (n_qubits - k)) + 1)
        .product();
    term1 * product
}

/// Normalize the global phase of a statevector so that the first non-zero amplitude is real and positive.
fn normalize_global_phase(sv: &mut Array1<Complex64>) {
    if let Some(first_nonzero) = sv.iter().find(|c| c.norm_sqr() > 1e-12) {
        let phase = first_nonzero.arg();
        let rotation = Complex64::new(0.0, -phase).exp();
        *sv *= rotation;
    }
}

/// Discretize a complex amplitude into (r, phase) representation.
/// Returns None if the amplitude is effectively zero.
fn discretize_amplitude(c: &Complex64) -> Option<DiscreteAmp> {
    let norm_sqr = c.norm_sqr();
    if norm_sqr < 1e-12 {
        return None;
    }

    let r = (-norm_sqr.log2()).round() as u32;
    let scaled_c = c * 2.0_f64.powf(r as f64 / 2.0);

    let phase = if (scaled_c.re - 1.0).abs() < 1e-6 {
        0
    } else if (scaled_c.im - 1.0).abs() < 1e-6 {
        1
    } else if (scaled_c.re + 1.0).abs() < 1e-6 {
        2
    } else if (scaled_c.im + 1.0).abs() < 1e-6 {
        3
    } else {
        return None; // Not a clean stabilizer amplitude
    };
    Some(DiscreteAmp { r, phase })
}

/// Convert a statevector into a discrete key representation.
fn statevector_to_discrete_key(sv: &Array1<Complex64>) -> Vec<Option<DiscreteAmp>> {
    sv.iter().map(discretize_amplitude).collect()
}

#[test]
fn test_random_clifford_uniformity() {
    // --- Test Configuration ---
    const N_QUBITS: usize = 3;
    const SAMPLES_PER_STATE: u32 = 100;
    // --- End Configuration ---

    let num_unique_states_theory = calculate_num_stabilizer_states(N_QUBITS);
    let total_samples = num_unique_states_theory * SAMPLES_PER_STATE as u64;

    println!(
        "Testing uniformity for {} qubits. Expecting {} unique states.",
        N_QUBITS, num_unique_states_theory
    );
    println!("Total samples to generate: {}", total_samples);

    let mut state_counts: HashMap<Vec<Option<DiscreteAmp>>, u32> = HashMap::new();

    for i in 0..total_samples {
        if (i > 0) && (i + 1) % 10000 == 0 {
            println!(
                "  ... generated {} / {} circuits ({} unique states found)",
                i + 1,
                total_samples,
                state_counts.len()
            );
        }

        // 1. Use the public API of CliffordCircuit to generate the circuit
        let circuit = CliffordCircuit::random_clifford(N_QUBITS, None);

        // 2. Convert to CHForm to get the statevector
        let state = StabilizerCHForm::from_clifford_circuit(&circuit)
            .expect("Failed to create CHForm from random circuit");
        let mut sv = state.to_statevector().expect("Failed to get statevector");

        // 3. Normalize and discretize the statevector
        normalize_global_phase(&mut sv);
        let key = statevector_to_discrete_key(&sv);

        // 4. Count occurrences
        *state_counts.entry(key).or_insert(0) += 1;
    }

    // --- Analysis ---
    println!("\n--- Analysis Results ---");
    let num_observed_unique_states = state_counts.len();
    println!("Observed unique states: {}", num_observed_unique_states);

    let discovery_rate = num_observed_unique_states as f64 / num_unique_states_theory as f64;
    println!(
        "Discovery rate of unique states: {:.2}%",
        discovery_rate * 100.0
    );

    let counts: Vec<u32> = state_counts.values().cloned().collect();
    let min_count = *counts.iter().min().unwrap_or(&0);
    let max_count = *counts.iter().max().unwrap_or(&0);
    let avg_count: f64 =
        counts.iter().map(|&c| c as f64).sum::<f64>() / num_observed_unique_states as f64;

    println!("Minimum occurrences for a state: {}", min_count);
    println!("Maximum occurrences for a state: {}", max_count);
    println!("Average occurrences per state: {:.2}", avg_count);

    // --- Assertions ---
    assert!(
        discovery_rate > 0.99,
        "Did not discover almost all states ({} / {})",
        num_observed_unique_states,
        num_unique_states_theory
    );

    let lower_bound = (SAMPLES_PER_STATE as f64 * 0.5) as u32;
    let upper_bound = (SAMPLES_PER_STATE as f64 * 1.5) as u32;

    assert!(
        min_count >= lower_bound,
        "Some states appeared too rarely (min: {})",
        min_count
    );
    assert!(
        max_count <= upper_bound,
        "Some states appeared too often (max: {})",
        max_count
    );

    println!("\nUniformity test passed for {} qubits", N_QUBITS);
}
