# 0.15 Migration Guide

This is the migration guide for [version 0.15](./summary.md) of Twilight. It is
focused primarily on the gateway. 

The new gateway API provides finer controls over shards in 33% fewer lines of
code. The ratelimiter has been rewritten. Double linked channels have been
removed, which improves performance. Depending on `twilight_http` is no longer
required. 

## Details

Shards are no longer thin clients being easily passed to event handler tasks.
They must instead be actively polled for events, requiring `&mut self`, in a
loop. State, such as the available ratelimit tokens, can be retrieved for event
handler tasks between polling for the next event but sending gateway commands is
a bit more troublesome.

Fortunately, shards expose a `sender()` method returning a `MessageSender`
struct where commands can be queued up and sent back to the shard for it to
relay it to Discord. As the `Cluster` type has been removed, its functionality
can be recreated by a `HashMap<ShardId, MessageSender>`.

This more flexible API makes it much easier to start and shutdown at runtime and
improves performance by removing a lot of internal complexity.

## The new API

Shards no longer return an additional event stream, which was run by a
background task. They are now driven through `next_message` or `next_event`.

To start multiple shards at once, the `stream` module exposes helper functions.

## Basic "Cluster" Example

Instead of creating a cluster that starts up all shards, use
`create_recommended` to create the recommended number of shards, and stream over
their events: 

```rust,no_run
use std::{env, error::Error};
use twilight_gateway::{Intents, Shard, ShardId};
use twilight_http::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;
    let client = Client::new(token.clone());
    let config = Config::new(token, Intents::GUILD_MESSAGES);

    let mut shards = stream::create_recommended(&client, config, |_, builder| builder.build())
        .await?
        .collect<Vec<_>>();

    let mut stream = ShardEventStream::new(shards.iter_mut());

    while let Some((shard, event)) = stream.next().await {
        let event = match event {
            Ok(event) => event,
            Err(source) => {
                tracing::warn!(?source, "error receiving event");

                if source.is_fatal() {
                    break;
                }

                continue;
            }
        };

        tracing::debug!(?event, shard = ?shard.id(), "received event");
    }

    Ok(())
}
```

