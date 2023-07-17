//! Run the recommended number of shards, each in their own task.

use std::env;
use tokio::{signal, sync::watch, task::JoinSet};
use twilight_gateway::{stream, CloseFrame, Config, Intents, Shard};
use twilight_http::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;
    let client = Client::new(token.clone());
    let config = Config::new(token, Intents::GUILDS);

    let shards = stream::create_recommended(&client, config, |_, builder| builder.build()).await?;

    let (tx, rx) = watch::channel(false);

    let mut set = JoinSet::new();

    for mut shard in shards {
        let mut rx = rx.clone();
        set.spawn(async move {
            // Run the two futures concurrently, returning when the first branch
            // completes and cancels the other one.
            tokio::select! {
                _ = runner(&mut shard) => {},
                _ = rx.changed() => {
                    _ = shard.close(CloseFrame::NORMAL).await;
                }
            }
        });
    }

    signal::ctrl_c().await?;

    tracing::debug!("shutting down");

    // Instruct the tasks to shutdown.
    tx.send(true)?;

    // Await all spawned tasks.
    while set.join_next().await.is_some() {}

    Ok(())
}

async fn runner(shard: &mut Shard) {
    loop {
        let event = match shard.next_event().await {
            Ok(event) => event,
            Err(source) => {
                tracing::warn!(?source, "error receiving event");

                if source.is_fatal() {
                    break;
                }

                continue;
            }
        };

        tracing::debug!(?event, shard = ?shard.id(), "received event");
    }
}
