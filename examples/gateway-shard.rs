use futures_util::StreamExt;
use std::{env, error::Error};
use twilight_gateway::{Intents, Shard};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let intents = Intents::GUILD_MESSAGES | Intents::GUILD_VOICE_STATES;
    let (shard, mut events) = Shard::new(env::var("DISCORD_TOKEN")?, intents);

    shard.start().await?;
    println!("Created shard");

    while let Some(event) = events.next().await {
        println!("Event: {:?}", event);
    }

    Ok(())
}
