# Build all programs
build:
    cd program && cargo prove build

# Execute ML-DSA variant (2=ML-DSA-44, 3=ML-DSA-65, 5=ML-DSA-87)
execute variant="2":
    just build
    cd script && RUST_LOG=info cargo run --release -- --execute --variant {{variant}}

# Generate proof for ML-DSA variant (default: Groth16)
prove variant="2" mode="groth16":
    cd script && RUST_LOG=info cargo run --release -- --prove --variant {{variant}} --proof-mode {{mode}}

# Clean build artifacts
clean:
    cargo clean
    cd program && cargo clean
    cd script && cargo clean

# Run all variants (execute only)
test-all:
    just execute 2
    just execute 3
    just execute 5