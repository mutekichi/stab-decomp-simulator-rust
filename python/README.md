# NECSTAR: NEar-Clifford STAbilizer decomposition simulator in Rust (Python Bindings)

A high-performance quantum circuit simulator designed for the strong simulation of
near-Clifford circuits based on the stabilizer decomposition method.

NECSTAR is particularly effective for circuits dominated by Clifford gates
but also containing a small number of non-Clifford gates. Currently, NECSTAR supports
only T-gates as non-Clifford operations, but future versions may include additional
non-Clifford gates.

## Getting Started

You can install NECSTAR either via `pip` or by building from source.

### Install via pip (Recommended)
```bash
pip install necstar
```

*Note: Pre-built binaries are provided for major platforms. If a binary is not available for your system, the Rust toolchain is required to build from source.*

### Build from Source

Ensure you have the Rust toolchain installed. Then, clone the repository and build the Python package:

1. Clone the repository:

    ```bash
    git clone git@github.com:mutekichi/necstar.git
    cd necstar
    ```

2. (recommended) Create and activate a virtual environment:

    ```bash
    python -m venv venv
    source venv/bin/activate  # On Windows use `venv\Scripts\activate`
    ```

3. Build and install the package:

    ```bash
    pip install -e .
    ```

    Since NECSTAR uses `maturin` as its build backend, `pip` will automatically handle the Rust compilation. This requires that you have the Rust toolchain installed on your system.


## Example Usage

```python
import necstar

# 1. Build a quantum circuit
qc = necstar.QuantumCircuit(2)
qc.apply_h(0)
qc.apply_cx(0, 1)
qc.apply_t(1) # Non-Clifford T-gate

# 2. Compile into a QuantumState (internally decomposes T-states)
state = necstar.QuantumState.from_circuit(qc)

# 3. Perform operations
shots = 1024
samples = state.sample(qargs=[0, 1], shots=shots)
print(f"Samples: {samples}")

pauli_z0 = necstar.PauliString.from_str("ZI")
exp_val = state.exp_value(pauli_z0)
print(f"Expectation value: {exp_val}")

# (Optional) Check the stabilizer rank
print(f"Stabilizer rank: {state.stabilizer_rank}")
```

See the [Documentation](TODO:LINK) for more details and examples.