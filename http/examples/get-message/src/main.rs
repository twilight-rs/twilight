use futures::future;
use std::{env, error::Error};
use twilight_http::Client;
use twilight_model::id::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let client = Client::new(env::var("DISCORD_TOKEN")?);
    let guild_id = GuildId(771785232151478302);

    let me = client.current_user().await?;
    println!("Current user: {}#{}", me.name, me.discriminator);

    let commands = client.get_guild_commands(407532991313739776.into(), guild_id).await?;
    println!("Commands: {:#?}", commands);
    
    Ok(())
}
