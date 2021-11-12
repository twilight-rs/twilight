# Install nightly toolchain
rustup toolchain install nightly

# Configure rustdoc environment
export RUSTDOCFLAGS="--cfg docsrs -D broken_intra_doc_links"

# Build crates without examples
exclude_examples=($(grep -h '^name' examples/**/Cargo.toml | cut -d'"' -f2 | xargs -I '{}' echo '--exclude {}'))
cargo +nightly doc --workspace --no-deps --features=permission-calculator "${exclude_examples[@]}"
cargo +nightly doc -p twilight-util --no-deps --all-features

# Prepare docs for publish
echo '<meta http-equiv="refresh" content="0;url=twilight/index.html">' > target/doc/index.html
