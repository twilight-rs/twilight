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

## Features

### `http-support`

The `http-support` feature adds support for the `http` module to return
request types from the [`http`] crate. This is enabled by default.

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

### TLS

`twilight-lavalink` has features to enable [`tokio-websockets`]' TLS
features. These features are mutually exclusive. `rustls-platform-verifier` is
enabled by default.

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
This requires configuring a crypto provider.

#### `rustls-platform-verifier`

The `rustls-platform-verifier` feature enables [`tokio-websockets`]' `rustls-platform-verifier`
feature, which uses [`rustls`] as the TLS backend and [`rustls-platform-verifier`] for
certificate validation. This requires configuring a crypto provider.

This is enabled by default.

#### `rustls-webpki-roots`

The `rustls-webpki-roots` feature enables [`tokio-websockets`]' `rustls-webpki-roots` feature,
which uses [`rustls`] as the TLS backend and [`webpki-roots`] for root certificates.
This requires configuring a crypto provider.

This should be preferred over `rustls-native-roots` in Docker containers based on `scratch`.

## Examples

Create a [client], add a [node], and give events to the client to [process]
events:

```rust,no_run
use std::{
    env,
    future::Future,
    net::SocketAddr,
    str::FromStr,
};
use twilight_gateway::{Event, EventTypeFlags, Intents, Shard, ShardId, StreamExt as _};
use twilight_http::Client as HttpClient;
use twilight_lavalink::{http::LoadedTracks, model::Play, Lavalink};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;
    let lavalink_host = SocketAddr::from_str(&env::var("LAVALINK_HOST")?)?;
    let lavalink_auth = env::var("LAVALINK_AUTHORIZATION")?;
    let shard_count = 1u32;

    let http = HttpClient::new(token.clone());
    let user_id = http.current_user().await?.model().await?.id;

    let lavalink = Lavalink::new(user_id, shard_count);
    lavalink.add(lavalink_host, lavalink_auth).await?;

    let intents = Intents::GUILD_MESSAGES | Intents::GUILD_VOICE_STATES;
    let mut shard = Shard::new(ShardId::ONE, token, intents);

    while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
        let Ok(event) = item else {
            tracing::warn!(source = ?item.unwrap_err(), "error receiving event");

            continue;
        };

        lavalink.process(&event).await?;
    }

    Ok(())
}
```

There is also an example of a basic bot located in the [root of the
`twilight` repository][github examples link].

[Lavalink]: https://github.com/freyacodes/Lavalink
[`CryptoProvider::install_default`]: https://docs.rs/rustls/latest/rustls/crypto/struct.CryptoProvider.html#method.install_default
[`aws-lc-rs`]: https://crates.io/crates/aws-lc-rs
[`http`]: https://crates.io/crates/http
[`ring`]: https://crates.io/crates/ring
[`rustls`]: https://crates.io/crates/rustls
[`rustls-native-certs`]: https://crates.io/crates/rustls-native-certs
[`rustls-platform-verifier`]: https://crates.io/crates/rustls-platform-verifier
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
[rust badge]: https://img.shields.io/badge/rust-1.79+-93450a.svg?style=for-the-badge&logo=rust
