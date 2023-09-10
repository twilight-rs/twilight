mod common;

use common::*;
use tokio::time::{Duration, Instant};
use twilight_gateway_queue::{InMemoryQueue, Queue, IDENTIFY_DELAY};

#[tokio::test]
async fn disabled_is_instant() {
    let queue = InMemoryQueue::new(0, 0, Duration::ZERO, 0);
    let now = Instant::now();

    let t1 = queue.enqueue(0);
    let t2 = queue.enqueue(0);

    _ = t1.await;
    _ = t2.await;

    assert!(now.elapsed() < IDENTIFY_DELAY, "did not run instantly");
}

#[tokio::test]
async fn update_fills_bucket() {
    let queue = InMemoryQueue::new(1, 10, Duration::from_secs(60), 10);
    let now = Instant::now();

    // Background task not run due to single-threaded runtime.
    let t1 = queue.enqueue(0);
    let t2 = queue.enqueue(1);
    queue.update(2, 10, Duration::from_secs(60), 10);

    _ = t1.await;
    _ = t2.await;

    assert!(now.elapsed() < IDENTIFY_DELAY, "ran serially");
}

#[tokio::test(start_paused = true)]
async fn integration() {
    let mut queue = InMemoryQueue::new(1, 1000, Duration::ZERO, 1000);
    same_id_is_serial(&queue).await;

    queue = InMemoryQueue::new(2, 1000, Duration::ZERO, 1000);
    different_id_is_parallel(&queue).await;

    queue = InMemoryQueue::new(1, 0, Duration::from_secs(60), 1);
    reset_after_refills(&queue, Duration::from_secs(60)).await;

    queue = InMemoryQueue::new(1, 1, Duration::ZERO, 1);
    reset_after_started(&queue).await;

    queue = InMemoryQueue::new(4, 1000, Duration::ZERO, 1000);
    multi_bucket(queue).await;
}
