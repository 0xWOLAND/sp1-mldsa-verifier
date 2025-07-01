# Build the program
build:
    cd program && cargo prove build

# Execute the program without generating a proof
execute variant="2":
    just build
    cd script && RUST_LOG=info cargo run --release -- --execute --variant {{variant}}

# Generate a proof (default: Groth16)
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