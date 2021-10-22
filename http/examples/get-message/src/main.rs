use std::{env, error::Error, fs::File, io::Read};
use twilight_http::Client;
use twilight_model::id::ChannelId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let client = Client::new(env::var("DISCORD_TOKEN")?);
    let channel_id = ChannelId::new(745811002771374151).expect("non zero");

    let mut twilight_movie = File::open("twilight.jpg")?;
    let mut twilight_sparkle = File::open("twilight-sparkle.png")?;

    let mut tm_vec = Vec::new();
    let mut ts_vec = Vec::new();

    twilight_movie.read_to_end(&mut tm_vec)?;
    twilight_sparkle.read_to_end(&mut ts_vec)?;

    let files = [
        ("twilight-sparkle.png", ts_vec.as_slice()),
        ("twilight.jpg", tm_vec.as_slice()),
    ];

    client
        .create_message(channel_id)
        .content("Testing")
        .expect("content not a valid length")
        .files(&files)
        .exec()
        .await?;

    let me = client.current_user().exec().await?.model().await?;
    println!("Current user: {}#{}", me.name, me.discriminator);

    Ok(())
}
