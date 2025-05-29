use std::{
    env,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
use tokio::{task::JoinSet, time};
use tokio_stream::StreamExt as _;
use tokio_util::sync::CancellationToken;
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
        // Dropping `JoinSet` aborts all its tasks.
        let mut set = JoinSet::new();
        for shard in shards {
            set.spawn(runner(shard));
        }

        shards = reshard(&client, config.clone(), config_callback).await?;
    }
}

#[tracing::instrument(fields(shard = %shard.id()), skip_all)]
async fn runner(mut shard: Shard) {
    while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
        let Ok(event) = item else {
            tracing::warn!(source = ?item.unwrap_err(), "error receiving event");

            continue;
        };

        // You'd normally want to spawn a new tokio task for each event and
        // handle the event there to not block the shard.
        tracing::debug!(?event, "received event");
    }
}

// Instrument to differentiate between the logs produced here and in `runner`.
async fn reshard(
    client: &Client,
    config: Config,
    config_callback: impl Fn(ShardId, ConfigBuilder) -> Config,
) -> anyhow::Result<Vec<Shard>> {
    // Reshard every eight hours. This is an arbitrary number.
    const RESHARD_DURATION: Duration = Duration::from_secs(60 * 60 * 8);

    time::sleep(RESHARD_DURATION).await;

    let info = client.gateway().authed().await?.model().await?;

    let shards =
        twilight_gateway::create_iterator(0..info.shards, info.shards, config, config_callback)
            .collect::<Vec<_>>();

    let expected_duration = estimate_identifed(
        info.shards,
        info.session_start_limit.max_concurrency,
        info.session_start_limit.remaining,
        Duration::from_millis(info.session_start_limit.reset_after),
        info.session_start_limit.total,
    );
    let timeout = Instant::now() + expected_duration;

    // Before swapping the old and new list of shards, try to identify them.
    // Don't try too hard, however, as large bots may never have all shards
    // identified at the same time.
    let mut identified = Arc::new(Vec::with_capacity(shards.len()));
    Arc::get_mut(&mut identified)
        .unwrap()
        .fill_with(|| AtomicBool::new(false));
    let ct = CancellationToken::new();
    let tasks = shards
        .into_iter()
        .map(|s| {
            let identified = identified.clone();
            let ct = ct.clone();
            tokio::spawn(identify(s, identified, timeout, ct))
        })
        .collect::<Vec<_>>();

    let mut shards = Vec::with_capacity(tasks.len());
    for jh in tasks {
        shards.push(jh.await?);
    }

    Ok(shards)
}

#[tracing::instrument(fields(shard = %shard.id()), skip_all)]
async fn identify(
    mut shard: Shard,
    identified: Arc<Vec<AtomicBool>>,
    deadline: Instant,
    ct: CancellationToken,
) -> Shard {
    loop {
        let identified_count = identified
            .iter()
            .fold(0, |acc, i| acc + i.load(Ordering::Relaxed) as usize);
        let future = ct.run_until_cancelled(time::timeout_at(deadline, shard.next()));

        match future.await {
            Some(Ok(Some(Err(source)))) => {
                tracing::warn!(?source, "error receiving message");
            }
            Some(Err(_)) if identified_count >= (identified.len() * 3) / 4 => {
                ct.cancel();
                return shard;
            }
            None => return shard,
            _ => {
                let is_identified = shard.state().is_identified();
                identified[shard.id().number() as usize].store(is_identified, Ordering::Relaxed);
            }
        }
    }
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
