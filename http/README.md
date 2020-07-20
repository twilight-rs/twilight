<!-- cargo-sync-readme start -->

# twilight-http

HTTP support for the twilight ecosystem.

## Features

### Deserialization

`twilight-http` supports [`serde_json`] and [`simd-json`] for deserializing responses.

#### `simd-json`

The `simd-json` feature enables [`simd-json`] support to use simd features of
the modern cpus to deserialize responses faster. It is not enabled by
default.

To use this feature you need to also add these lines to
`<project root>/.cargo/config`:

```toml
[build]
rustflags = ["-C", "target-cpu=native"]
```

You can also set the environment variable
`RUSTFLAGS="-C target-cpu=native"`. If you enable both `serde_json` and
`simd-json` at the same time, then `simd-json` will be used.

To enable `simd-json`, do something like this in your `Cargo.toml`:

```toml
[dependencies]
twilight-http = { branch = "trunk", default-features = false, features = ["native", "simd-json"], git = "https://github.com/twilight-rs/twilight" }
```

### TLS

`twilight-http` has features to enable [`reqwest`]'s TLS features. These
features are mutually exclusive. `native` is enabled by default.

#### `native`

The `native` feature enables [`reqwest`]'s `default-tls`
feature, which is mostly equivalent to using [`native-tls`].

This is enabled by default.

#### `rustls`

The `rustls` feature enables [`reqwest`]'s `rustls` feature, which uses
[`rustls`] as the TLS backend.

To enable `rustls`, do something like this in your `Cargo.toml`:

```toml
[dependencies]
twilight-http = { branch = "trunk", default-features = false, features = ["rustls"], git = "https://github.com/twilight-rs/twilight" }
```

[`native-tls`]: https://crates.io/crates/native-tls
[`reqwest`]: https://crates.io/crates/reqwest
[`rustls`]: https://crates.io/crates/rustls
[`serde_json`]: https://crates.io/crates/serde_json
[`simd-json`]: https://crates.io/crates/simd-json

<!-- cargo-sync-readme end -->
