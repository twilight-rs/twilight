# twilight

[![codecov badge][]][codecov link] [![discord badge][]][discord link] [![github badge][]][github link] [![license badge][]][license link] ![rust badge]

![project logo][logo]

`twilight` is a powerful, flexible, and scalable ecosystem of Rust libraries
for the Discord API.

The ecosystem of first-class crates includes [`twilight-cache-inmemory`],
[`twilight-gateway`], [`twilight-http`], [`twilight-model`], and more. These
are explained in detail below.

The main `twilight` crate is purely an advertisement crate: it has *no*
functionality. Please use the individual crates listed below instead!

## Installation

Twilight supports a MSRV of Rust 1.67.

We recommend that most users start out with these crates:

- [`twilight-cache-inmemory`][crates:cache-inmemory]
- [`twilight-gateway`][crates:gateway]
- [`twilight-http`][crates:http]
- [`twilight-model`][crates:model]

If you need any other functionality that Twilight provides, you can just add
that dependency in.

## Core Crates

These are essential crates that most users will use together for a full
development experience. You may not need all of these - such as
[`twilight-cache-inmemory`] - but they are often used together to accomplish
most of what you need.

### [`twilight-model`]

Models defining structures, enums, and bitflags for the entirety of the
Discord API. It is split into a number of sub-modules, such as `gateway` for
containing the WebSocket gateway types, `guild` for containing types owned
by guilds (servers), `voice` containing the types used by the Voice
WebSocket API, and more.

These are all in a single crate so that you can use `gateway` models without
depending on [`twilight-gateway`]. One use case is if you write your own
WebSocket gateway implementation.

### [`twilight-cache-inmemory`]

In-process-memory based cache over objects received from the gateway. It's
responsible for holding and managing information about things like guilds,
channels, role information, voice states, and any other events that come
from Discord.

### [`twilight-gateway`]

Implementation of Discord's sharding gateway sessions. This is responsible
for receiving stateful events in real-time from Discord and sending *some*
stateful information.

### [`twilight-http`]

HTTP client supporting all of the Discord REST API. It is based on `hyper`.
It meets Discord's ratelimiting requirements and supports proxying.

### [`twilight-standby`]

Event processor that allows for tasks to wait for an event to come in. This
is useful, for example, when you have a reaction menu and want to wait for a
specific reaction on it to come in.

## Additional Crates

These are crates that are officially supported by Twilight, but aren't
considered core crates due to being vendor-specific or non-essential for
most users.

### [`twilight-lavalink`]

Client for [Lavalink] as part of the twilight ecosystem.

It includes support for managing multiple nodes, a player manager for
conveniently using players to send events and retrieve information for each
guild, and an HTTP module for creating requests using the [`http`] crate and
providing models to deserialize their responses.

### [`twilight-mention`]

Create display formatters for various model types that format mentions. For
example, it can create formatters for mentioning a channel or emoji, or
pinging a role or user.

### [`twilight-util`]

Utility crate that adds utilities to the twilight ecosystem that do not fit
in any other crate. Currently, it contains:

- A trait to make extracting data from Discord identifiers (Snowflakes)
  easier;
- A calculator to calculate the permissions of a member in a guild or
  channel.

### [`twilight-gateway-queue`]

A trait and some implementations that are used by the gateway to ratelimit
identify calls. Developers should prefer to use the re-exports of these
crates through the gateway.

## Examples

The following example is a template for bootstrapping a new bot using
Twilight's HTTP and gateway clients with its in-memory cache. In order to
run this, replace the contents of a new project's `main.rs` file with the
following. Be sure to set the `DISCORD_TOKEN` environment variable to your
bot's token. You must also depend on `tokio`, `twilight-cache-inmemory`,
`twilight-gateway`, `twilight-http`, and `twilight-model` in your `Cargo.toml`.

```rust,no_run
use std::{env, error::Error, sync::Arc};
use twilight_cache_inmemory::{DefaultInMemoryCache, ResourceType};
use twilight_gateway::{Event, EventTypeFlags, Intents, Shard, ShardId, StreamExt as _};
use twilight_http::Client as HttpClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;

    // Use intents to only receive guild message events.
    let mut shard = Shard::new(
        ShardId::ONE,
        token.clone(),
        Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT,
    );

    // HTTP is separate from the gateway, so create a new client.
    let http = Arc::new(HttpClient::new(token));

    // Since we only care about new messages, make the cache only
    // cache new messages.
    let cache = DefaultInMemoryCache::builder()
        .resource_types(ResourceType::MESSAGE)
        .build();

    // Process each event as they come in.
    while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
        let Ok(event) = item else {
            tracing::warn!(source = ?item.unwrap_err(), "error receiving event");

            continue;
        };

        // Update the cache with the event.
        cache.update(&event);

        tokio::spawn(handle_event(event, Arc::clone(&http)));
    }

    Ok(())
}

async fn handle_event(
    event: Event,
    http: Arc<HttpClient>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::MessageCreate(msg) if msg.content == "!ping" => {
            http.create_message(msg.channel_id)
                .content("Pong!")
                .await?;
        }
        // Other events here...
        _ => {}
    }

    Ok(())
}
```

## License

All first-party crates are licensed under [ISC][LICENSE.md]

[LICENSE.md]: https://github.com/twilight-rs/twilight/blob/main/LICENSE.md
[Lavalink]: https://github.com/freyacodes/Lavalink
[`http`]: https://crates.io/crates/http
[crates:cache-inmemory]: https://crates.io/crates/twilight-cache-inmemory
[crates:gateway]: https://crates.io/crates/twilight-gateway
[crates:http]: https://crates.io/crates/twilight-http
[crates:model]: https://crates.io/crates/twilight-model
[codecov badge]: https://img.shields.io/codecov/c/gh/twilight-rs/twilight?logo=codecov&style=for-the-badge&token=E9ERLJL0L2
[codecov link]: https://app.codecov.io/gh/twilight-rs/twilight/
[discord badge]: https://img.shields.io/discord/745809834183753828?color=%237289DA&label=discord%20server&logo=discord&style=for-the-badge
[discord link]: https://discord.gg/7jj8n7D
[github badge]: https://img.shields.io/badge/github-twilight-6f42c1.svg?style=for-the-badge&logo=github
[github link]: https://github.com/twilight-rs/twilight
[license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=for-the-badge&logo=pastebin
[license link]: https://github.com/twilight-rs/twilight/blob/main/LICENSE.md
[logo]: https://raw.githubusercontent.com/twilight-rs/twilight/main/logo.png
[rust badge]: https://img.shields.io/badge/rust-1.67+-93450a.svg?style=for-the-badge&logo=rust
[`twilight-cache-inmemory`]: https://twilight.rs/chapter_1_crates/section_4_cache_inmemory.html
[`twilight-gateway-queue`]: https://twilight.rs/chapter_1_crates/section_7_first_party/section_5_gateway_queue.html
[`twilight-gateway`]: https://twilight.rs/chapter_1_crates/section_3_gateway.html
[`twilight-http`]: https://twilight.rs/chapter_1_crates/section_2_http.html
[`twilight-lavalink`]: https://twilight.rs/chapter_1_crates/section_7_first_party/section_3_lavalink.html
[`twilight-mention`]: https://twilight.rs/chapter_1_crates/section_7_first_party/section_2_mention.html
[`twilight-model`]: https://twilight.rs/chapter_1_crates/section_1_model.html
[`twilight-standby`]: https://twilight.rs/chapter_1_crates/section_6_standby.html
[`twilight-util`]: https://twilight.rs/chapter_1_crates/section_7_first_party/section_4_util.html
