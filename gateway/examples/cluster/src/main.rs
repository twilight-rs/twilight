use dawn_gateway::cluster::{Cluster, Config};

use futures::StreamExt;
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    pretty_env_logger::init_timed();

    let config = Config::builder(env::var("DISCORD_TOKEN")?).build();

    let http = config.http_client().clone();

    let cu = http.current_user().await?;

    println!("CU: {:#?}", cu);

    let cluster = Cluster::new(config);
    println!("Created shard");

    cluster.up().await?;

    let mut events = cluster.events().await;

    while let Some(event) = events.next().await {
        println!("Event: {:?}", event);
    }

    Ok(())
}
