# twilight-gateway

[![codecov badge][]][codecov link] [![discord badge][]][discord link] [![github badge][]][github link] [![license badge][]][license link] ![rust badge]

`twilight-gateway` is an implementation of Discord's sharding gateway sessions.
This is responsible for receiving stateful events in real-time from Discord
and sending *some* stateful information.

The primary type is the `Shard`, a stateful interface to maintain a Websocket
connection to Discord's gateway. Much of its functionality can be configured, and
it's used to receive deserialized gateway event payloads or raw Websocket
messages, useful for load balancing and microservices.

Using the `stream` module, shards can be easily managed in groups.

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
* `twilight-http` (*default*): enable the `stream::start_recommended` function
* Zlib (mutually exclusive)
  * `zlib-stock` (*default*): [`flate2`]'s stock zlib implementation
  * `zlib-ng`: use [`zlib-ng`] for zlib, may have better performance

## Examples

Start a shard and loop over guild and voice state events:

```rust,no_run
use std::env;
use twilight_gateway::{Intents, Shard, ShardId};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;
    let intents = Intents::GUILDS | Intents::GUILD_VOICE_STATES;

    // Initialize the first and only shard in use by a bot.
    let mut shard = Shard::new(ShardId::ONE, token, intents);

    tracing::info!("started shard");

    loop {
        let event = match shard.next_event().await {
            Ok(event) => event,
            Err(source) => {
                tracing::warn!(
                    source = &source as &dyn std::error::Error,
                    "error receiving event"
                );

                // If the error is fatal, as may be the case for invalid
                // authentication or intents, then break out of the loop to
                // avoid constantly attempting to reconnect.
                if source.is_fatal() {
                    break;
                }

                continue;
            },
        };

        tracing::debug!(?event, "received event");
    }

    Ok(())
}
```

Create the recommended number of shards and stream over their events:

```rust,no_run
use futures::StreamExt;
use std::{collections::HashMap, env, future};
use twilight_gateway::{stream::{self, ShardEventStream}, Config, Intents};
use twilight_http::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = env::var("DISCORD_TOKEN")?;
    let client = Client::new(token.clone());

    // Callback to create a config for each shard, useful for when not all shards
    // have the same configuration, such as for per-shard presences.
    let config_callback = |_| Config::new(token.clone(), Intents::GUILDS);

    let mut shards = stream::start_recommended(&client, config_callback)
        .await?
        .collect::<Vec<_>>();

    let mut stream = ShardEventStream::new(shards.iter_mut());

    loop {
        let (shard, event) = match stream.next().await {
            Some((shard, Ok(event))) => (shard, event),
            Some((shard, Err(source))) => {
                tracing::warn!(
                    source = &source as &dyn std::error::Error,
                    "error receiving event"
                );

                if source.is_fatal() {
                    break;
                }

                continue;
            },
            None => break,
        };

        println!("received event on shard {}: {event:?}", shard.id());
    }

    Ok(())
}
```

There are a few additional examples located in the
[repository][github examples link].

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
[rust badge]: https://img.shields.io/badge/rust-1.64+-93450a.svg?style=for-the-badge&logo=rust
