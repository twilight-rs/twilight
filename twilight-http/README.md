# twilight-http

[![codecov badge][]][codecov link] [![discord badge][]][discord link] [![github badge][]][github link] [![license badge][]][license link] ![rust badge]

HTTP support for the twilight ecosystem.

## Examples

There are a few usage examples located in the [root of the `twilight`
repository][github examples link].

## Features

### Decompression

The `decompression` feature enables brotli decompression support via the [`brotli`] crate.

This is enabled by default.

### Deserialization

`twilight-http` supports [`serde_json`] and [`simd-json`] for deserializing
responses.

#### `simd-json`

The `simd-json` feature enables [`simd-json`] support to use simd features
of modern cpus to deserialize responses faster. It is not enabled by
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
twilight-http = { default-features = false, features = ["rustls-native-roots", "simd-json"], version = "0.2" }
```

### TLS

**Note**: not enabling any TLS feature is supported for use behind a proxy;
Discord's API is HTTPS only.

`twilight-http` has features to enable HTTPS connectivity with [`hyper`]. These
features are mutually exclusive. `rustls-native-roots` is enabled by default.

#### `native-tls`

The `native-tls` feature uses a HTTPS connector provided by [`hyper-tls`].

To enable `native-tls`, do something like this in your `Cargo.toml`:

```toml
[dependencies]
twilight-http = { default-features = false, features = ["native"], version = "0.2" }
```

#### `rustls-native-roots`

The `rustls-native-roots` feature uses a HTTPS connector provided by [`hyper-rustls`], which uses
[`rustls`] as the TLS backend, and enables its `native-tokio` feature, which uses [`rustls-native-certs`]
for root certificates.

This is enabled by default.

#### `rustls-webpki-roots`

The `rustls-webpki-roots` feature uses a HTTPS connector provided by [`hyper-rustls`], which uses
[`rustls`] as the TLS backend, and enables its `webpki-tokio` feature, which uses [`webpki-roots`]
for root certificates.

This should be preferred over `rustls-native-roots` in Docker containers based on `scratch`.

### Trust-DNS

The `hickory` feature enables [`hyper-hickory`], which replaces the default
`GaiResolver` in [`hyper`]. [`hyper-hickory`] instead provides a fully async
DNS resolver on the application level.

[`brotli`]: https://github.com/dropbox/rust-brotli
[`hyper`]: https://crates.io/crates/hyper
[`hyper-hickory`]: https://crates.io/crates/hyper-hickory
[`hyper-rustls`]: https://crates.io/crates/hyper-rustls
[`hyper-tls`]: https://crates.io/crates/hyper-tls
[`rustls`]: https://crates.io/crates/rustls
[`rustls-native-certs`]: https://crates.io/crates/rustls-native-certs
[`serde_json`]: https://crates.io/crates/serde_json
[`simd-json`]: https://crates.io/crates/simd-json
[`webpki-roots`]: https://crates.io/crates/webpki-roots
[codecov badge]: https://img.shields.io/codecov/c/gh/twilight-rs/twilight?logo=codecov&style=for-the-badge&token=E9ERLJL0L2
[codecov link]: https://app.codecov.io/gh/twilight-rs/twilight/
[discord badge]: https://img.shields.io/discord/745809834183753828?color=%237289DA&label=discord%20server&logo=discord&style=for-the-badge
[discord link]: https://discord.gg/7jj8n7D
[github badge]: https://img.shields.io/badge/github-twilight-6f42c1.svg?style=for-the-badge&logo=github
[github examples link]: https://github.com/twilight-rs/twilight/tree/main/examples
[github link]: https://github.com/twilight-rs/twilight
[license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=for-the-badge&logo=pastebin
[license link]: https://github.com/twilight-rs/twilight/blob/main/LICENSE.md
[rust badge]: https://img.shields.io/badge/rust-1.67+-93450a.svg?style=for-the-badge&logo=rust
