use super::{
    config::{Config, ShardScheme},
    error::{Error, Result},
};
use crate::{
    queue::{LocalQueue, Queue},
    shard::{event::EventType, Config as ShardConfig, Event, Information, Shard},
};
use futures::{
    future,
    lock::Mutex,
    stream::{SelectAll, Stream, StreamExt},
};
use std::{
    collections::HashMap,
    sync::{Arc, Weak},
};

struct ClusterRef {
    config: Config,
    queue: Box<dyn Queue>,
    shards: Arc<Mutex<HashMap<u64, Shard>>>,
}

/// A manager for multiple shards.
///
/// The Cluster can be cloned and will point to the same cluster, so you can
/// pass it around as needed.
///
/// # Examples
///
/// Refer to the module-level documentation for examples.
#[derive(Clone)]
pub struct Cluster(Arc<ClusterRef>);

impl Cluster {
    /// Creates a new cluster from a configuration without bringing it up.
    pub fn new(config: impl Into<Config>) -> Self {
        Self::from(config)
    }

    /// Returns an immutable reference to the configuration of this cluster.
    pub fn config(&self) -> &Config {
        &self.0.config
    }

    /// Brings up the cluster, starting all of the shards that it was configured
    /// to manage.
    ///
    /// # Examples
    ///
    /// Bring up a cluster, starting shards all 10 shards that a bot uses:
    ///
    /// ```no_run
    /// use dawn_gateway::cluster::{
    ///     config::{Config, ShardScheme},
    ///     Cluster,
    /// };
    /// use std::{
    ///     convert::TryFrom,
    ///     env,
    /// };
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let mut config = Config::builder(env::var("DISCORD_TOKEN")?);
    ///
    /// let scheme = ShardScheme::try_from((0..=9, 10))?;
    /// config.shard_scheme(scheme);
    /// let cluster = Cluster::new(config);
    ///
    /// // Finally, bring up the cluster.
    /// cluster.up().await?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::GettingGatewayInfo`] if the [configured shard scheme]
    /// is [`ShardScheme::Auto`].
    ///
    /// [`Error::GettingGatewayInfo`]: enum.Error.html#variant.GettingGatewayInfo
    /// [`ShardScheme::Auto`]: config/enum.ShardScheme.html#variant.Auto
    /// [configured shard scheme]: config/struct.Config.html#method.shard_scheme
    pub async fn up(&self) -> Result<()> {
        let [from, to, total] =
            match self.0.config.shard_scheme() {
                ShardScheme::Auto => {
                    let http = self.0.config.http_client();

                    let gateway = http.gateway().authed().await.map_err(|source| {
                        Error::GettingGatewayInfo {
                            source,
                        }
                    })?;

                    [0, gateway.shards - 1, gateway.shards]
                },
                ShardScheme::Range {
                    from,
                    to,
                    total,
                } => [from, to, total],
            };
        #[cfg(feature = "metrics")]
        {
            use std::convert::TryInto;
            metrics::gauge!("Cluster-Shard-Count", total.try_into().unwrap_or(-1));
        }
        future::join_all(
            (from..=to)
                .map(|id| Self::start(Arc::downgrade(&self.0), id, total))
                .collect::<Vec<_>>(),
        )
        .await;

        Ok(())
    }

    /// Brings down the cluster, stopping all of the shards that it's managing.
    pub async fn down(&self) {
        let lock = self.0.shards.lock().await;

        let tasks = lock.values().map(Shard::shutdown).collect::<Vec<_>>();

        future::join_all(tasks).await;
    }

    /// Returns a Shard by its ID.
    pub async fn shard(&self, id: u64) -> Option<Shard> {
        self.0.shards.lock().await.get(&id).cloned()
    }

    /// Returns information about all shards.
    ///
    /// # Examples
    ///
    /// After waiting a minute, print the ID, latency, and stage of each shard:
    ///
    /// ```no_run
    /// use dawn_gateway::cluster::Cluster;
    /// use std::{env, time::Duration};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let cluster = Cluster::new(env::var("DISCORD_TOKEN")?);
    /// cluster.up().await;
    ///
    /// tokio::time::delay_for(Duration::from_secs(60)).await;
    ///
    /// for (shard_id, info) in cluster.info().await {
    ///     println!(
    ///         "Shard {} is {} with an average latency of {:?}",
    ///         shard_id,
    ///         info.stage(),
    ///         info.latency().average(),
    ///     );
    /// }
    /// # Ok(()) }
    /// ```
    pub async fn info(&self) -> HashMap<u64, Information> {
        // Clone this out to prevent locking up access to all of the shards.
        let shards = self.0.shards.lock().await.clone();

        future::join_all(
            shards
                .into_iter()
                .map(|(id, shard)| async move { (id, shard.info().await) }),
        )
        .await
        .into_iter()
        .collect::<HashMap<_, _>>()
    }

    /// Returns a stream of events from all shards managed by this Cluster.
    ///
    /// Each item in the stream contains both the shard's ID and the event
    /// itself.
    pub async fn events(&self) -> impl Stream<Item = (u64, Event)> {
        let shards = self.0.shards.lock().await.clone();
        cluster_events(shards).await
    }

    /// Like [`events`], but filters the events so that the stream consumer
    /// receives only the selected event types.
    ///
    /// # Examples
    ///
    /// Retrieve a stream of events when a message is created, deleted, or
    /// updated:
    ///
    /// ```no_run
    /// use dawn_gateway::cluster::{Cluster, Event, EventType};
    /// use futures::StreamExt;
    /// use std::env;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let cluster = Cluster::from(env::var("DISCORD_TOKEN")?);
    /// cluster.up().await;
    ///
    /// let types = EventType::MESSAGE_CREATE
    ///     | EventType::MESSAGE_DELETE
    ///     | EventType::MESSAGE_UPDATE;
    /// let mut events = cluster.some_events(types).await;
    ///
    /// while let Some((shard_id, event)) = events.next().await {
    ///     match event {
    ///         Event::MessageCreate(_) => println!("Shard {} got a new message", shard_id),
    ///         Event::MessageDelete(_) => println!("Shard {} got a deleted message", shard_id),
    ///         Event::MessageUpdate(_) => println!("Shard {} got an updated message", shard_id),
    ///         // No other events will come in through the stream.
    ///         _ => {},
    ///     }
    /// }
    /// # Ok(()) }
    /// ```
    ///
    /// [`events`]: #method.events
    pub async fn some_events(&self, types: EventType) -> impl Stream<Item = (u64, Event)> {
        let shards = self.0.shards.lock().await.clone();
        cluster_some_events(shards, types).await
    }

    /// Queues a request to start a shard by ID and starts it once the queue
    /// accepts the request.
    ///
    /// Accepts weak references to the queue and map of shards, because by the
    /// time the future is polled the cluter may have already dropped, bringing
    /// down the queue and shards with it.
    async fn start(cluster: Weak<ClusterRef>, shard_id: u64, _: u64) -> Option<Shard> {
        cluster.upgrade()?.queue.request().await;

        let token = cluster.upgrade()?.config.shard_config().token().to_owned();
        let config = ShardConfig::builder(token).build();

        let shard = Shard::new(config).await.ok()?;

        if let Some(old) = cluster
            .upgrade()?
            .shards
            .lock()
            .await
            .insert(shard_id, shard.clone())
        {
            old.shutdown().await;
        }

        Some(shard)
    }
}

impl<T: Into<Config>> From<T> for Cluster {
    fn from(config: T) -> Self {
        Self(Arc::new(ClusterRef {
            config: config.into(),
            queue: Box::new(LocalQueue::new()),
            shards: Arc::new(Mutex::new(HashMap::new())),
        }))
    }
}

async fn cluster_events(
    shards: impl IntoIterator<Item = (u64, Shard)>,
) -> impl Stream<Item = (u64, Event)> {
    let mut all = SelectAll::new();

    for (id, shard) in shards {
        all.push(shard.events().await.map(move |e| (id, e)));
    }

    all
}

async fn cluster_some_events(
    shards: impl IntoIterator<Item = (u64, Shard)>,
    types: EventType,
) -> impl Stream<Item = (u64, Event)> {
    let mut all = SelectAll::new();

    for (id, shard) in shards {
        all.push(shard.some_events(types).await.map(move |e| (id, e)));
    }

    all
}
