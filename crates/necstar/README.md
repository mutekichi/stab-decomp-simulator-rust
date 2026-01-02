# NECSTAR: NEar-Clifford STAbilizer decomposition simulator in Rust

A high-performance quantum circuit simulator designed for the strong simulation of
near-Clifford circuits based on the stabilizer decomposition method.

NECSTAR is particularly effective for circuits dominated by Clifford gates
but also containing a small number of non-Clifford gates. Currently, NECSTAR supports
only T-gates as non-Clifford operations, but future versions may include additional
non-Clifford gates.

## Getting Started

### Use from *crates.io*
To use NECSTAR as a library in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
necstar = "0.1.0"
```

### Build from Source
If you want to modify the source code, you can set up the project locally.

1. Clone the repository:

    ```bash
    git clone git@github.com:mutekichi/necstar.git
    ```

2. Specify the path dependency in your `Cargo.toml`:

    ```toml
    [dependencies]
    necstar = { path = "/path/to/necstar/crates/necstar" }
    ```
    
    Replace `/path/to/necstar` with the actual path to the cloned repository.

## Example Usage
```rust
use necstar::prelude::{QuantumCircuit, QuantumState};
use necstar::types::PauliString;
use std::str::FromStr;

// 1. Build a quantum circuit
let mut circuit = QuantumCircuit::new(2);
circuit.apply_h(0);
circuit.apply_cx(0, 1);
circuit.apply_t(1); // Non-Clifford T-gate

// 2. Compile into a QuantumState (internally decomposes T-states)
let mut state = QuantumState::from_circuit(&circuit).unwrap();

// 3. Perform operations
let shots = 1024;
let samples = state.sample(&[0, 1], shots, None).unwrap();
println!("Samples: {:?}", samples);

let pauli_z0 = PauliString::from_str("ZI").unwrap();
let exp_val = state.exp_value(&pauli_z0).unwrap();
println!("Expectation value: {}", exp_val);

// (Optional) Check the stabilizer rank
println!("Stabilizer rank: {}", state.stabilizer_rank());
```

See the [Documentation](TODO:LINK) for more details and examples.