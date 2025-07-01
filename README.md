# ML-DSA Verifier on SP1

This repository provides a minimal verifier for ML-DSA (Dilithium) digital signatures, implemented in Rust and designed to run on the [SP1 zkVM](https://sp1.succinct.xyz/). It contains separate optimized circuits for each ML-DSA variant (44, 65, 87) using the `crystals-dilithium` library.

## Quick Start

Build and execute the verifier (requires Rust and [SP1 toolchain](https://sp1.succinct.xyz/)):

```sh
just build           # Build all SP1 programs
just execute 2       # Run ML-DSA-44 verification
just execute 3       # Run ML-DSA-65 verification
just execute 5       # Run ML-DSA-87 verification
```

To generate proofs (Groth16 by default):

```sh
just prove 2         # Prove ML-DSA-44 verification
just prove 3         # Prove ML-DSA-65 verification
just prove 5         # Prove ML-DSA-87 verification
```
