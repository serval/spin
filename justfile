# List available recipes
help:
    just -l

# Build all targets in debug mode
@build:
    cargo build --release --all-targets

# Build all targets in release mode
@release:
    cargo build --release --all-targets

# Build documentation for all crates
@doc *FLAGS:
    cargo doc --release --no-deps --workspace {{FLAGS}}

# Run the same checks we run in CI
@ci: test
    cargo clippy
    cargo fmt --check
    cargo deny check licenses

# Get security advisories from cargo-deny
@security:
    cargo deny check advisories

# Run tests with nextest
@test:
    cargo nextest run --all-targets

# Lint and automatically fix what we can fix
@lint:
    cargo clippy --fix --allow-dirty --allow-staged
    cargo fmt

# Install required linting/testing tools via cargo.
@install-tools:
    cargo install cargo-nextest
    cargo install cargo-deny

# Check for unused dependencies.
check-unused:
    cargo +nightly udeps --all

# Install into your PATH
@install:
    cargo install --path .

# Uninstall into your PATH
@uninstall:
    cargo uninstall $(cat Cargo.toml|grep ^name\ =|head -n1|cut -d'"' -f2)
