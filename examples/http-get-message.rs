use futures_util::future;
use std::{env, future::IntoFuture};
use twilight_http::Client;
use twilight_model::id::Id;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    // Select rustls backend
    rustls::crypto::ring::default_provider().install_default().unwrap();

    let client = Client::new(env::var("DISCORD_TOKEN")?);
    let channel_id = Id::new(381_926_291_785_383_946);

    future::join_all((1u8..=10).map(|x| {
        client
            .create_message(channel_id)
            .content(&format!("Ping #{x}"))
            .into_future()
    }))
    .await;

    let me = client.current_user().await?.model().await?;
    println!("Current user: {}#{}", me.name, me.discriminator);

    Ok(())
}
