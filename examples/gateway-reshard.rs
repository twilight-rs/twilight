use futures_util::StreamExt;
use std::{
    env,
    future::{poll_fn, Future},
    task::Poll,
    time::Duration,
};
use tokio::time;
use twilight_gateway::{
    stream::{self, ShardEventStream, ShardMessageStream},
    Config, ConfigBuilder, Intents, Shard, ShardId,
};
use twilight_http::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;
    let client = Client::new(token.clone());
    let config = Config::new(token, Intents::GUILDS);
    let config_callback = |_, builder: ConfigBuilder| builder.build();

    let mut shards = stream::create_recommended(&client, config.clone(), config_callback)
        .await?
        .collect::<Vec<_>>();

    loop {
        // Run the two futures concurrently, returning when the first branch
        // completes and cancels the other one.
        tokio::select! {
            _ = runner(shards) => break,
            new_shards = reshard(&client, config.clone(), config_callback) => {
                shards = new_shards?;
            }
        }
    }

    Ok(())
}

// Instrument to differentiate between the logs produced here and in `reshard`.
#[tracing::instrument(skip_all)]
async fn runner(mut shards: Vec<Shard>) {
    let mut stream = ShardEventStream::new(shards.iter_mut());

    while let Some((shard, event)) = stream.next().await {
        let event = match event {
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

// Instrument to differentiate between the logs produced here and in `runner`.
#[tracing::instrument(skip_all)]
async fn reshard(
    client: &Client,
    config: Config,
    config_callback: impl Fn(ShardId, ConfigBuilder) -> Config,
) -> anyhow::Result<Vec<Shard>> {
    // Reshard every eight hours. This is an arbitrary number.
    const RESHARD_DURATION: Duration = Duration::from_secs(60 * 60 * 8);

    time::sleep(RESHARD_DURATION).await;

    let info = client.gateway().authed().await?.model().await?;

    let mut shards =
        stream::create_range(.., info.shards, config, config_callback).collect::<Vec<_>>();

    let expected_duration = estimate_identifed(
        info.shards,
        info.session_start_limit.max_concurrency,
        info.session_start_limit.remaining,
        Duration::from_millis(info.session_start_limit.reset_after),
        info.session_start_limit.total,
    );
    tokio::pin! {
        let timeout = time::sleep(expected_duration);
    }
    // Register timer.
    poll_fn(|cx| {
        _ = timeout.as_mut().poll(cx);
        Poll::Ready(())
    })
    .await;

    // Before swapping the old and new list of shards, try to identify them.
    // Don't try too hard, however, as large bots may never have all shards
    // identified at the same time.
    let mut identified = vec![false; shards.len()];
    let mut stream = ShardMessageStream::new(shards.iter_mut());

    loop {
        let identified_count = identified.iter().map(|&i| i as usize).sum::<usize>();
        tokio::select! {
            _ = &mut timeout, if identified_count >= (identified.len() * 3) / 4 => {
                drop(stream);
                break;
            }
            Some(res) = stream.next() => {
                match res {
                    (_, Err(source)) => {
                        tracing::warn!(?source, "error receiving message");

                        if source.is_fatal() {
                            anyhow::bail!(source);
                        }
                    }
                    (shard, _) => {
                        identified[shard.id().number() as usize] = shard.status().is_identified();
                    }
                }
            }
        }
    }

    Ok(shards)
}

fn estimate_identifed(
    shards: u32,
    max_concurrency: u16,
    remaining: u32,
    reset_after: Duration,
    total: u32,
) -> Duration {
    const DAY: Duration = Duration::from_secs(60 * 60 * 24);

    let refills = shards / remaining;
    let buckets = (shards as f32 / max_concurrency as f32).round() as u64;
    reset_after * (refills > 0) as u32
        + (1..refills).map(|_| DAY).sum::<Duration>()
        + Duration::from_secs(5 * buckets % total as u64)
}
