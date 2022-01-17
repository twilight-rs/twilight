use std::{env, error::Error};
use twilight_http::Client;
use twilight_model::{
    channel::message::allowed_mentions::{AllowedMentions, AllowedMentionsBuilder},
    id::Id,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    //if we want to set the default for allowed mentions we need to use the builder, keep in mind these calls can't be chained!
    let client = Client::builder()
        .token(env::var("DISCORD_TOKEN")?)
        //add an empty allowed mentions, this will prevent any and all pings
        .default_allowed_mentions(AllowedMentions::default())
        .build();

    let channel_id = Id::new(381_926_291_785_383_946);
    let user_id = Id::new(77_469_400_222_932_992);

    // Since we wish to warn a user that they attempted to ping @everyone, we
    // allow the user ID to be pinged with allowed mentions.
    let allowed_mentions = AllowedMentionsBuilder::new()
        .user_ids(Vec::from([user_id]))
        .build();

    client
        .create_message(channel_id)
        .content(&format!(
            "<@{}> you are not allowed to ping @everyone!",
            user_id
        ))?
        .allowed_mentions(Some(&allowed_mentions))
        .exec()
        .await?;

    Ok(())
}
