use super::config::{ClusterConfig, ShardScheme};
use crate::{
    shard::{CommandError, Information, ResumeSession, Shard},
    EventTypeFlags,
};
use futures_util::{
    future,
    lock::Mutex,
    stream::{SelectAll, Stream, StreamExt},
};
use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    iter::FromIterator,
    sync::{Arc, Weak},
};
use twilight_http::Error as HttpError;
use twilight_model::gateway::event::Event;

/// Sending a command to a shard failed.
#[derive(Debug)]
pub enum ClusterCommandError {
    /// The shard exists, but sending the provided value failed.
    Sending {
        /// Reason for the error.
        source: CommandError,
    },
    /// The provided shard ID does not exist.
    ShardNonexistent {
        /// The provided shard ID.
        id: u64,
    },
}

impl Display for ClusterCommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::Sending { source } => Display::fmt(source, f),
            Self::ShardNonexistent { id } => {
                f.write_fmt(format_args!("shard {} does not exist", id,))
            }
        }
    }
}

impl Error for ClusterCommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Sending { source } => Some(source),
            Self::ShardNonexistent { .. } => None,
        }
    }
}

/// Starting a cluster failed.
#[derive(Debug)]
pub enum ClusterStartError {
    /// Retrieving the bot's gateway information via the HTTP API failed.
    ///
    /// This information includes the number of shards for the cluster to
    /// automatically use.
    RetrievingGatewayInfo {
        /// Reason for the error.
        source: HttpError,
    },
}

impl Display for ClusterStartError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::RetrievingGatewayInfo { .. } => {
                f.write_str("getting the bot's gateway info failed")
            }
        }
    }
}

impl Error for ClusterStartError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::RetrievingGatewayInfo { source } => Some(source),
        }
    }
}

#[derive(Debug)]
struct ClusterRef {
    config: ClusterConfig,
    shard_from: u64,
    shard_to: u64,
    shards: Mutex<HashMap<u64, Shard>>,
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
    ///
    /// # Errors
    ///
    /// Returns [`ClusterStartError::RetrievingGatewayInfo`] if there was an HTTP error Retrieving
    /// the gateway information.
    ///
    /// [`ClusterStartError::RetrievingGatewayInfo`]: enum.ClusterStartError.html#variant.RetrievingGatewayInfo
    pub async fn new(config: impl Into<ClusterConfig>) -> Result<Self, ClusterStartError> {
        Self::_new(config.into()).await
    }

    async fn _new(config: ClusterConfig) -> Result<Self, ClusterStartError> {
        let [from, to, total] = match config.shard_scheme() {
            ShardScheme::Auto => {
                let http = config.http_client();

                let gateway = http
                    .gateway()
                    .authed()
                    .await
                    .map_err(|source| ClusterStartError::RetrievingGatewayInfo { source })?;

                [0, gateway.shards - 1, gateway.shards]
            }
            ShardScheme::Range { from, to, total } => [from, to, total],
        };

        #[cfg(feature = "metrics")]
        {
            use std::convert::TryInto;

            metrics::gauge!("Cluster-Shard-Count", total.try_into().unwrap_or(-1));
        }

        let mut shards = HashMap::new();

        for idx in from..=to {
            let mut shard_config = config.shard_config().clone();
            shard_config.shard = [idx, total];
            let resume_sessions = config.resume_sessions().get(&idx);

            if let Some(data) = resume_sessions {
                shard_config.session_id = Some(data.session_id.clone());
                shard_config.sequence = Some(data.sequence);
            };

            shards.insert(idx, Shard::new(shard_config));
        }

        Ok(Self(Arc::new(ClusterRef {
            config,
            shard_from: from,
            shard_to: to,
            shards: Mutex::new(shards),
        })))
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
    /// let cluster = Cluster::new(config).await?;
    ///
    /// // Finally, bring up the cluster.
    /// cluster.up().await;
    /// # Ok(()) }
    /// ```
    pub async fn up(&self) {
        future::join_all(
            (self.0.shard_from..=self.0.shard_to)
                .map(|id| Self::start(Arc::downgrade(&self.0), id))
                .collect::<Vec<_>>(),
        )
        .await;
    }

    /// Brings down the cluster, stopping all of the shards that it's managing.
    pub async fn down(&self) {
        let lock = self.0.shards.lock().await;

        let tasks = lock.values().map(Shard::shutdown).collect::<Vec<_>>();

        future::join_all(tasks).await;
    }

    /// Brings down the cluster in a resumable way and returns all info needed
    /// for resuming.
    ///
    /// The returned map is keyed by the shard's ID to the information needed
    /// to resume. If a shard can't resume, then it is not included in the map.
    ///
    /// **Note**: Discord only allows resuming for a few minutes after
    /// disconnection. You may also not be able to resume if you missed too many
    /// events already.
    pub async fn down_resumable(&self) -> HashMap<u64, ResumeSession> {
        let lock = self.0.shards.lock().await;

        let tasks = lock
            .values()
            .map(Shard::shutdown_resumable)
            .collect::<Vec<_>>();

        let sessions = future::join_all(tasks).await;

        HashMap::from_iter(
            sessions
                .into_iter()
                .filter_map(|(shard_id, session)| session.map(|session| (shard_id, session))),
        )
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
    /// let cluster = Cluster::new(env::var("DISCORD_TOKEN")?).await?;
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
        .filter_map(|(id, info)| info.map(|info| (id, info)).ok())
        .collect::<HashMap<_, _>>()
    }

    /// Send a command to the specified shard.
    ///
    /// # Errors
    ///
    /// Returns [`ClusterCommandError::Sending`] if the shard exists, but
    /// sending it failed.
    ///
    /// Returns [`ClusterCommandError::ShardNonexistent`] if the provided shard
    /// ID does not exist in the cluster.
    ///
    /// [`ClusterCommandError::Sending`]: enum.ClusterCommandError.html#variant.Sending
    /// [`ClusterCommandError::ShardNonexistent`]: enum.ClusterCommandError.html#variant.ShardNonexistent
    pub async fn command(
        &self,
        id: u64,
        value: &impl serde::Serialize,
    ) -> Result<(), ClusterCommandError> {
        let shard = match self.0.shards.lock().await.get(&id) {
            Some(shard) => shard.clone(),
            None => return Err(ClusterCommandError::ShardNonexistent { id }),
        };

        shard
            .command(value)
            .await
            .map_err(|source| ClusterCommandError::Sending { source })?;

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
    /// let cluster = Cluster::new(env::var("DISCORD_TOKEN")?).await?;
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
    async fn start(cluster: Weak<ClusterRef>, shard_id: u64) -> Option<Shard> {
        let cluster = cluster.upgrade()?;
        let mut shard = cluster.shards.lock().await.get(&shard_id).cloned()?;
        shard.start().await.ok()?;

        Some(shard)
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
