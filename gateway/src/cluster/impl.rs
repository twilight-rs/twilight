use super::{
    config::{ClusterConfig, ShardScheme},
    error::{Error, Result},
};
use crate::shard::ShardResumeData;
use crate::{
    shard::{Information, Shard},
    EventTypeFlags,
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
use twilight_model::gateway::event::Event;

#[derive(Debug)]
struct ClusterRef {
    config: ClusterConfig,
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
#[derive(Clone, Debug)]
pub struct Cluster(Arc<ClusterRef>);

impl Cluster {
    /// Creates a new cluster from a configuration without bringing it up.
    pub fn new(config: impl Into<ClusterConfig>) -> Self {
        Self::from(config)
    }

    /// Returns an immutable reference to the configuration of this cluster.
    pub fn config(&self) -> &ClusterConfig {
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
    /// use twilight_gateway::cluster::{
    ///     config::{ClusterConfig, ShardScheme},
    ///     Cluster,
    /// };
    /// use std::{
    ///     convert::TryFrom,
    ///     env,
    /// };
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let scheme = ShardScheme::try_from((0..=9, 10))?;
    /// let mut config = ClusterConfig::builder(env::var("DISCORD_TOKEN")?)
    ///                         .shard_scheme(scheme)
    ///                         .build();
    ///
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
    /// [configured shard scheme]: config/struct.ClusterConfig.html#method.shard_scheme
    pub async fn up(&self) -> Result<()> {
        let [from, to, total] = match self.0.config.shard_scheme() {
            ShardScheme::Auto => {
                let http = self.0.config.http_client();

                let gateway = http
                    .gateway()
                    .authed()
                    .await
                    .map_err(|source| Error::GettingGatewayInfo { source })?;

                [0, gateway.shards - 1, gateway.shards]
            }
            ShardScheme::Range { from, to, total } => [from, to, total],
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

    /// Brings down the cluster in a resumable way and returns all info needed for resuming
    ///
    /// Note discord only allows resuming for a few minutes after disconnection. You can also not resume if you missed too many events already
    pub async fn down_resumable(&self) -> Vec<(u64, Option<ShardResumeData>)> {
        let lock = self.0.shards.lock().await;

        let tasks = lock
            .values()
            .map(Shard::shutdown_resumable)
            .collect::<Vec<_>>();

        future::join_all(tasks).await
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
    /// use twilight_gateway::cluster::Cluster;
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

    /// Send a command to the specified shard.
    ///
    /// # Errors
    /// Fails if command could not be serialized or if the shard does not exist.
    pub async fn command(&self, id: u64, com: &impl serde::Serialize) -> Result<()> {
        let shard = match self.0.shards.lock().await.get(&id) {
            Some(shard) => shard.clone(),
            None => return Err(Error::ShardDoesNotExist { id }),
        };

        shard
            .command(com)
            .await
            .map_err(|err| Error::ShardError { source: err })?;

        Ok(())
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
    /// use twilight_gateway::{Cluster, EventTypeFlags, Event};
    /// use futures::StreamExt;
    /// use std::env;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let cluster = Cluster::from(env::var("DISCORD_TOKEN")?);
    /// cluster.up().await;
    ///
    /// let types = EventTypeFlags::MESSAGE_CREATE
    ///     | EventTypeFlags::MESSAGE_DELETE
    ///     | EventTypeFlags::MESSAGE_UPDATE;
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
    pub async fn some_events(&self, types: EventTypeFlags) -> impl Stream<Item = (u64, Event)> {
        let shards = self.0.shards.lock().await.clone();
        cluster_some_events(shards, types).await
    }

    /// Queues a request to start a shard by ID and starts it once the queue
    /// accepts the request.
    ///
    /// Accepts weak references to the queue and map of shards, because by the
    /// time the future is polled the cluster may have already dropped, bringing
    /// down the queue and shards with it.
    async fn start(cluster: Weak<ClusterRef>, shard_id: u64, shard_total: u64) -> Option<Shard> {
        let cluster = cluster.upgrade()?;

        let mut config = cluster.config.shard_config().clone();

        config.shard = [shard_id, shard_total];
        let resume_sessions = cluster.config.resume_sessions().get(&shard_id);
        if let Some(data) = resume_sessions {
            config.session_id = Some(data.session_id.clone());
            config.sequence = Some(data.sequence);
        };

        let shard = Shard::new(config).await.ok()?;

        if let Some(old) = cluster.shards.lock().await.insert(shard_id, shard.clone()) {
            old.shutdown().await;
        }

        Some(shard)
    }
}

impl<T: Into<ClusterConfig>> From<T> for Cluster {
    fn from(config: T) -> Self {
        Self(Arc::new(ClusterRef {
            config: config.into(),
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
    types: EventTypeFlags,
) -> impl Stream<Item = (u64, Event)> {
    let mut all = SelectAll::new();

    for (id, shard) in shards {
        all.push(shard.some_events(types).await.map(move |e| (id, e)));
    }

    all
}
