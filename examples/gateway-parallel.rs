//! Run the recommended number of shards in `available_parallelism` tasks. Note
//! that neither [`ShardEventStream`] nor [`ShardMessageStream`] is required for
//! parallelism; each shard can run on an independent task.
//!
//! [`ShardMessageStream`]: twilight_gateway::stream::ShardMessageStream

use futures_util::{future::join_all, StreamExt};
use std::{env, iter, sync::Arc, thread};
use tokio::{signal, sync::watch, task::JoinSet};
use twilight_gateway::{
    message::CloseFrame,
    queue::LocalQueue,
    stream::{self, ShardEventStream},
    Config, Intents, Shard,
};
use twilight_http::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;
    let client = Client::new(token.clone());

    let queue = Arc::new(LocalQueue::new());
    // callback to create a config for each shard, useful for when not all
    // shards have the same configuration, such as for per-shard presences
    let config_callback = |_| {
        Config::builder(token.clone(), Intents::GUILDS)
            .queue(queue.clone())
            .build()
    };

    let tasks = thread::available_parallelism()?.get();

    // Split shards into a vec of `tasks` vecs of shards.
    let init = iter::repeat_with(Vec::new)
        .take(tasks)
        .collect::<Vec<Vec<_>>>();
    let shards = stream::create_recommended(&client, config_callback)
        .await?
        .enumerate()
        .fold(init, |mut fold, (idx, shard)| {
            fold[idx % tasks].push(shard);
            fold
        });

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
