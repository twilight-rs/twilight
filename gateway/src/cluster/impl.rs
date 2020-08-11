use super::config::{ClusterConfig, ShardScheme};
use crate::{
    shard::{CommandError, Information, ResumeSession, Shard},
    EventTypeFlags,
};
use dashmap::DashMap;
use futures_util::{
    future,
    stream::{SelectAll, Stream, StreamExt},
};
use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    iter::FromIterator,
    sync::Arc,
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
    /// This can occur when using [automatic sharding] and retrieval of the
    /// number of recommended number of shards to start fails, which can happen
    /// due to something like a network or response parsing issue.
    ///
    /// [automatic sharding]: config/enum.ShardScheme.html#variant.Auto
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
    shards: DashMap<u64, Shard>,
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

        let shards = (from..=to)
            .map(|idx| {
                let mut shard_config = config.shard_config().clone();
                shard_config.shard = [idx, total];

                if let Some(data) = config.resume_sessions().get(&idx) {
                    shard_config.session_id = Some(data.session_id.clone());
                    shard_config.sequence = Some(data.sequence);
                }

                (idx, Shard::new(shard_config))
            })
            .collect();

        Ok(Self(Arc::new(ClusterRef {
            config,
            shard_from: from,
            shard_to: to,
            shards,
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
            (self.0.shard_from..=self.0.shard_to).map(|id| Self::start(Arc::clone(&self.0), id)),
        )
        .await;
    }

    /// Brings down the cluster, stopping all of the shards that it's managing.
    pub fn down(&self) {
        for r in self.0.shards.iter() {
            r.value().shutdown();
        }
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
    pub fn down_resumable(&self) -> HashMap<u64, ResumeSession> {
        self.0
            .shards
            .iter()
            .map(|r| r.value().shutdown_resumable())
            .filter_map(|(id, session)| session.map(|s| (id, s)))
            .collect()
    }

    /// Returns a Shard by its ID.
    pub fn shard(&self, id: u64) -> Option<Shard> {
        self.0.shards.get(&id).map(|r| r.value().clone())
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
    /// for (shard_id, info) in cluster.info() {
    ///     println!(
    ///         "Shard {} is {} with an average latency of {:?}",
    ///         shard_id,
    ///         info.stage(),
    ///         info.latency().average(),
    ///     );
    /// }
    /// # Ok(()) }
    /// ```
    pub fn info(&self) -> HashMap<u64, Information> {
        self.0
            .shards
            .iter()
            .filter_map(|r| r.value().info().ok().map(|info| (*r.key(), info)))
            .collect()
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
        let shard = self
            .0
            .shards
            .get(&id)
            .map(|r| r.value().clone())
            .ok_or(ClusterCommandError::ShardNonexistent { id })?;

        shard
            .command(value)
            .await
            .map_err(|source| ClusterCommandError::Sending { source })
    }

    /// Returns a stream of events from all shards managed by this Cluster.
    ///
    /// Each item in the stream contains both the shard's ID and the event
    /// itself.
    pub fn events<'a>(&'a self) -> impl Stream<Item = (u64, Event)> + 'a {
        self.some_events(EventTypeFlags::default())
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
    /// let mut events = cluster.some_events(types);
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
    pub fn some_events<'a>(
        &'a self,
        types: EventTypeFlags,
    ) -> impl Stream<Item = (u64, Event)> + 'a {
        let stream = self
            .0
            .shards
            .iter()
            .map(|r| r.value().some_events(types).map(move |e| (*r.key(), e)));

        SelectAll::from_iter(stream)
    }

    /// Queues a request to start a shard by ID and starts it once the queue
    /// accepts the request.
    ///
    /// Accepts weak references to the queue and map of shards, because by the
    /// time the future is polled the cluster may have already dropped, bringing
    /// down the queue and shards with it.
    async fn start(cluster: Arc<ClusterRef>, shard_id: u64) -> Option<Shard> {
        let mut shard = cluster.shards.get(&shard_id)?.value().clone();

        shard.start().await.ok()?;

        Some(shard)
    }
}
