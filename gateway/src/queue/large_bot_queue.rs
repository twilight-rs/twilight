use super::{DayLimiter, Queue};
use async_trait::async_trait;
use futures_channel::{
    mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    oneshot::{self, Sender},
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use log::{info, warn};
use std::{fmt::Debug, time::Duration};
use tokio::time::delay_for;

/// Large bot queue is for bots that are marked as very large by Discord.
///
/// Usage with other bots will end up getting a large amount of failed identifies.
#[derive(Debug)]
pub struct LargeBotQueue {
    buckets: Vec<UnboundedSender<Sender<()>>>,
    limiter: DayLimiter,
}

impl LargeBotQueue {
    /// Creates a new large bot queue
    pub async fn new(buckets: usize, http: &twilight_http::Client) -> Self {
        let mut queues = Vec::with_capacity(buckets);
        for _ in 0..buckets {
            let (tx, rx) = unbounded();

            tokio::spawn(async {
                waiter(rx).await;
            });

            queues.push(tx)
        }

        let limiter = DayLimiter::new(http).await.expect(
            "Getting the first session limits failed, \
             Is network connection available?",
        );

        if log::log_enabled!(log::Level::Info) {
            let lock = limiter.0.lock().await;
            log::info!(
                "{}/{} identifies used before next reset in {:.2?}",
                lock.current,
                lock.total,
                lock.next_reset
            );
        }

        Self {
            buckets: queues,
            limiter,
        }
    }
}

async fn waiter(mut rx: UnboundedReceiver<Sender<()>>) {
    const DUR: Duration = Duration::from_secs(6);
    while let Some(req) = rx.next().await {
        if let Err(err) = req.send(()) {
            warn!(
                "[LargeBotQueue/waiter] send failed with: {:?}, skipping",
                err
            );
        }
        delay_for(DUR).await;
    }
}

#[async_trait]
impl Queue for LargeBotQueue {
    /// Request to be able to identify with the gateway. This will place this
    /// request behind all other requests, and the returned future will resolve
    /// once the request has been completed.
    async fn request(&self, shard_id: [u64; 2]) {
        #[allow(clippy::cast_possible_truncation)]
        let bucket = (shard_id[0] % (self.buckets.len() as u64)) as usize;
        let (tx, rx) = oneshot::channel();

        self.limiter.get().await;
        if let Err(err) = self.buckets[bucket].clone().send(tx).await {
            warn!("[LargeBotQueue] send failed with: {:?}, skipping", err);
            return;
        }

        info!("Waiting for allowance on shard: {}!", shard_id[0]);

        let _ = rx.await;
    }
}
