use futures_util::future;
use std::{env, future::IntoFuture};
use twilight_http::Client;
use twilight_model::id::Id;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let client = Client::new(env::var("DISCORD_TOKEN")?);
    let channel_id = Id::new(745811002771374151);

    client
        .create_message(channel_id)
        .content(&format!("Ping #{{x}}"))
        .expect("content not a valid length")
        .await.unwrap();

    let me = client.current_user().await?.model().await?;
    println!("Current user: {}#{}", me.name, me.discriminator);

    Ok(())
}
