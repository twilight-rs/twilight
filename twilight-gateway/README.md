# twilight-gateway

[![codecov badge][]][codecov link] [![discord badge][]][discord link] [![github badge][]][github link] [![license badge][]][license link] ![rust badge]

`twilight-gateway` is an implementation of Discord's sharding gateway sessions.
This is responsible for receiving stateful events in real-time from Discord and
sending *some* stateful information.

The primary type is the `Shard`, a stateful interface to maintain a Websocket
connection to Discord's gateway. Much of its functionality can be configured,
and it's used to receive gateway events or raw Websocket messages, useful for
load balancing and microservices.

Multiple shards may easily be created at once, with a per shard config created
from a `Fn(ShardId, ConfigBuilder) -> Config` closure, with the help of the
`create_` set of functions. These functions will reuse shards' TLS context and
[session queue][queue], something otherwise achieved by cloning an existing
[`Config`].

## Features

* `simd-json`: use [`simd-json`] instead of [`serde_json`] for deserializing
  events
* TLS (mutually exclusive)
  * `native-tls`: platform's native TLS implementation via [`native-tls`]
  * `rustls-native-roots`: [`rustls`] using native root certificates
  * `rustls-platform-verifier` (*default*): [`rustls`] using operating system's
    certificate facilities via [`rustls-platform-verifier`]
  * `rustls-webpki-roots`: [`rustls`] using [`webpki-roots`] for root
    certificates, useful for `scratch` containers
* `twilight-http` (*default*): enable the `stream::create_recommended` function
* Transport compression (mutually exclusive)
  * `zlib`: Zlib transport compression using [`zlib-rs`][^1]
  * `zstd` (*default*): Zstandard transport compression using [`zstd-sys`]

## Examples

Create a shard and loop over guild events:

```rust,no_run
use std::env;
use twilight_gateway::{EventTypeFlags, Intents, Shard, ShardId, StreamExt as _};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    // Select rustls backend
    rustls::crypto::ring::default_provider().install_default().unwrap();

    let token = env::var("TOKEN")?;

    // Initialize the first and only shard in use by a bot.
    let mut shard = Shard::new(ShardId::ONE, token, Intents::GUILDS);

    tracing::info!("started shard");

    while let Some(event) = shard.next_event(EventTypeFlags::all()).await {
        match event {
            Ok(event) => tracing::info!(?event, "received event"),
            Err(source) => tracing::warn!(?source, "failed to receive event"),
        }
    }

    Ok(())
}
```

There are a few additional examples located in the
[repository][github examples link]. Check out our [template] to get started
quickly.

[^1]: Except for the s390x arch, where [`zlib-ng-sys`] is used instead.

[`native-tls`]: https://crates.io/crates/native-tls
[`rustls`]: https://crates.io/crates/rustls
[`rustls-platform-verifier`]: https://crates.io/crates/rustls-platform-verifier
[`serde_json`]: https://crates.io/crates/serde_json
[`simd-json`]: https://crates.io/crates/simd-json
[`webpki-roots`]: https://crates.io/crates/webpki-roots
[`zlib-ng-sys`]: https://crates.io/crates/libz-ng-sys
[`zlib-rs`]: https://crates.io/crates/zlib-rs
[`zstd-sys`]: https://crates.io/crates/zstd-sys
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
[template]: https://github.com/twilight-rs/template
