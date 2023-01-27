use futures_util::StreamExt;
use std::env;
use twilight_gateway::{Intents, Shard};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let intents = Intents::GUILDS | Intents::GUILD_MODERATION;
    let (shard, mut events) = Shard::new(env::var("DISCORD_TOKEN")?, intents);

    shard.start().await?;
    println!("Created shard");

    while let Some(event) = events.next().await {
        println!("Event: {event:?}");
    }

    Ok(())
}
