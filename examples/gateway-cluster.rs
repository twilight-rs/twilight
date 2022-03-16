use futures_util::StreamExt;
use std::{env, error::Error, sync::Arc};
use twilight_gateway::{Cluster, Intents};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let intents =
        Intents::GUILD_MESSAGES | Intents::GUILD_MESSAGE_REACTIONS | Intents::GUILD_MESSAGE_TYPING;
    let (cluster, mut events) = Cluster::new(env::var("DISCORD_TOKEN")?, intents).await?;
    let cluster = Arc::new(cluster);

    let cluster_spawn = Arc::clone(&cluster);

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    while let Some((id, event)) = events.next().await {
        println!("Shard: {}, Event: {:?}", id, event.kind());
    }

    Ok(())
}
