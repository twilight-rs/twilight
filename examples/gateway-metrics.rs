use futures_util::StreamExt;
use metrics_runtime::{exporters::LogExporter, observers::JsonBuilder, Receiver};
use std::{env, error::Error, time::Duration};
use twilight_gateway::{Cluster, Intents};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let receiver = Receiver::builder()
        .build()
        .expect("failed to create receiver");
    let mut exporter = LogExporter::new(
        receiver.controller(),
        JsonBuilder::new().set_pretty_json(true),
        log::Level::Info,
        Duration::from_secs(30),
    );
    // Install receiver.
    receiver.install();

    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let intents = Intents::GUILD_BANS | Intents::GUILD_EMOJIS | Intents::GUILD_MESSAGES;
    let (cluster, mut events) = Cluster::new(env::var("DISCORD_TOKEN")?, intents).await?;
    println!("Created cluster");

    cluster.up().await;
    println!("Started cluster");

    // Start exporter in a separate task
    tokio::task::spawn_blocking(move || exporter.run());

    while let Some(event) = events.next().await {
        println!("Event: {:?}", event.1.kind());
    }

    Ok(())
}
