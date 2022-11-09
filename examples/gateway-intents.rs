use std::env;
use twilight_gateway::{Intents, Shard, ShardId};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let intents = Intents::GUILD_MESSAGES | Intents::DIRECT_MESSAGES;
    let mut shard = Shard::new(ShardId::ONE, env::var("DISCORD_TOKEN")?, intents);
    println!("Created shard");

    loop {
        let event = match shard.next_event().await {
            Ok(event) => event,
            Err(source) => {
                tracing::warn!(
                    source = &source as &dyn std::error::Error,
                    "error receiving event"
                );

                if source.is_fatal() {
                    break;
                }

                continue;
            }
        };

        println!("Event: {event:?}");
    }

    Ok(())
}
