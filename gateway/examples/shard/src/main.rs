use futures::StreamExt;
use std::{env, error::Error};
use twilight_gateway::Shard;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    pretty_env_logger::init_timed();

    let mut shard = Shard::new(env::var("DISCORD_TOKEN")?);
    let mut events = shard.events().await;

    shard.start().await?;
    println!("Created shard");

    while let Some(event) = events.next().await {
        println!("Event: {:?}", event);
    }

    Ok(())
}
