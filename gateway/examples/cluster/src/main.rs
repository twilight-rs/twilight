use twilight_gateway::cluster::{config::ShardScheme, Cluster, ClusterConfig};

use futures::StreamExt;
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    pretty_env_logger::init_timed();

    // This is also the default.
    let scheme = ShardScheme::Range {
        from: 0,
        to: 10,
        total: 11,
    };

    let config = ClusterConfig::builder(env::var("DISCORD_TOKEN")?)
        .shard_scheme(scheme)
        .build();

    let cluster = Cluster::new(config).await?;

    let mut events = cluster.events().await;

    let cluster_spawn = cluster.clone();

    tokio::spawn(async move {
        cluster_spawn.up().await;
    });

    while let Some((id, event)) = events.next().await {
        println!("Shard: {}, Event: {:?}", id, event.kind());
    }

    Ok(())
}
