# NECSTAR: NEar-Clifford STAbilizer decomposition simulator in Rust

A high-performance quantum circuit simulator designed for the strong simulation of
near-Clifford circuits based on the stabilizer decomposition method [1].

NECSTAR is particularly effective for circuits dominated by Clifford gates
but also containing a small number of non-Clifford gates. Currently, NECSTAR supports
only T-gates as non-Clifford operations, but future versions may include additional
non-Clifford gates.

## Features

* **Stabilizer Decomposition Core**: The simulator represents the quantum state as a linear combination of stabilizer states [1]. This approach avoids the memory overhead of dense state vectors and is efficient for circuits with low non-Clifford gate counts.
* **Magic State Teleportation**: Non-Clifford gates are applied via the gate teleportation protocol using magic states. A T-gate is implemented by consuming a T-state, and the tensor product of T-states is automatically decomposed into stabilizer states using optimized techniques [2].
* **Intuitive Declarative API**: Users can define quantum computations by building a :class:`~necstar.QuantumCircuit`. This is compiled into a :class:`~necstar.QuantumState`, which manages the internal stabilizer decomposition and provides a clean interface for simulation.

## Repository Structure

The project is organized as a Rust workspace with the following components:

| Directory | Package Name | Description |
| --- | --- | --- |
| [`crates/necstar`](https://github.com/mutekichi/necstar/tree/main/crates/necstar) | `necstar` | The main Rust core crate containing the simulation logic. |
| [`crates/stabilizer-ch-form-rust`](https://github.com/mutekichi/necstar/tree/main/crates/stabilizer-ch-form-rust) | `stabilizer-ch-form-rust` | A sub-crate dedicated to fast stabilizer state manipulation using CH-form. |
| [`python/`](https://github.com/mutekichi/necstar/tree/main/python) | `necstar` (on PyPI) | Python bindings and high-level API for the simulator. |

## Quick Start

### Python (Recommended)

For Python users, NECSTAR can be installed via `pip`:

```bash
pip install necstar

```

Please refer to the [Python README](https://github.com/mutekichi/necstar/tree/main/python/README.md) for more details.

### Rust

To use NECSTAR as a library in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
necstar = "0.1.0"

```

Please refer to the [NECSTAR README](https://github.com/mutekichi/necstar/tree/main/crates/necstar/README.md) for more details.

## Statement of Need

(TODO: Describe the problem this project solves and who would benefit from it.)

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/mutekichi/necstar/blob/main/LICENSE) file for details.

## References

* [1] S. Bravyi, D. Browne, P. Calpin, E. Campbell, D. Gosset, and M. Howard, "Simulation of quantum circuits by low-rank stabilizer decompositions", Quantum 3, 181 (2019). <https://doi.org/10.22331/q-2019-09-02-181>
* [2] H. Qassim, H. Pashayan, and D. Gosset, "Improved upper bounds on the stabilizer rank of magic states", Quantum 5, 604 (2021). <https://doi.org/10.22331/q-2021-12-20-606>

## Authors

* **Yuki Watanabe** - [ywatanabe.r5p@gmail.com](mailto:ywatanabe.r5p@gmail.com)
