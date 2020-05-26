[![license badge][]][license link] [![rust badge]][rust link]

![project logo][logo]

# twilight

`twilight` is an asynchronous, simple, and extensible set of libraries which can
be used separately or in combination for the Discord API.

The ecosystem of first-class crates includes `twilight-cache`,
`twilight-command-parser`, `twilight-gateway`, `twilight-http`, `twilight-model`,
and more. These are explained in detail below.

The main `twilight` crate is a "skeleton crate": it includes all of the
non-vendor-specific crates in the `twilight` ecosystem.

## Installation

Most of Twilight requires at least 1.40+ (rust stable).

Add this to your `Cargo.toml`'s `[dependencies]` section:

```toml
twilight = {version = "0.0.1-alpha.0", git = "https://github.com/twilight-rs/twilight.git" }
```

## Crates

These are crates that can work together for a full application experience.
You may not need all of these - such as `twilight-cache` - but they can be
mixed together to accomplish just what you need.

### `twilight-model`

`twilight-model` is a set of models defining structures, enums, and bitflags
for the entirety of the Discord API. It is split into a number of
sub-modules, such as `gateway` for containing the WebSocket gateway types,
`guild` for containing types owned by guilds (servers), `voice` containing
the types used by the Voice WebSocket API, and more.

These are all in a single crate so that you can use `gateway` models without
depending on `twilight-gateway`. One use case is if you write your own WebSocket
gateway implementation.

### `twilight-cache`

`twilight-cache` is based on a single trait which can be implemented to use
custom third-party backends with a single ubiquitous interface. The Cache is
responsible for holding information about things like guilds, channels, role
information, voice states, and any other data that comes from Discord.

Included by default is an `InMemoryCache` backend, which caches within the
process's memory.

### `twilight-gateway`

`twilight-gateway` is an implementation of Discord's sharding gateway sessions.
This is responsible for receiving stateful events in real-time from Discord
and sending *some* stateful information.

It includes two primary types: the Shard and Cluster.

The Shard handles a single WebSocket connection and can manage up to 2500
guilds. If you manage a small bot in under about 2000 guilds, then this is
what you use. See the [Discord docs][docs:discord:sharding] for more
information on sharding.

The Cluster is an interface which manages the health of the shards it
manages and proxies all of their events under one unified stream. This is
useful to use if you have a large bot in over 1000 or 2000 guilds.

### `twilight-command-parser`

`twilight-command-parser` is a crate for parsing commands out of messages
received over the gateway. It finds messages commanding your bot and parses
the arguments out.

### `twilight-http`

`twilight-http` is an HTTP client supporting all of the Discord REST API. It is
based on `hyper`. It meets Discord's ratelimiting requirements and supports
proxying.


## Examples
```rust
use std::{env, error::Error};
use tokio::stream::StreamExt;

use twilight::{
    cache::{
        twilight_cache_inmemory::config::{InMemoryConfigBuilder, EventType},
        InMemoryCache,
    },
    gateway::cluster::{config::ShardScheme, Cluster, ClusterConfig},
    gateway::shard::Event,
    http::Client as HttpClient,
    model::gateway::GatewayIntents,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let token = env::var("DISCORD_TOKEN")?;

    // This is also the default.
    let scheme = ShardScheme::Auto;

    let config = ClusterConfig::builder(&token)
        .shard_scheme(scheme)
        // Use intents to only listen to GUILD_MESSAGES events
        .intents(Some(
            GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES,
        ))
        .build();

    // Start up the cluster
    let cluster = Cluster::new(config);
    cluster.up().await?;

    // The http client is seperate from the gateway,
    // so startup a new one
    let http = HttpClient::new(&token);

    // Since we only care about messages, make the cache only
    // cache message related events
    let cache_config = InMemoryConfigBuilder::new()
        .event_types(
            EventType::MESSAGE_CREATE
                | EventType::MESSAGE_DELETE
                | EventType::MESSAGE_DELETE_BULK
                | EventType::MESSAGE_UPDATE,
        )
        .build();
    let cache = InMemoryCache::from(cache_config);


    let mut events = cluster.events().await;
    // Startup an event loop for each event in the event stream
    while let Some(event) = events.next().await {
        // Update the cache
        cache.update(&event.1).await.expect("Cache failed, OhNoe");

        // Spawn a new task to handle the event
        tokio::spawn(handle_event(event, http.clone()));
    }

    Ok(())
}

async fn handle_event(
    event: (u64, Event),
    http: HttpClient,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        (_, Event::MessageCreate(msg)) if msg.content == "!ping" => {
            http.create_message(msg.channel_id).content("Pong!").await?;
        }
        (id, Event::ShardConnected(_)) => {
            println!("Connected on shard {}", id);
        }
        _ => {}
    }

    Ok(())
}
```


## License

All first-party crates are licensed under [ISC][LICENSE.md]

[LICENSE.md]: https://github.com/twilight-rs/twilight/blob/master/LICENSE.md
[docs:discord:sharding]: https://discord.com/developers/docs/topics/gateway#sharding
[license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
[license link]: https://opensource.org/licenses/ISC
[logo]: https://raw.githubusercontent.com/twilight-rs/twilight/master/logo.png
[rust badge]: https://img.shields.io/badge/rust-1.40+%20(stable)-93450a.svg?style=flat-square
[rust link]: https://github.com/rust-lang/rust/milestone/66
