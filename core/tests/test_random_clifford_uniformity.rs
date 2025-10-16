use ndarray::Array1;
use num_complex::Complex64;
use stab_decomp_simulator_rust::prelude::*;
use std::collections::HashMap;

use stab_decomp_simulator_rust::circuit::random_clifford::random_clifford;

/// A discrete representation of a complex amplitude of the form s * (1/sqrt(2))^r.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct DiscreteAmp {
    r: u32,    // The exponent of (1/sqrt(2)).
    phase: u8, // 0: 1, 1: i, 2: -1, 3: -i
}

/// Calculates the theoretical number of unique stabilizer states for n qubits.
fn calculate_num_stabilizer_states(n_qubits: usize) -> u64 {
    let term1 = 1u64 << n_qubits; // 2^n
    let product: u64 = (0..n_qubits)
        .map(|k| (1u64 << (n_qubits - k)) + 1)
        .product();
    term1 * product
}

/// Normalizes the global phase of a statevector in-place.
fn normalize_global_phase(sv: &mut Array1<Complex64>) {
    if let Some(first_nonzero) = sv.iter().find(|c| c.norm_sqr() > 1e-12) {
        let phase = first_nonzero.arg();
        let rotation = Complex64::new(0.0, -phase).exp();
        *sv *= rotation;
    }
}

/// Converts a complex amplitude to its discrete representation.
fn discretize_amplitude(c: &Complex64) -> Option<DiscreteAmp> {
    let norm_sqr = c.norm_sqr();
    if norm_sqr < 1e-12 {
        return None; // Treat as zero
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
        4
    }; // Error/Unknown

    if phase < 4 {
        Some(DiscreteAmp { r, phase })
    } else {
        None // Should not happen for valid stabilizer states
    }
}

/// Converts a statevector into a hashable key using discrete amplitude representation.
fn statevector_to_discrete_key(sv: &Array1<Complex64>) -> Vec<Option<DiscreteAmp>> {
    sv.iter().map(discretize_amplitude).collect()
}

/// Decodes a discrete key back into a human-readable string for display.
fn decode_key_to_string(key: &[Option<DiscreteAmp>]) -> String {
    let n_qubits = (key.len() as f64).log2() as usize;
    let mut components = Vec::new();
    for (i, amp_opt) in key.iter().enumerate() {
        if let Some(amp) = amp_opt {
            let phase_str = match amp.phase {
                0 => "+1",
                1 => "+i",
                2 => "-1",
                3 => "-i",
                _ => "?",
            };
            let norm_str = if amp.r == 0 {
                "".to_string()
            } else if amp.r == 1 {
                "/√2".to_string()
            } else {
                format!("/(√2)^{}", amp.r)
            };

            components.push(format!(
                "({}{})|{:0width$b}>",
                phase_str,
                norm_str,
                i,
                width = n_qubits
            ));
        }
    }
    if components.is_empty() {
        "0".to_string()
    } else {
        components.join(" ")
    }
}

#[test]
#[ignore]
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
        if (i > 0) && (i + 1) % 1000 == 0 {
            println!("  ... generated {} / {} circuits", i + 1, total_samples);
        }
        let circuit = random_clifford(N_QUBITS, None);
        let state = QuantumState::from_circuit(&circuit).unwrap();
        let mut sv = state.to_statevector().unwrap();

        normalize_global_phase(&mut sv);
        let key = statevector_to_discrete_key(&sv);
        *state_counts.entry(key).or_insert(0) += 1;
    }

    println!("\n--- Analysis Results ---");
    let num_observed_unique_states = state_counts.len();
    println!("Observed unique states: {}", num_observed_unique_states);

    // --- Detailed Debug Output (Decoded) ---
    println!("\n--- Detailed State Counts (sorted by occurrence) ---");
    let mut sorted_counts: Vec<_> = state_counts.iter().collect();
    sorted_counts.sort_by(|a, b| b.1.cmp(a.1));

    for (key, count) in sorted_counts {
        println!("Count: {:<5} | State: {}", count, decode_key_to_string(key));
    }
    // --- End of new section ---

    println!("\n--- Verification ---");
    let discovery_rate = num_observed_unique_states as f64 / num_unique_states_theory as f64;
    println!(
        "Discovery rate of unique states: {:.2}%",
        discovery_rate * 100.0
    );
    assert!(discovery_rate > 0.99);

    let counts: Vec<u32> = state_counts.values().cloned().collect();
    let min_count = *counts.iter().min().unwrap_or(&0);
    let max_count = *counts.iter().max().unwrap_or(&0);
    let avg_count: f64 =
        counts.iter().map(|&c| c as f64).sum::<f64>() / num_observed_unique_states as f64;

    println!("Minimum occurrences for a state: {}", min_count);
    println!("Maximum occurrences for a state: {}", max_count);
    println!("Average occurrences per state: {:.2}", avg_count);

    let lower_bound = (SAMPLES_PER_STATE as f64 * 0.5) as u32;
    let upper_bound = (SAMPLES_PER_STATE as f64 * 1.5) as u32;

    assert!(min_count >= lower_bound);
    assert!(max_count <= upper_bound);

    println!("\nUniformity test passed for {} qubits", N_QUBITS);
}
