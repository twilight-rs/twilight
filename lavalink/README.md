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

### TLS

`twilight-lavalink` has features to enable [`tokio-tungstenite`]'s TLS
features. These features are mutually exclusive. `rustls-native-roots` is enabled by
default.

#### `native`

The `native` feature enables [`tokio-tungstenite`]'s `native-tls` feature.

To enable `native`, do something like this in your `Cargo.toml`:

```toml
[dependencies]
twilight-lavalink = { default-features = false, features = ["native"], version = "0.2" }
```

#### `rustls-native-roots`

The `rustls-native-roots` feature enables [`tokio-tungstenite`]'s `rustls-tls-native-roots` feature,
which uses [`rustls`] as the TLS backend and [`rustls-native-certs`] for root certificates.

This is enabled by default.

#### `rustls-webpki-roots`

The `rustls-webpki-roots` feature enables [`tokio-tungstenite`]'s `rustls-tls-webpki-roots` feature,
which uses [`rustls`] as the TLS backend and [`webpki-roots`] for root certificates.

This should be preferred over `rustls-native-roots` in Docker containers based on `scratch`.

## Examples

Create a [client], add a [node], and give events to the client to [process]
events:

```rust,no_run
use futures_util::stream::StreamExt;
use std::{
    env,
    future::Future,
    net::SocketAddr,
    str::FromStr,
};
use twilight_gateway::{config::ShardId, Event, Intents, Shard};
use twilight_http::Client as HttpClient;
use twilight_lavalink::{http::LoadedTracks, model::Play, Lavalink};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = env::var("DISCORD_TOKEN")?;
    let lavalink_host = SocketAddr::from_str(&env::var("LAVALINK_HOST")?)?;
    let lavalink_auth = env::var("LAVALINK_AUTHORIZATION")?;
    let shard_count = 1u64;

    let http = HttpClient::new(token.clone());
    let user_id = http.current_user().exec().await?.model().await?.id;

    let lavalink = Lavalink::new(user_id, shard_count);
    lavalink.add(lavalink_host, lavalink_auth).await?;

    let intents = Intents::GUILD_MESSAGES | Intents::GUILD_VOICE_STATES;
    let mut shard = Shard::new(ShardId::ONE, token, intents).await?;

    loop {
        let event = match shard.next_event().await {
            Ok(event) => event,
            Err(source) => {
                println!("error receiving event: {:?}", source);

                continue;
            }
        };

        lavalink.process(&event).await?;
    }

    Ok(())
}
```

There is also an example of a basic bot located in the [root of the
`twilight` repository][github examples link].

[Lavalink]: https://github.com/freyacodes/Lavalink
[`http`]: https://crates.io/crates/http
[`rustls`]: https://crates.io/crates/rustls
[`rustls-native-certs`]: https://crates.io/crates/rustls-native-certs
[`tokio-tungstenite`]: https://crates.io/crates/tokio-tungstenite
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
[rust badge]: https://img.shields.io/badge/rust-1.60+-93450a.svg?style=for-the-badge&logo=rust
