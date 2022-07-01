use futures_util::StreamExt;
use std::env;
use twilight_gateway::{config::ShardId, Intents, Shard};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let intents = Intents::GUILD_MESSAGES | Intents::GUILD_VOICE_STATES;
    let mut shard = Shard::new(ShardId::ONE, env::var("DISCORD_TOKEN")?, intents).await?;

    println!("Created shard");

    while let Ok(event) = shard.next_event().await {
        println!("Event: {event:?}");
    }

    Ok(())
}
