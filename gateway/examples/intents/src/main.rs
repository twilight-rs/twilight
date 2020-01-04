use dawn_gateway::{shard::Config, Shard};
use futures::StreamExt;
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    pretty_env_logger::init_timed();

    let config = {
        use dawn_model::gateway::GatewayIntents;
        let mut conf = Config::builder(env::var("DISCORD_TOKEN")?);
        conf.intents(GatewayIntents::GUILD_MESSAGES | GatewayIntents::DIRECT_MESSAGES);
        conf.build()
    };

    let shard = Shard::new(config).await?;
    println!("Created shard");

    let mut events = shard.events().await;

    while let Some(event) = events.next().await {
        println!("Event: {:?}", event);
    }

    Ok(())
}
