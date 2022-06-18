# Overview

<img
  src="https://raw.githubusercontent.com/twilight-rs/twilight/main/logo.png"
  alt="twilight logo"
/>

[Join us on Discord! :)][server]

**twilight** is a powerful asynchronous, flexible, and scalable ecosystem of
Rust libraries for the Discord API.

[Check out the crates on crates.io][crates.io].

## Who Twilight is For

Twilight is meant for people who are very familiar with Rust and at least
somewhat familiar with Discord bots. It aims to be the library you use when you
want - or, maybe for scaling reasons, need - the freedom to structure things
how you want and do things that other libraries may not strongly cater to.

If you're a beginner with Rust, then that's cool and we hope you like it!
[serenity] is a great library for getting started and offers an opinionated,
batteries-included approach to making bots. You'll probably have a better
experience with it and we recommend you check it out.

## The Guide

In this guide you'll learn about the core crates in the twilight ecosystem,
useful first-party crates for more advanced use cases, and third-party crates
giving you a tailored experience.

## Links

The organization for the project is [on GitHub][github].

The crates are available on [crates.io].

The API docs are also hosted for the [latest version][docs:latest].

There is a community and support server [on Discord][server].

## A Quick Example

Below is a quick example of a program printing "Pong!" when a ping command comes
in from a channel:

```rust,no_run
use futures::stream::StreamExt;
use std::{env, error::Error, sync::Arc};
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{cluster::{Cluster, ShardScheme}, Event, Intents};
use twilight_http::Client as HttpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let token = env::var("DISCORD_TOKEN")?;

    // Start a single shard.
    let scheme = ShardScheme::Range {
        from: 0,
        to: 0,
        total: 1,
    };

    // Specify intents requesting events about things like new and updated
    // messages in a guild and direct messages.
    let intents = Intents::GUILD_MESSAGES | Intents::DIRECT_MESSAGES;

    let (cluster, mut events) = Cluster::builder(token.clone(), intents)
        .shard_scheme(scheme)
        .build()
        .await?;

    let cluster = Arc::new(cluster);

    // Start up the cluster
    let cluster_spawn = cluster.clone();

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    // The http client is seperate from the gateway, so startup a new
    // one, also use Arc such that it can be cloned to other threads.
    let http = Arc::new(HttpClient::new(token));

    // Since we only care about messages, make the cache only process messages.
    let cache = InMemoryCache::builder()
        .resource_types(ResourceType::MESSAGE)
        .build();

    // Startup an event loop to process each event in the event stream as they
    // come in.
    while let Some((shard_id, event)) = events.next().await {
        // Update the cache.
        cache.update(&event);

        // Spawn a new task to handle the event
        tokio::spawn(handle_event(shard_id, event, Arc::clone(&http)));
    }

    Ok(())
}

async fn handle_event(
    shard_id: u64,
    event: Event,
    http: Arc<HttpClient>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match event {
        Event::MessageCreate(msg) if msg.content == "!ping" => {
            http.create_message(msg.channel_id).content("Pong!")?.exec().await?;
        }
        Event::ShardConnected(_) => {
            println!("Connected on shard {}", shard_id);
        }
        _ => {}
    }

    Ok(())
}
```

[crates.io]: https://crates.io/teams/github:twilight-rs:core
[docs:latest]: https://api.twilight.rs
[github]: https://github.com/twilight-rs
[serenity]: https://crates.io/crates/serenity
[server]: https://discord.gg/twilight-rs
