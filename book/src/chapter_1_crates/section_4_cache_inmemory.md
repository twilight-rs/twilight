# Cache

Twilight includes an in-process-memory cache. It's responsible for processing
events and caching things like guilds, channels, users, and voice states.


## Examples

Process new messages that come over a shard into the cache:

```rust,no_run
# #[tokio::main]
# async fn main() -> Result<(), Box<dyn std::error::Error>> {
use futures::StreamExt;
use std::env;
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::{Intents, Shard};

let token = env::var("DISCORD_TOKEN")?;

let (shard, mut events) = Shard::new(token, Intents::GUILD_MESSAGES);
shard.start().await?;

let cache = InMemoryCache::new();

while let Some(event) = events.next().await {
    cache.update(&event);
}
#     Ok(())
# }
```

## Links

*source*: <https://github.com/twilight-rs/twilight/tree/main/cache/in-memory>

*docs*: <https://docs.rs/twilight-cache-inmemory>

*crates.io*: <https://crates.io/crates/twilight-cache-inmemory>
