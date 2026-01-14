use tokio::time::{self, Duration, Instant};
use twilight_gateway_queue::{IDENTIFY_DELAY, LIMIT_PERIOD, Queue};

pub async fn same_id_is_serial(queue: impl Queue) {
    let now = Instant::now();

    let t1 = queue.enqueue(0);
    let t2 = queue.enqueue(0);

    _ = t1.await;
    _ = t2.await;

    assert!(now.elapsed() >= IDENTIFY_DELAY, "ran concurrently");
}

/// Requires a queue with `max_concurrency` > 1.
pub async fn different_id_is_parallel(queue: impl Queue) {
    let now = Instant::now();

    let mut t1 = queue.enqueue(1);
    let t2 = queue.enqueue(0);

    tokio::select! {
        biased;
        _ = &mut t1 => panic!("not started in order"),
        _ = t2 => {
            _ = t1.await;
            assert!(now.elapsed() < IDENTIFY_DELAY, "ran serially");
        }
    }
}

/// Requires a queue with `remaining` of 0.
pub async fn reset_after_refills(queue: impl Queue, reset_after: Duration) {
    let now = Instant::now();

    let t1 = queue.enqueue(0);

    _ = t1.await;

    assert!(
        (now.elapsed().as_secs_f64() - reset_after.as_secs_f64()).abs() <= 1e-2,
        "did not refill in time"
    );
}

/// Requires a fresh queue with `remaining` of 1.
pub async fn reset_after_started(queue: impl Queue) {
    time::sleep(LIMIT_PERIOD / 2).await;

    let t1 = queue.enqueue(0);
    let t2 = queue.enqueue(0);

    _ = t1.await;

    let now = Instant::now();

    _ = t2.await;

    assert!(
        (now.elapsed().as_secs_f64() - LIMIT_PERIOD.as_secs_f64()).abs() <= 1e-2,
        "queue misstimed remaining refill"
    );
}

/// Requires a queue with `max_concurrency` >= 4.
pub async fn multi_bucket(queue: impl Queue) {
    let now = Instant::now();

    let t1 = queue.enqueue(0);
    let t2 = queue.enqueue(1);
    let t3 = queue.enqueue(3);
    let t4 = queue.enqueue(3);

    _ = t1.await;
    _ = t2.await;
    _ = t3.await;

    assert!(now.elapsed() < IDENTIFY_DELAY, "ran serially");

    _ = t4.await;

    assert!(now.elapsed() >= IDENTIFY_DELAY, "ran concurrently");
}
