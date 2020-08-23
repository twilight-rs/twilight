use futures::StreamExt;
use std::{env, error::Error, future::Future, pin::Pin, sync::Arc};
use twilight_gateway::{queue::Queue, Shard};

#[derive(Debug)]
struct BadQueue;

impl Queue for BadQueue {
    // DISCLAIMER: THIS IS A VERY BAD QUEUE!
    fn request(&'_ self, _shard_id: [u64; 2]) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async {})
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;
    let mut shard = Shard::builder(&token)
        .queue(Arc::new(Box::new(BadQueue)))
        .build();

    shard.start().await?;
    println!("Created shard");

    let mut events = shard.events();

    while let Some(event) = events.next().await {
        println!("Event: {:?}", event.kind());
    }

    Ok(())
}
