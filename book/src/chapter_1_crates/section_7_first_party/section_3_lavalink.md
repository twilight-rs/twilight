# Lavalink

`twilight-lavalink` is a client for [Lavalink] for use with [model] events from
the [gateway].

It includes support for managing multiple nodes, a player manager for
conveniently using players to send events and retrieve information for each
guild, and an HTTP module for creating requests using the http crate and
providing models to deserialize their responses.

## Features

### HTTP Support

The `http-support` feature adds types for creating requests and deserializing
response bodies of Lavalink's HTTP routes via the `http` crate.

This is enabled by default.

### TLS

`twilight-lavalink` has features to enable [`async-tungstenite`]'s TLS features.
These features are mutually exclusive.

`rustls` is enabled by default.

#### Native

The `native` feature enables [`async-tungstenite`]'s `tokio-native-tls` feature.
This will use native TLS support, for example OpenSSL on Linux.

#### RusTLS

The `rustls` feature enables [`async-tungstenite`]'s `tokio-rustls` which uses
the [RusTLS] crate as the TLS backend.

This is enabled by default.

## Examples

Create a [client], add a [node], and give events to the client to [process]
events:

```rust,no_run
use futures::StreamExt;
use std::{
    env,
    error::Error,
    net::SocketAddr,
    str::FromStr,
};
use twilight_gateway::{Intents, Shard};
use twilight_http::Client as HttpClient;
use twilight_lavalink::Lavalink;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let token = env::var("DISCORD_TOKEN")?;
    let lavalink_host = SocketAddr::from_str(&env::var("LAVALINK_HOST")?)?;
    let lavalink_auth = env::var("LAVALINK_AUTHORIZATION")?;
    let shard_count = 1_u64;

    let http = HttpClient::new(token.clone());
    let user_id = http.current_user().await?.model().await?.id;

    let lavalink = Lavalink::new(user_id, shard_count);
    lavalink.add(lavalink_host, lavalink_auth).await?;

    let intents = Intents::GUILD_MESSAGES | Intents::GUILD_VOICE_STATES;
    let (shard, mut events) = Shard::new(token, intents);

    shard.start().await?;

    while let Some(event) = events.next().await {
        lavalink.process(&event).await?;
    }

    Ok(())
}
```

## Links

*source*: <https://github.com/twilight-rs/twilight/tree/main/twilight-lavalink>

*docs*: <https://docs.rs/twilight-lavalink>

*crates.io*: <https://crates.io/crates/twilight-lavalink>

[RusTLS]: https://crates.io/crates/rustls
[Lavalink]: https://github.com/freyacodes/Lavalink
[client]: https://twilight-rs.github.io/twilight/twilight_lavalink/client/struct.Lavalink.html
[gateway]: ../section_3_gateway.html
[model]: ../section_1_model.html
[node]: https://twilight-rs.github.io/twilight/twilight_lavalink/node/struct.Node.html
[process]: https://twilight-rs.github.io/twilight/twilight_lavalink/client/struct.Lavalink.html#method.process
[`async-tungstenite`]: https://crates.io/crates/async-tungstenite
