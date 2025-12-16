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

`twilight-lavalink` has features to enable [`tokio-websockets`]' TLS features.
These features are mutually exclusive. `rustls-native-roots` is enabled by
default.

#### Native-TLS

The `native-tls` feature enables [`tokio-websockets`]' `native-tls` feature.

#### RusTLS

RusTLS allows specifying from where certificate roots are retrieved from.

##### Native roots

The `rustls-native-roots` feature enables [`tokio-websockets`]'
`rustls-native-roots` feature.

This is enabled by default.

##### Web PKI roots

The `rustls-webpki-roots` feature enables [`tokio-websockets`]'
`rustls-webpki-roots` feature.

## Examples

Create a [client], add a [node], and give events to the client to [process]
events:

```rust,no_run
use std::{
    env,
    error::Error,
    net::SocketAddr,
    str::FromStr,
};
use twilight_gateway::{EventTypeFlags, Intents, Shard, ShardId, StreamExt as _};
use twilight_http::Client as HttpClient;
use twilight_lavalink::Lavalink;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    // Select rustls backend
    rustls::crypto::ring::default_provider().install_default().unwrap();

    let token = env::var("DISCORD_TOKEN")?;
    let lavalink_host = SocketAddr::from_str(&env::var("LAVALINK_HOST")?)?;
    let lavalink_auth = env::var("LAVALINK_AUTHORIZATION")?;
    let shard_count = 1_u32;

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

## Links

*source*: <https://github.com/twilight-rs/twilight/tree/main/twilight-lavalink>

*docs*: <https://docs.rs/twilight-lavalink>

*crates.io*: <https://crates.io/crates/twilight-lavalink>

[Lavalink]: https://github.com/freyacodes/Lavalink
[client]: https://twilight-rs.github.io/twilight/twilight_lavalink/client/struct.Lavalink.html
[gateway]: ../section_3_gateway.html
[model]: ../section_1_model.html
[node]: https://twilight-rs.github.io/twilight/twilight_lavalink/node/struct.Node.html
[process]: https://twilight-rs.github.io/twilight/twilight_lavalink/client/struct.Lavalink.html#method.process
[`tokio-websockets`]: https://crates.io/crates/tokio-websockets
