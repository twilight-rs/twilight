use twilight_gateway::cluster::{config::ShardScheme, Cluster, Config};

use futures::StreamExt;
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    pretty_env_logger::init_timed();

    // This is also the default.
    let scheme = ShardScheme::Auto;

    let config = Config::builder(env::var("DISCORD_TOKEN")?)
        .shard_scheme(scheme)
        .build();

    let cluster = Cluster::new(config);

    cluster.up().await?;

    let mut events = cluster.events().await;

    while let Some((id, event)) = events.next().await {
        println!("Shard: {}, Event: {:?}", id, event.event_type());
    }

    Ok(())
}
