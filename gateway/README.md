<!-- cargo-sync-readme start -->

# twilight-gateway

[![codecov badge][]][codecov link] [![discord badge][]][discord link] [![github badge][]][github link] [![license badge][]][license link] ![rust badge]

`twilight-gateway` is an implementation of Discord's sharding gateway sessions.
This is responsible for receiving stateful events in real-time from Discord
and sending *some* stateful information.

It includes two primary types: the Shard and Cluster.

The Shard handles a single websocket connection and can manage up to 2500
guilds. If you manage a small bot in under about 2000 guilds, then this is
what you use. See the [Discord Docs/Sharding][docs:discord:sharding] for
more information on sharding.

The Cluster is an interface which manages the health of the shards it
manages and proxies all of their events under one unified stream. This is
useful to use if you have a large bot in over 1000 or 2000 guilds.

## Examples

There are a few usage examples located in the [root of the `twilight`
repository][github examples link].

## Features

### Deserialization

`twilight-gateway` supports [`serde_json`] and [`simd-json`] for
deserializing and serializing events.

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
you can also use this environment variable `RUSTFLAGS="-C target-cpu=native"`.

```toml
[dependencies]
twilight-gateway = { default-features = false, features = ["rustls-native-roots", "simd-json"], version = "0.2" }
```

### TLS

`twilight-gateway` has features to enable [`tokio-tungstenite`] and
[`twilight-http`]'s TLS features. These features are mutually exclusive.
`rustls-native-roots` is enabled by default.

#### `native`

The `native` feature enables [`tokio-tungstenite`]'s `native-tls`
feature as well as [`twilight-http`]'s `native` feature which is mostly
equivalent to using [`native-tls`].

To enable `native`, do something like this in your `Cargo.toml`:

```toml
[dependencies]
twilight-gateway = { default-features = false, features = ["native"], version = "0.2" }
```

#### `rustls-native-roots`

The `rustls-native-roots` feature enables [`tokio-tungstenite`]'s `rustls-tls-native-roots` feature and
[`twilight-http`]'s `rustls-native-roots` feature, which use [`rustls`] as the TLS backend and [`rustls-native-certs`]
for root certificates.

This is enabled by default.

#### `rustls-webpki-roots`

The `rustls-webpki-roots` feature enables [`tokio-tungstenite`]'s `rustls-tls-webpki-roots` feature and
[`twilight-http`]'s `rustls-webpki-roots` feature, which use [`rustls`] as the TLS backend and [`webpki-roots`]
for root certificates.

This should be preferred over `rustls-native-roots` in Docker containers based on `scratch`.

### zlib

zlib compression is enabled with one of the two `zlib` features described below.

There are 2 zlib features `zlib-stock` and `zlib-simd`, if both are enabled it
will use `zlib-stock`.

`zlib-stock` is enabled by default.

Enabling **only** `zlib-simd` will make the library use [`zlib-ng`] which is a modern
fork of zlib that is faster and more effective, but it needs `cmake` to compile.

### Tracing

The `tracing` feature enables logging via the [`tracing`] crate.

This is enabled by default.

### Metrics

The `metrics` feature provides metrics information via the `metrics` crate.
Some of the metrics logged are counters about received event counts and
their types and gauges about the capacity and efficiency of the inflater of
each shard.

This is disabled by default.

[`native-tls`]: https://crates.io/crates/native-tls
[`rustls`]: https://crates.io/crates/rustls
[`rustls-native-certs`]: https://crates.io/crates/rustls-native-certs
[`serde_json`]: https://crates.io/crates/serde_json
[`simd-json`]: https://crates.io/crates/simd-json
[`tokio-tungstenite`]: https://crates.io/crates/tokio-tungstenite
[`tracing`]: https://crates.io/crates/tracing
[`twilight-http`]: https://twilight-rs.github.io/twilight/twilight_http/index.html
[`webpki-roots`]: https://crates.io/crates/webpki-roots
[`zlib-ng`]: https://github.com/zlib-ng/zlib-ng
[codecov badge]: https://img.shields.io/codecov/c/gh/twilight-rs/twilight?logo=codecov&style=for-the-badge&token=E9ERLJL0L2
[codecov link]: https://app.codecov.io/gh/twilight-rs/twilight/
[discord badge]: https://img.shields.io/discord/745809834183753828?color=%237289DA&label=discord%20server&logo=discord&style=for-the-badge
[discord link]: https://discord.gg/7jj8n7D
[docs:discord:sharding]: https://discord.com/developers/docs/topics/gateway#sharding
[github badge]: https://img.shields.io/badge/github-twilight-6f42c1.svg?style=for-the-badge&logo=github
[github examples link]: https://github.com/twilight-rs/twilight/tree/main/examples
[github link]: https://github.com/twilight-rs/twilight
[license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=for-the-badge&logo=pastebin
[license link]: https://github.com/twilight-rs/twilight/blob/main/LICENSE.md
[rust badge]: https://img.shields.io/badge/rust-1.57+-93450a.svg?style=for-the-badge&logo=rust

<!-- cargo-sync-readme end -->
