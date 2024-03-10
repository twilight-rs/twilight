# twilight-lavalink

[![codecov badge][]][codecov link] [![discord badge][]][discord link] [![github badge][]][github link] [![license badge][]][license link] ![rust badge]

`twilight-lavalink` is a client for [Lavalink] as part of the twilight
ecosystem.

It includes support for managing multiple nodes, a player manager for
conveniently using players to send events and retrieve information for each
guild, and an HTTP module for creating requests using the [`http`] crate and
providing models to deserialize their responses. It will automatically
handle sending voice channel updates to Lavalink by processing events via
the [client's `process` method][`Lavalink::process`], which you must call
with every Voice State Update and Voice Server Update you receive.

***IMPORTANT***

> Lavalink has went to another major API version which changed significantly. To establish this clear change and allow porting current bots easier you can use whichever api you prefer. See the [v3 module](crate::v3) for more details on this interface.

## Features

### `http-support`

The `http-support` feature adds support for the `http` module to return
request types from the [`http`] crate. This is enabled by default.

### TLS

`twilight-lavalink` has features to enable [`tokio-websockets`]' TLS
features. These features are mutually exclusive. `rustls-native-roots` is enabled by
default.

#### `native-tls`

The `native-tls` feature enables [`tokio-websockets`]' `native-tls` feature.

To enable `native-tls`, do something like this in your `Cargo.toml`:

```toml
[dependencies]
twilight-lavalink = { default-features = false, features = ["native-tls"], version = "0.2" }
```

#### `rustls-native-roots`

The `rustls-native-roots` feature enables [`tokio-websockets`]' `rustls-native-roots` feature,
which uses [`rustls`] as the TLS backend and [`rustls-native-certs`] for root certificates.

This is enabled by default.

#### `rustls-webpki-roots`

The `rustls-webpki-roots` feature enables [`tokio-websockets`]' `rustls-webpki-roots` feature,
which uses [`rustls`] as the TLS backend and [`webpki-roots`] for root certificates.

This should be preferred over `rustls-native-roots` in Docker containers based on `scratch`.

## Examples

There is an example of a basic bot located in the [root of the
`twilight` repository][github examples link].

[Lavalink]: https://github.com/freyacodes/Lavalink
[`http`]: https://crates.io/crates/http
[`rustls`]: https://crates.io/crates/rustls
[`rustls-native-certs`]: https://crates.io/crates/rustls-native-certs
[`tokio-websockets`]: https://crates.io/crates/tokio-websockets
[`webpki-roots`]: https://crates.io/crates/webpki-roots
[client]: Lavalink
[codecov badge]: https://img.shields.io/codecov/c/gh/twilight-rs/twilight?logo=codecov&style=for-the-badge&token=E9ERLJL0L2
[codecov link]: https://app.codecov.io/gh/twilight-rs/twilight/
[discord badge]: https://img.shields.io/discord/745809834183753828?color=%237289DA&label=discord%20server&logo=discord&style=for-the-badge
[discord link]: https://discord.gg/7jj8n7D
[github badge]: https://img.shields.io/badge/github-twilight-6f42c1.svg?style=for-the-badge&logo=github
[github examples link]: https://github.com/twilight-rs/twilight/tree/main/examples
[github link]: https://github.com/twilight-rs/twilight
[license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=for-the-badge&logo=pastebin
[license link]: https://github.com/twilight-rs/twilight/blob/main/LICENSE.md
[node]: Node
[process]: Lavalink::process
[rust badge]: https://img.shields.io/badge/rust-1.67+-93450a.svg?style=for-the-badge&logo=rust
