# necstar QuickStart

## 🐍 For Python Users

### Requirements

    * Python 3.11+
  * Rust (and Cargo)
  * `maturin` (Python package)

### Local Installation

1.  Install `maturin`:

    ```bash
    pip install maturin
    ```

2.  From the root of this repository, build and install `necstar` into your environment:

    ```bash
    # (Recommended) Create and activate a virtual environment first
    # python -m venv .venv
    # source .venv/bin/activate

    # Build and install in editable mode
    maturin develop
    ```

> **Note on Requirements:** This local installation method builds the package from source, which **requires a Rust compiler (Rust and Cargo) to be installed**.
>
> If this package were published to PyPI with pre-compiled *wheels*, most users could simply run `pip install necstar` **without needing Rust**. The Rust requirement applies only when installing from source (like this local method, or if PyPI lacks a wheel for your specific platform).

### Example Usage

```python
import necstar

# 1. Create a 2-qubit circuit
qc = necstar.QuantumCircuit(2)

# 2. Apply gates (including non-Clifford T-gate)
qc.apply_h(0)
qc.apply_cx(0, 1)
qc.apply_t(1)

try:
    # 3. Compile the circuit into a state
    state = necstar.QuantumState.from_circuit(qc)
    print(f"State stabilizer rank (χ): {state.stabilizer_rank}")

    # 4. Calculate an expectation value
    pauli_z0 = necstar.PauliString.from_str("ZI")
    exp_val = state.exp_value(pauli_z0)
    print(f"Expectation value of Z0: {exp_val}")

    # 5. Sample measurement outcomes
    samples = state.sample(qargs=[0, 1], shots=1024, seed=42)
    print(f"Samples: {samples}")

except ValueError as e:
    print(f"An error occurred: {e}")

```

-----

## 🦀 For Rust Users

### Requirements

  * Rust (and Cargo)

### Local Setup

Add the `necstar` crate (the core library) to your `Cargo.toml` using a local path reference.

```toml
[dependencies]
# Adjust the path to point to the 'core' directory in this repository
necstar = { path = "../path/to/necstar/core" }
```

### Example Usage

```rust
use necstar::prelude::{QuantumCircuit, QuantumState, Result};
use necstar::types::PauliString;
use std::str::FromStr;

fn main() -> Result<()> {
    // 1. Create a 2-qubit circuit
    let mut qc = QuantumCircuit::new(2);

    // 2. Apply gates (including non-Clifford T-gate)
    qc.apply_h(0);
    qc.apply_cx(0, 1);
    qc.apply_t(1);

    // 3. Compile the circuit into a state
    let state = QuantumState::from_circuit(&qc)?;
    println!("State stabilizer rank (χ): {}", state.stabilizer_rank());

    // 4. Calculate an expectation value
    let pauli_z0 = PauliString::from_str("ZI").unwrap();
    let exp_val = state.exp_value(&pauli_z0)?;
    println!("Expectation value of Z0: {}", exp_val);

    // 5. Sample measurement outcomes
    let samples = state.sample(&[0, 1], 1024, None)?;
    println!("Samples: {:?}", samples);

    Ok(())
}
```

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.