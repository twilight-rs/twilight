# V3

This is the implementation of the v3 lavalink player api. There were substantial changes to the api.
You can include and use this project without upgrading by simply prefixing your includes with `v3::`.

## Examples

Create a [client], add a [node], and give events to the client to [process]
events:

```rust,no_run
use std::{
    env,
    future::Future,
    net::SocketAddr,
    str::FromStr,
};
use twilight_gateway::{Event, EventTypeFlags, Intents, Shard, ShardId, StreamExt as _};
use twilight_http::Client as HttpClient;
use twilight_lavalink::{http::LoadedTracks, model::Play, Lavalink};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;
    let lavalink_host = SocketAddr::from_str(&env::var("LAVALINK_HOST")?)?;
    let lavalink_auth = env::var("LAVALINK_AUTHORIZATION")?;
    let shard_count = 1u32;

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
