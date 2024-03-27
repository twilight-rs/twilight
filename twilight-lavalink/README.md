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

A breakdown of how this functions:
- The client is [`Lavalink`](crate::client::Lavalink) that forwards the required events from Discord.
    - We read the [Voice State and Voice Server Updates](https://discord.com/developers/docs/topics/gateway-events#voice) from discord to format the data to send to a Lavalink VoiceUpdate Event.
    - There is a lower level [node](crate::node) that processes this for you. It isn't recommended to use this but rather the lavalink struct with the players. If you don't find functionality please open up and issue to expose what you need.
- You send the client an [outgoing event](crate::model::outgoing). These include play, pause, seek, etc. You send these through the [player](crate::player) that is attached to Lavalink.
- If you want to search or load you need to create a http client currently and then you can use [these helpers functions](crate::http#functions) to generate the http uri and body to send over your http client. you will then get response you can deserialize as json into the structs in the [http module](crate::http).

***NOTE: We currently only support `v4` of Lavlink. Support for `v3` is dropped. There was big changes in the api meaning the outgoing are now using a http client instead of websockets. The json request and responses all changed naming and fields changed.***

Currently some [Filters](crate::model::outgoing::Filters) are not yet supported. There are some unsupported end points that were added yet such as [Lavalink Info](https://lavalink.dev/api/rest.html#get-lavalink-version) or [Session Api](https://lavalink.dev/api/rest.html#session-api) that weren't previously available. If you would like native support for something please reach out and open an issue for that feature. The porting only ported the functionality of the previous `v3` forward.

## Features

### `http-support`

The `http-support` feature adds support for the `http` module to return
request types from the [`http`] crate. This is enabled by default.

### `lavalink-protocol-http2`

The `lavalink-protocol-http2` switches the underlying protocol to communicate with the lavalink server.
If enabled, http2 will be used. By default, http1 is used. You will need to enable http2 support
in your lavalink server configuration if you want to use this feature because by default it is disabled.

***NOTE: This is not to be confused with the `http-support` support flag or crate. This is separate and doesn't depend on the use of that feature.***

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
