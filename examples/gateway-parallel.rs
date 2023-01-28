//! Run the recommended number of shards in `available_parallelism` tasks. Note
//! that neither [`ShardEventStream`] nor [`ShardMessageStream`] is required for
//! parallelism; each shard can run on an independent task.
//!
//! [`ShardMessageStream`]: twilight_gateway::stream::ShardMessageStream

use futures_util::{future::join_all, StreamExt};
use std::{env, iter, thread};
use tokio::{signal, sync::watch, task::JoinSet};
use twilight_gateway::{stream::ShardEventStream, CloseFrame, Config, Intents, Shard, ShardId};
use twilight_http::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;

    let client = Client::new(token.clone());

    let config = Config::new(token, Intents::GUILDS);
    let recommended_shards = client.gateway().authed().await?.model().await?.shards;
    let shards = {
        let tasks = thread::available_parallelism()?.get();

        // Split shards into a vec of `tasks` vecs of shards.
        let init = iter::repeat_with(Vec::new)
            .take(tasks)
            .collect::<Vec<Vec<_>>>();
        (0..recommended_shards)
            .map(|id| Shard::new(ShardId::new(id, recommended_shards), config.clone()))
            .enumerate()
            .fold(init, |mut fold, (idx, shard)| {
                fold[idx % tasks].push(shard);
                fold
            })
    };

    let (tx, rx) = watch::channel(false);

    let mut set = JoinSet::new();

    for mut shards in shards {
        let mut rx = rx.clone();
        set.spawn(async move {
            // Run `process` and `rx.changed()` concurrently, returning when
            // the first branch completes, cancelling the other one.
            tokio::select! {
                _ = process(shards.iter_mut()) => {},
                _ = rx.changed() => {
                    join_all(shards.iter_mut().map(|shard| async move {
                        shard.close(CloseFrame::NORMAL).await
                    })).await;
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

async fn process(shards: impl Iterator<Item = &mut Shard>) {
    let mut stream = ShardEventStream::new(shards);
    loop {
        let (shard, event) = match stream.next().await {
            Some((shard, Ok(event))) => (shard, event),
            Some((_, Err(source))) => {
                tracing::warn!(?source, "error receiving event");

                if source.is_fatal() {
                    break;
                }

                continue;
            }
            None => break,
        };

        tracing::debug!(?event, shard = ?shard.id(), "received event");
    }
}
