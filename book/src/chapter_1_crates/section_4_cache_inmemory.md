# Cache

Twilight includes an in-process-memory cache. It's responsible for processing
events and caching things like guilds, channels, users, and voice states.


## Examples

Process new messages that come over a shard into the cache:

```rust,no_run
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
use std::env;
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::{Intents, Shard, ShardId};

let token = env::var("DISCORD_TOKEN")?;

let mut shard = Shard::new(ShardId::ONE, token, Intents::GUILD_MESSAGES);

let cache = InMemoryCache::new();

loop {
    let event = match shard.next_event().await {
        Ok(event) => event,
        Err(source) => {
            tracing::warn!(?source, "error receiving event");

            if source.is_fatal() {
                break;
            }

            continue;
        }
    };

    cache.update(&event);
}
#     Ok(())
# }
```

## Links

*source*: <https://github.com/twilight-rs/twilight/tree/main/twilight-cache-inmemory>

*docs*: <https://docs.rs/twilight-cache-inmemory>

*crates.io*: <https://crates.io/crates/twilight-cache-inmemory>
