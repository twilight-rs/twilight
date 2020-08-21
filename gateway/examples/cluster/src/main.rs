use twilight_gateway::cluster::{Cluster, ShardScheme};

use futures::StreamExt;
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    // This is also the default.
    let scheme = ShardScheme::Auto;

    let cluster = Cluster::builder(env::var("DISCORD_TOKEN")?)
        .shard_scheme(scheme)
        .build()
        .await?;

    let mut events = cluster.events();

    let cluster_spawn = cluster.clone();

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    while let Some((id, event)) = events.next().await {
        println!("Shard: {}, Event: {:?}", id, event.kind());
    }

    Ok(())
}
