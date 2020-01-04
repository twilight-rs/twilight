use dawn_gateway::Shard;
use futures::StreamExt;
use log::Level;
use metrics_runtime::{exporters::LogExporter, observers::JsonBuilder, Receiver};
use std::{env, error::Error, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let receiver = Receiver::builder()
        .build()
        .expect("failed to create receiver");
    let mut exporter = LogExporter::new(
        receiver.controller(),
        JsonBuilder::new().set_pretty_json(true),
        Level::Info,
        Duration::from_secs(60),
    );
    // Install receiver.
    receiver.install();

    pretty_env_logger::init_timed();

    let shard = Shard::new(env::var("DISCORD_TOKEN")?).await?;
    println!("Created shard");

    let mut events = shard.events().await;

    // Start exporter in a seperate task
    tokio::task::spawn_blocking(move || exporter.run());

    while let Some(event) = events.next().await {
        println!("Event: {:?}", event.event_type());
    }

    Ok(())
}
