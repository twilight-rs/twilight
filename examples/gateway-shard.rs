use std::env;
use twilight_gateway::{Config, Intents, Shard, ShardId};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let intents = Intents::GUILD_MESSAGES | Intents::GUILD_VOICE_STATES;
    let config = Config::new(env::var("DISCORD_TOKEN")?, intents);
    let mut shard = Shard::new(ShardId::ONE, config);

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

        println!("Event: {event:?}");
    }

    Ok(())
}
