cargo fmt -- --check
cargo clippy -- -D warnings

cargo nextest run

# Runs doctests explicitly, for nextest currently doesn't support doctests
cargo test --doc

# Tests time-consuming cases
cargo test --release -- --ignored

# cargo +nightly careful test

# cargo clean
# cargo miri test

# cargo bench
