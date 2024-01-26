use std::{
    env,
    future::{poll_fn, Future},
    task::Poll,
    time::Duration,
};
use tokio::{task::JoinSet, time};
use tokio_stream::{StreamExt as _, StreamMap};
use twilight_gateway::{
    Config, ConfigBuilder, EventTypeFlags, Intents, Shard, ShardId, StreamExt as _,
};
use twilight_http::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN")?;
    let client = Client::new(token.clone());
    let config = Config::new(token, Intents::GUILDS);
    let config_callback = |_, builder: ConfigBuilder| builder.build();

    let mut shards = twilight_gateway::create_recommended(&client, config.clone(), config_callback)
        .await?
        .collect::<Vec<_>>();

    loop {
        // Dopping `JoinSet` aborts all its tasks.
        let mut set = JoinSet::new();
        for mut shard in shards {
            set.spawn(async move {
                while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
                    let Ok(event) = item else {
                        tracing::warn!(source = ?item.unwrap_err(), "error receiving event");

                        continue;
                    };

                    // You'd normally want to spawn a new tokio task for each event and
                    // handle the event there to not block the shard.
                    tracing::debug!(?event, shard = ?shard.id(), "received event");
                }
            });
        }

        shards = reshard(&client, config.clone(), config_callback).await?;
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
        twilight_gateway::create_iterator(0..info.shards, info.shards, config, config_callback)
            .collect::<Vec<_>>();

    let expected_duration = estimate_identifed(
        info.shards,
        info.session_start_limit.max_concurrency,
        info.session_start_limit.remaining,
        Duration::from_millis(info.session_start_limit.reset_after),
        info.session_start_limit.total,
    );
    let timeout = time::sleep(expected_duration);
    tokio::pin!(timeout);
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
    let mut stream = StreamMap::from_iter(shards.iter_mut().map(|s| (s.id(), s)));

    loop {
        let identified_count = identified.iter().map(|&i| i as usize).sum::<usize>();
        tokio::select! {
            _ = &mut timeout, if identified_count >= (identified.len() * 3) / 4 => break,
            Some(item) = stream.next() => {
                match item {
                    (_, Err(source)) => tracing::warn!(?source, "error receiving message"),
                    (shard_id, _) => {
                        identified[shard_id.number() as usize] = stream
                            .values()
                            .find(|shard| shard.id() == shard_id)
                            .unwrap()
                            .state()
                            .is_identified();
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
