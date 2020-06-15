<!-- cargo-sync-readme start -->

# twilight-http

HTTP support for the twilight ecosystem.

## Features

### Deserialization

`twilight-http` supports `serde_json` and `simd-json` for deserializing
responses. `serde_json` is enabled by default.

#### `simd-json`

The `simd-json` feature enables [`simd-json`] support to use simd features of
the modern cpus to deserialize responses faster. It is not enabled by
default, and instead the `serde_json` feature is enabled by default.

To use this feature you need to also add these lines to
`<project root>/.cargo/config`:
```toml
[build]
rustflags = ["-C", "target-cpu=native"]
```

You can also set the environment variable
`RUSTFLAGS="-C target-cpu=native"`. If you enable both `serde_json` and
`simd-json` at the same time, then `simd-json` will be used.

#### `serde_json`

`serde_json` is the inverse of `simd-json` and will use the `serde_json`
crate to deserialize responses.

[`simd-json`]: https://crates.io/crates/simd-json

<!-- cargo-sync-readme end -->
