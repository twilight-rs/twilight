use metrics_runtime::{exporters::LogExporter, observers::JsonBuilder, Receiver};
use std::{env, time::Duration};
use twilight_gateway::{Intents, Shard, ShardId};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let intents =
        Intents::GUILD_BANS | Intents::GUILD_EMOJIS_AND_STICKERS | Intents::GUILD_MESSAGES;
    let mut shard = Shard::new(ShardId::ONE, env::var("DISCORD_TOKEN")?, intents).await?;
    println!("Created shard");

    // Start exporter in a separate task
    tokio::task::spawn_blocking(move || exporter.run());

    loop {
        let event = match shard.next_event().await {
            Ok(event) => event,
            Err(source) => {
                tracing::warn!(?source, "error receiving event");

                if source.is_fatal() {
                    break;
                }

                continue;
            }
        };

        println!("event: {:?}", event.kind());
    }

    Ok(())
}
