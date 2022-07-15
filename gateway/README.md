# twilight-gateway

[![codecov badge][]][codecov link] [![discord badge][]][discord link] [![github badge][]][github link] [![license badge][]][license link] ![rust badge]

`twilight-gateway` is an implementation of Discord's sharding gateway sessions.
This is responsible for receiving stateful events in real-time from Discord
and sending *some* stateful information.

The primary type is the `Shard`, a stateful interface to maintain a Websocket
connection to Discord's gateway. Much of its functionality can be configured, and
it's used to receive deserialized gateway event payloads or raw Websocket
messages, useful for load balancing and microservices. Using the `stream`
module, shards can be easily managed in groups.

The Cluster is an interface which manages the health of the shards it
manages and proxies all of their events under one unified stream. This is
useful to use if you have a large bot in over 1000 or 2000 guilds.

## Examples

There are a few usage examples located in the [root of the `twilight`
repository][github examples link].

## Features

* `metrics`: shard analytics for received events and uptime
* `simd-json`: use [`simd-json`] instead of [`serde_json`] for deserializing
  events
* TLS (mutually exclusive)
  * `native`: platform's native TLS implementation via [`native-tls`]
    equivalents
  * `rustls-native-roots` (*default*): [`rustls`] using native root certificates
  * `rustls-webpki-roots`: [`rustls`] using [`webpki-roots`] for root
    certificates, useful for `scratch` containers
* Zlib (mutually exclusive)
  * `zlib-stock` (*default*): [`flate2`]'s stock zlib implementation
  * `zlib-ng`: use [`zlib-ng`] for zlib, may have better performance

[`native-tls`]: https://crates.io/crates/native-tls
[`rustls`]: https://crates.io/crates/rustls
[`rustls-native-certs`]: https://crates.io/crates/rustls-native-certs
[`serde_json`]: https://crates.io/crates/serde_json
[`simd-json`]: https://crates.io/crates/simd-json
[`tokio-tungstenite`]: https://crates.io/crates/tokio-tungstenite
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
[rust badge]: https://img.shields.io/badge/rust-1.60+-93450a.svg?style=for-the-badge&logo=rust
