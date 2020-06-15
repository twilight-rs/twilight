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

### Runtimes

`twilight-http` supports some of the popular async runtimes. The
`tokio-runtime` feature is enabled by default.

#### `smol-runtime`

Use [`smol`] as an asynchronous executor for background tasks. The
[`futures-timer`] crate will be used for asynchronous timing. If you're
using `smol` in your runtime, then you'll want to disable the
`tokio-runtime` feature and enable this, like so:

```toml
[dependencies.twilight-http]
default-features = false
features = ["serde_json", "smol-runtime"]
git = "https://github.com/twilight-rs/twilight"
```

#### `tokio-runtime`

Use [`tokio`] as an asynchronous executor for background tasks and
asynchronous timing. If you're using `tokio` in your application, this is
what you want and you don't need to change anything in your dependencies.

[`futures-timer`]: https://crates.io/crates/futures-timer
[`simd-json`]: https://crates.io/crates/simd-json
[`smol`]: https://crates.io/crates/smol
[`tokio`]: https://crates.io/crates/tokio

<!-- cargo-sync-readme end -->
