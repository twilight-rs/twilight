# twilight-http

[![codecov badge][]][codecov link] [![discord badge][]][discord link] [![github badge][]][github link] [![license badge][]][license link] ![rust badge]

HTTP support for the twilight ecosystem.

## Examples

There are a few usage examples located in the [root of the `twilight`
repository][github examples link].

## Features

### Crypto provider

Using [`rustls`] for TLS requires configuring a crypto provider via crate
features or manually installing a global default. The default is `rustls-ring`.

#### `rustls-ring`

The `rustls-ring` feature will enable the use of [`ring`] as the crypto
provider. This is recommended for platform compatibility.

#### `rustls-aws_lc_rs`

The `rustls-aws_lc_rs` feature will enable the use of [`aws-lc-rs`] as the
crypto provider. This is recommended for performance and on widely used
platforms.

#### Manual installation

If none of the other crypto providers are enabled, a custom one must be
installed by the application using [`CryptoProvider::install_default`].

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
features are mutually exclusive. `rustls-platform-verifier` is enabled by default.

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
for root certificates. This requires configuring a crypto provider.

#### `rustls-platform-verifier`

The `rustls-platform-verifier` feature uses a HTTPS connector provided by [`hyper-rustls`], which uses
[`rustls`] as the TLS backend, and enables its [`rustls-platform-verifier`] feature, which uses
[`rustls-platform-verifier`] for certificate validation. This requires configuring a crypto provider.

This is enabled by default.

#### `rustls-webpki-roots`

The `rustls-webpki-roots` feature uses a HTTPS connector provided by [`hyper-rustls`], which uses
[`rustls`] as the TLS backend, and enables its `webpki-tokio` feature, which uses [`webpki-roots`]
for root certificates. This requires configuring a crypto provider.

This should be preferred over `rustls-native-roots` in Docker containers based on `scratch`.

### Trust-DNS

The `hickory` feature enables [`hyper-hickory`], which replaces the default
`GaiResolver` in [`hyper`]. [`hyper-hickory`] instead provides a fully async
DNS resolver on the application level.

[`CryptoProvider::install_default`]: https://docs.rs/rustls/latest/rustls/crypto/struct.CryptoProvider.html#method.install_default
[`aws-lc-rs`]: https://crates.io/crates/aws-lc-rs
[`brotli`]: https://github.com/dropbox/rust-brotli
[`hyper`]: https://crates.io/crates/hyper
[`hyper-hickory`]: https://crates.io/crates/hyper-hickory
[`hyper-rustls`]: https://crates.io/crates/hyper-rustls
[`hyper-tls`]: https://crates.io/crates/hyper-tls
[`ring`]: https://crates.io/crates/ring
[`rustls`]: https://crates.io/crates/rustls
[`rustls-native-certs`]: https://crates.io/crates/rustls-native-certs
[`rustls-platform-verifier`]: https://crates.io/crates/rustls-platform-verifier
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
[rust badge]: https://img.shields.io/badge/rust-1.79+-93450a.svg?style=for-the-badge&logo=rust
