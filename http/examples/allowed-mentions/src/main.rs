use twilight_http::Client;
use twilight_model::id::{ChannelId, UserId};
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    pretty_env_logger::init_timed();

    let client = Client::new(env::var("DISCORD_TOKEN")?);
    let channel_id = ChannelId(381_926_291_785_383_946);
    let user_id = UserId(77_469_400_222_932_992);

    client
        .create_message(channel_id)
        .content(format!("Hi <@{}>", user_id.0))
        .allowed_mentions()
        .parse_specific_users(vec![user_id])
        .build()
        .await?;

    Ok(())
}
