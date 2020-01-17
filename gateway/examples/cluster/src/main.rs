use dawn_gateway::cluster::Cluster;

use futures::StreamExt;
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    pretty_env_logger::init_timed();

    let cluster = Cluster::from(env::var("DISCORD_TOKEN")?);

    cluster.up().await?;

    let mut events = cluster.events().await;

    while let Some((id, event)) = events.next().await {
        println!("Shard: {}, Event: {:?}", id, event.event_type());
    }

    Ok(())
}
