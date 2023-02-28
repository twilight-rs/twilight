use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use std::{iter, time::Duration};
use tokio::{runtime, time::Instant};
use twilight_gateway::CommandRatelimiter;

/// Results in a capacity of 118
const HEARTBEAT_INTERVAL: Duration = Duration::MAX;
const PERIOD: Duration = Duration::from_secs(60);

fn ratelimiter(c: &mut Criterion) {
    let mut group = c.benchmark_group("command ratelimiter");
    let elapsed = iter::repeat_with(|| Instant::now() + PERIOD);
    let fresh = iter::repeat_with(|| Instant::now());
    let rt = || {
        runtime::Builder::new_current_thread()
            .enable_time()
            .build()
            .unwrap()
    };

    let bench = |mut data: CommandRatelimiter| async move { data._acquire().await };

    group.bench_function("empty", move |b| {
        b.to_async(rt()).iter_batched(
            || CommandRatelimiter::_new(HEARTBEAT_INTERVAL, []),
            bench,
            BatchSize::SmallInput,
        )
    });

    group.bench_function("half elapsed", move |b| {
        b.to_async(rt()).iter_batched(
            || CommandRatelimiter::_new(HEARTBEAT_INTERVAL, elapsed.take(118 / 2)),
            bench,
            BatchSize::SmallInput,
        )
    });

    group.bench_function("half full", move |b| {
        b.to_async(rt()).iter_batched(
            || CommandRatelimiter::_new(HEARTBEAT_INTERVAL, fresh.take(118 / 2)),
            bench,
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(benches, ratelimiter);
criterion_main!(benches);
