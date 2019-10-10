use async_trait::async_trait;
use futures_channel::oneshot::{self, Sender};
use futures_util::lock::Mutex;
use log::warn;
use std::{
    collections::VecDeque,
    fmt::Debug,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
        Weak,
    },
    time::Duration,
};

#[async_trait]
pub trait Queue: Debug + Send + Sync {
    async fn request(&self);
    async fn is_running(&self) -> bool;
}

#[derive(Debug, Default)]
struct LocalQueueRef {
    requests: Arc<Mutex<VecDeque<Sender<()>>>>,
    task_running: AtomicBool,
}

/// A local, in-process implementation of a [`Queue`] which manages the
/// connection attempts of one or more [`Shard`]s.
///
/// The queue will take incoming requests and then queue them, releasing one of
/// the requests every 6 seconds. The queue is necessary because there's a
/// ratelimit on how often shards can initiate sessions.
///
/// You usually won't need to handle this yourself, because the [`Cluster`] will
/// do that for you when managing multiple shards.
///
/// # When not to use this
///
/// This queue implementation is "local", meaning it's intended to be used if
/// you manage shards only in this process. If you run shards in multiple
/// different processes (do you utilize microservices a lot?), then you **must
/// not** use this implementation. Shards across multiple processes may
/// create new sessions at the same time, which is bad.
///
/// If you can't use this, look into an alternative implementation of the
/// [`Queue`], such as the [`gateway-queue`] broker.
///
/// [`Cluster`]: ../cluster/struct.Cluster.html
/// [`Shard`]: ../shard/struct.Shard.html
/// [`gateway-queue`]: https://github.com/dawn-rs/gateway-queue
#[derive(Clone, Debug, Default)]
pub struct LocalQueue(Arc<LocalQueueRef>);

impl LocalQueue {
    /// Creates a new local queue.
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Queue for LocalQueue {
    /// Request to be able to identify with the gateway. This will place this
    /// request behind all other requests, and the returned future will resolve
    /// once the request has been completed.
    async fn request(&self) {
        let (tx, rx) = oneshot::channel();

        self.0.requests.lock().await.push_back(tx);

        if !self.is_running().await {
            let inner = Arc::downgrade(&self.0);

            tokio_executor::spawn(async {
                queue_spawner(inner).await;
            });

            self.0.task_running.store(true, Ordering::Release);
        }

        let _ = rx.await;
    }

    /// Whether the queue is actively going through requests.
    ///
    /// Once all requests have been completed, this will return `false`.
    async fn is_running(&self) -> bool {
        self.0.task_running.load(Ordering::Relaxed)
    }
}

async fn queue_spawner(queue: Weak<LocalQueueRef>) -> Option<()> {
    const DUR: Duration = Duration::from_secs(6);

    while let Some(req) = queue.upgrade()?.requests.lock().await.pop_front() {
        if let Err(()) = req.send(()) {
            warn!("Request rx dropped before success");
        }

        tokio_timer::delay_for(DUR).await;
    }

    tokio_timer::delay_for(DUR).await;

    queue
        .upgrade()?
        .task_running
        .store(false, Ordering::Release);

    Some(())
}
