<!-- cargo-sync-readme start -->

# twilight

[![discord badge][]][discord link] [![github badge][]][github link] [![license badge][]][license link] ![rust badge]

![project logo][logo]

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
twilight = { branch = "trunk", git = "https://github.com/twilight-rs/twilight.git" }
```

## Core Crates

These are essential crates that most users will use together for a full
development experience. You may not need all of these - such as
`twilight-cache` - but they are often used together to accomplish most of
what you need.

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

### `twilight-command-parser`

`twilight-command-parser` is a crate for parsing commands out of messages
received over the gateway. It finds messages commanding your bot and parses
the arguments out.

### `twilight-http`

`twilight-http` is an HTTP client supporting all of the Discord REST API. It is
based on `hyper`. It meets Discord's ratelimiting requirements and supports
proxying.

### `twilight-standby`

`twilight-standby` is an event processor that allows for tasks to wait for an
event to come in. This is useful, for example, when you have a reaction menu
and want to wait for a reaction to it to come in.

## Additional Crates

These are crates that are officially supported by Twilight, but aren't
considered core crates due to being vendor-specific or non-essential for most
users.

### `twilight-embed-builder`

[`twilight-embed-builder`] is a utility crate for creating and validating
message embeds, to be used when creating or updating messages.

### `twilight-lavalink`

[`twilight-lavalink`] is a client for [Lavalink] as part of the twilight
ecosystem.

It includes support for managing multiple nodes, a player manager for
conveniently using players to send events and retrieve information for each
guild, and an HTTP module for creating requests using the [`http`] crate and
providing models to deserialize their responses.

### `twilight-mention`

[`twilight-mention`] is a utility crate for creating display formatters for
various model types that format mentions. For example, it can create
formatters for mentioning a channel or emoji, or pinging a role or user.

## Examples

```rust,no_run
use std::{env, error::Error};
use tokio::stream::StreamExt;
use twilight::{
    cache::{
        twilight_cache_inmemory::config::{InMemoryConfigBuilder, EventType},
        InMemoryCache,
    },
    gateway::{cluster::{config::ShardScheme, Cluster, ClusterConfig}, Event},
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
    let cluster = Cluster::new(config).await?;

    let cluster_spawn = cluster.clone();

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

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

    let mut events = cluster.events();
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
            http.create_message(msg.channel_id).content("Pong!")?.await?;
        }
        (id, Event::ShardConnected(_)) => {
            println!("Connected on shard {}", id);
        }
        _ => {}
    }

    Ok(())
}
```

## Note about tracing

When using tracing you won't, by default, see logs from any libraries that use the
`log` crate. You can add that back by using the [`tracing-log`] crate and
initializing it like this:
```rust,ignore
tracing_log::LogTracer::init()?;
```

## License

All first-party crates are licensed under [ISC][LICENSE.md]

[LICENSE.md]: https://github.com/twilight-rs/twilight/blob/trunk/LICENSE.md
[Lavalink]: https://github.com/Frederikam/Lavalink
[`http`]: https://crates.io/crates/http
[discord badge]: https://img.shields.io/discord/745809834183753828?color=%237289DA&label=discord%20server&logo=discord&style=for-the-badge
[discord link]: https://discord.gg/7jj8n7D
[docs:discord:sharding]: https://discord.com/developers/docs/topics/gateway#sharding
[github badge]: https://img.shields.io/badge/github-twilight-6f42c1.svg?style=for-the-badge&logo=github
[github link]: https://github.com/twilight-rs/twilight
[license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=for-the-badge&logo=pastebin
[license link]: https://github.com/twilight-rs/twilight/blob/trunk/LICENSE.md
[logo]: https://raw.githubusercontent.com/twilight-rs/twilight/trunk/logo.png
[rust badge]: https://img.shields.io/badge/rust-stable-93450a.svg?style=for-the-badge&logo=rust
[`twilight-embed-builder`]: https://github.com/twilight-rs/twilight/tree/trunk/utils/embed-builder
[`twilight-lavalink`]: https://github.com/twilight-rs/twilight/tree/trunk/lavalink
[`twilight-mention`]: https://github.com/twilight-rs/twilight/tree/trunk/utils/mention
[`tracing-log`]: https://github.com/tokio-rs/tracing/tree/master/tracing-log

<!-- cargo-sync-readme end -->
