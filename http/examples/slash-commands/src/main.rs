use std::{env, error::Error};
use twilight_http::Client;
use twilight_model::applications::command::{BaseCommandOptionData, CommandOption};
use twilight_model::id::{ApplicationId, GuildId};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let aid = ApplicationId(0);
    let gid = GuildId(0);

    let client = Client::builder()
        .token(env::var("DISCORD_TOKEN")?)
        .application_id(aid)
        .build();

    let me = client.current_user().await?;
    println!("Current user: {}#{}", me.name, me.discriminator);

    client
        .create_guild_command(gid, "ping", "pongs")?
        .push_command_option(CommandOption::User(BaseCommandOptionData {
            name: "who".to_owned(),
            description: "ping someone!".to_owned(),
            ..Default::default()
        }))
        .await
        .expect("create pong command");

    Ok(())
}
