use futures::StreamExt;
use std::{env, error::Error};
use twilight_gateway::{Intents, Shard};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let intents = Intents::GUILD_MESSAGES | Intents::GUILD_VOICE_STATES;
    let mut shard = Shard::new(env::var("DISCORD_TOKEN")?, intents);
    let mut events = shard.events();

    shard.start().await?;
    println!("Created shard");

    while let Some(event) = events.next().await {
        println!("Event: {:?}", event);
    }

    Ok(())
}
