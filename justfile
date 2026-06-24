# List available commands
default:
    @just --list

# Format all code
fmt:
    cargo fmt --all

# Check formatting without modifying files (CI-safe)
fmt-check:
    cargo fmt --all -- --check

# Run clippy across the workspace, treating warnings as errors
lint:
    cargo clippy --workspace --all-targets -- -D warnings

# Run the full test suite
test:
    cargo test --workspace

# Run the TUI binary
run:
    cargo run -p stash-tui

# Run everything CI runs, locally, before you push
ci: fmt-check lint test
    @echo "All checks passed."

# Audit dependencies for known vulnerabilities
audit:
    cargo audit