use std::{env, error::Error};
use twilight_http::Client;
use twilight_model::applications::{BaseCommandOptionData, CommandOption};
use twilight_model::id::{ApplicationId, GuildId};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let client = Client::new(env::var("DISCORD_TOKEN")?);
    let aid = ApplicationId(0);
    let gid = GuildId(0);

    let me = client.current_user().await?;
    println!("Current user: {}#{}", me.name, me.discriminator);

    client
        .create_guild_command(aid, gid, "ping", "pongs")
        .push_command_option(CommandOption::User(BaseCommandOptionData {
            name: "who".to_owned(),
            description: "ping someone!".to_owned(),
            ..Default::default()
        }))
        .await
        .expect("create pong command");

    Ok(())
}
