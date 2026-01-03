# stabilizer-ch-form-rust: Fast Stabilizer State Manipulation using CH-form

A Rust library for simulating stabilizer states using the CH-form representation based on the reference [1].
This representation keeps track of the global phase unlike the typical tableau representation.

## Getting Started
### Use from *crates.io*
To use `stabilizer-ch-form-rust` as a library in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
stabilizer-ch-form-rust = "0.1.1"
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
    stabilizer-ch-form-rust = { path = "/path/to/necstar/crates/stabilizer-ch-form-rust" }
    ```

    Replace `/path/to/necstar` with the actual path to the cloned repository.

## Documentation
See the [Documentation](https://mutekichi.github.io/necstar/rust/stabilizer_ch_form_rust/) for more details and examples.

## References

* [1] S. Bravyi, D. Browne, P. Calpin, E. Campbell, D. Gosset, and M. Howard, "Simulation of quantum circuits by low-rank stabilizer decompositions", Quantum 3, 181 (2019). <https://doi.org/10.22331/q-2019-09-02-181>