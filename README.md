# ML-DSA Verifier on SP1

This repository provides a minimal verifier for ML-DSA (Dilithium) digital signatures, implemented in Rust and designed to run on the [SP1 zkVM](https://sp1.succinct.xyz/). It supports verification for three Dilithium variants (44, 65, 87) using the `crystals-dilithium` library.

## Quick Start

Build and execute the verifier (requires Rust and [SP1 toolchain](https://sp1.succinct.xyz/)):

```sh
just build           # Build the SP1 program
just execute 2       # Run verification for variant 44
just execute 3       # Run verification for variant 65
just execute 5       # Run verification for variant 87
```

To generate a proof (Groth16 by default):

```sh
just prove 2         # Prove for variant 44
just prove 3         # Prove for variant 65
just prove 5         # Prove for variant 87
```

## Project Structure
- `program/`   — SP1 zkVM program for signature verification
- `script/`    — CLI for running and proving verification
- `dilithium-src/` — Pure Rust implementation of CRYSTALS-Dilithium (GPL-3.0)

## License
- Main repository: MIT License (see `LICENSE-MIT`)
- `dilithium-src/`: GPL-3.0 License (see `dilithium-src/LICENSE`) 