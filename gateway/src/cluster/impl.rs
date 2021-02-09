use super::{
    builder::{ClusterBuilder, ShardScheme},
    config::Config,
};
use crate::{
    shard::{raw_message::Message, Information, ResumeSession, Shard},
    EventTypeFlags, Intents,
};
use futures_util::{
    future,
    stream::{SelectAll, Stream, StreamExt},
};
use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    iter::FromIterator,
    sync::{Arc, Mutex},
};
use twilight_model::gateway::event::Event;

/// Sending a command to a shard failed.
#[derive(Debug)]
pub struct ClusterCommandError {
    kind: ClusterCommandErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl ClusterCommandError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub fn kind(&self) -> &ClusterCommandErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        ClusterCommandErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, self.source)
    }

    fn from_send(error: ClusterSendError) -> Self {
        let (kind, source) = error.into_parts();

        match kind {
            ClusterSendErrorType::Sending => Self {
                source,
                kind: ClusterCommandErrorType::Sending,
            },
            ClusterSendErrorType::ShardNonexistent { id } => Self {
                source,
                kind: ClusterCommandErrorType::ShardNonexistent { id },
            },
        }
    }
}

impl Display for ClusterCommandError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            ClusterCommandErrorType::Sending => {
                f.write_str("sending the message over the websocket failed")
            }
            ClusterCommandErrorType::ShardNonexistent { id } => {
                f.write_fmt(format_args!("shard {} does not exist", id,))
            }
        }
    }
}

impl Error for ClusterCommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`ClusterCommandError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum ClusterCommandErrorType {
    /// The shard exists, but sending the provided value failed.
    Sending,
    /// Provided shard ID does not exist.
    ShardNonexistent {
        /// Provided shard ID.
        id: u64,
    },
}

/// Sending a raw websocket message via a shard failed.
#[derive(Debug)]
pub struct ClusterSendError {
    kind: ClusterSendErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl ClusterSendError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub fn kind(&self) -> &ClusterSendErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (ClusterSendErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }
}

impl Display for ClusterSendError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            ClusterSendErrorType::Sending => f.write_str("failed to send message over websocket"),
            ClusterSendErrorType::ShardNonexistent { id } => {
                f.write_fmt(format_args!("shard {} does not exist", id))
            }
        }
    }
}

impl Error for ClusterSendError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`ClusterSendError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum ClusterSendErrorType {
    /// The shard exists, but sending the provided value failed.
    Sending,
    /// Provided shard ID does not exist.
    ShardNonexistent {
        /// Provided shard ID.
        id: u64,
    },
}

/// Starting a cluster failed.
#[derive(Debug)]
pub struct ClusterStartError {
    kind: ClusterStartErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl ClusterStartError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub fn kind(&self) -> &ClusterStartErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (ClusterStartErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, self.source)
    }
}

impl Display for ClusterStartError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            ClusterStartErrorType::RetrievingGatewayInfo { .. } => {
                f.write_str("getting the bot's gateway info failed")
            }
        }
    }
}

impl Error for ClusterStartError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`ClusterStartErrorType`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum ClusterStartErrorType {
    /// Retrieving the bot's gateway information via the HTTP API failed.
    ///
    /// This can occur when using [automatic sharding] and retrieval of the
    /// number of recommended number of shards to start fails, which can happen
    /// due to something like a network or response parsing issue.
    ///
    /// [automatic sharding]: ShardScheme::Auto
    RetrievingGatewayInfo,
}

#[derive(Debug)]
struct ClusterRef {
    config: Config,
    shard_from: u64,
    shard_to: u64,
    shards: Mutex<HashMap<u64, Shard>>,
}

/// A manager for multiple shards.
///
/// The Cluster can be cloned and will point to the same cluster, so you can
/// pass it around as needed.
///
/// # Cloning
///
/// The cluster internally wraps its data within an Arc. This means that the
/// cluster can be cloned and passed around tasks and threads cheaply.
///
/// # Examples
///
/// Refer to the module-level documentation for examples.
#[derive(Clone, Debug)]
pub struct Cluster(Arc<ClusterRef>);

impl Cluster {
    /// Create a new unconfigured cluster.
    ///
    /// Use [`builder`] to configure and construct a cluster.
    ///
    /// # Errors
    ///
    /// Returns a [`ClusterStartErrorType::RetrievingGatewayInfo`] error type if
    /// there was an HTTP error Retrieving the gateway information.
    ///
    /// [`builder`]: Self::builder
    pub async fn new(
        token: impl Into<String>,
        intents: Intents,
    ) -> Result<Self, ClusterStartError> {
        Self::builder(token, intents).build().await
    }

    pub(super) async fn new_with_config(mut config: Config) -> Result<Self, ClusterStartError> {
        let [from, to, total] = match config.shard_scheme() {
            ShardScheme::Auto => {
                let http = config.http_client();

                let gateway =
                    http.gateway()
                        .authed()
                        .await
                        .map_err(|source| ClusterStartError {
                            kind: ClusterStartErrorType::RetrievingGatewayInfo,
                            source: Some(Box::new(source)),
                        })?;

                [0, gateway.shards - 1, gateway.shards]
            }
            ShardScheme::Range { from, to, total } => [*from, *to, *total],
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

                if let Some(data) = config.resume_sessions.remove(&idx) {
                    shard_config.session_id = Some(data.session_id.into_boxed_str());
                    shard_config.sequence = Some(data.sequence);
                }

                (idx, Shard::new_with_config(shard_config))
            })
            .collect();

        Ok(Self(Arc::new(ClusterRef {
            config,
            shard_from: from,
            shard_to: to,
            shards: Mutex::new(shards),
        })))
    }

    /// Create a builder to configure and construct a cluster.
    pub fn builder(token: impl Into<String>, intents: Intents) -> ClusterBuilder {
        ClusterBuilder::new(token, intents)
    }

    /// Return an immutable reference to the configuration of this cluster.
    pub fn config(&self) -> &Config {
        &self.0.config
    }

    /// Bring up the cluster, starting all of the shards that it was configured
    /// to manage.
    ///
    /// # Examples
    ///
    /// Bring up a cluster, starting shards all 10 shards that a bot uses:
    ///
    /// ```no_run
    /// use twilight_gateway::{cluster::{Cluster, ShardScheme}, Intents};
    /// use std::{
    ///     convert::TryFrom,
    ///     env,
    /// };
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let token = env::var("DISCORD_TOKEN")?;
    /// let scheme = ShardScheme::try_from((0..=9, 10))?;
    /// let cluster = Cluster::builder(token, Intents::GUILD_MESSAGES)
    ///     .shard_scheme(scheme)
    ///     .build()
    ///     .await?;
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

    /// Bring down the cluster, stopping all of the shards that it's managing.
    pub fn down(&self) {
        for shard in self.0.shards.lock().expect("shards poisoned").values() {
            shard.shutdown();
        }
    }

    /// Bring down the cluster in a resumable way and returns all info needed
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
            .lock()
            .expect("shards poisoned")
            .values()
            .map(Shard::shutdown_resumable)
            .filter_map(|(id, session)| session.map(|s| (id, s)))
            .collect()
    }

    /// Return a Shard by its ID.
    pub fn shard(&self, id: u64) -> Option<Shard> {
        self.0
            .shards
            .lock()
            .expect("shards poisoned")
            .get(&id)
            .cloned()
    }

    /// Return a list of all the shards.
    pub fn shards(&self) -> Vec<Shard> {
        self.0
            .shards
            .lock()
            .expect("shards poisned")
            .values()
            .cloned()
            .collect()
    }

    /// Return information about all shards.
    ///
    /// # Examples
    ///
    /// After waiting a minute, print the ID, latency, and stage of each shard:
    ///
    /// ```no_run
    /// use twilight_gateway::{Cluster, Intents};
    /// use std::{env, time::Duration};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let cluster = Cluster::new(env::var("DISCORD_TOKEN")?, Intents::empty()).await?;
    /// cluster.up().await;
    ///
    /// tokio::time::sleep(Duration::from_secs(60)).await;
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
            .lock()
            .expect("shards poisoned")
            .iter()
            .filter_map(|(id, shard)| shard.info().ok().map(|info| (*id, info)))
            .collect()
    }

    /// Send a command to the specified shard.
    ///
    /// # Errors
    ///
    /// Returns a [`ClusterCommandErrorType::Sending`] error type if the shard
    /// exists, but sending it failed.
    ///
    /// Returns a [`ClusterCommandErrorType::ShardNonexistent`] error type if
    /// the provided shard ID does not exist in the cluster.
    pub async fn command(
        &self,
        id: u64,
        value: &impl serde::Serialize,
    ) -> Result<(), ClusterCommandError> {
        let shard = self.shard(id).ok_or(ClusterCommandError {
            kind: ClusterCommandErrorType::ShardNonexistent { id },
            source: None,
        })?;

        shard
            .command(value)
            .await
            .map_err(|source| ClusterCommandError {
                kind: ClusterCommandErrorType::Sending,
                source: Some(Box::new(source)),
            })
    }

    /// Send a raw command to the specified shard.
    ///
    /// # Errors
    ///
    /// Returns a [`ClusterCommandErrorType::Sending`] error type if the shard
    /// exists, but sending it failed.
    ///
    /// Returns a [`ClusterCommandErrorType::ShardNonexistent`] error type if
    /// the provided shard ID does not exist in the cluster.
    #[deprecated(note = "Use `send` which is more versatile", since = "0.3.0")]
    pub async fn command_raw(&self, id: u64, value: Vec<u8>) -> Result<(), ClusterCommandError> {
        self.send(id, Message::Binary(value))
            .await
            .map_err(ClusterCommandError::from_send)
    }

    /// Send a raw websocket message.
    ///
    /// # Examples
    ///
    /// Send a restart close to shard ID 7:
    ///
    /// ```no_run
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use std::env;
    /// use twilight_gateway::{
    ///     cluster::Cluster,
    ///     shard::raw_message::{CloseFrame, Message},
    ///     Intents,
    /// };
    ///
    /// let token = env::var("DISCORD_TOKEN")?;
    /// let cluster = Cluster::new(token, Intents::GUILDS).await?;
    /// cluster.up().await;
    ///
    /// // some time later..
    /// let close = CloseFrame::from((1012, ""));
    /// let message = Message::Close(Some(close));
    /// cluster.send(7, message).await?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ClusterCommandErrorType::Sending`] error type if the shard
    /// exists, but sending the close code failed.
    ///
    /// Returns a [`ClusterCommandErrorType::ShardNonexistent`] error type if
    /// the provided shard ID does not exist in the cluster.
    ///
    /// [`SessionInactiveError`]: struct.SessionInactiveError.html
    pub async fn send(&self, id: u64, message: Message) -> Result<(), ClusterSendError> {
        let shard = self.shard(id).ok_or(ClusterSendError {
            kind: ClusterSendErrorType::ShardNonexistent { id },
            source: None,
        })?;

        shard
            .send(message)
            .await
            .map_err(|source| ClusterSendError {
                kind: ClusterSendErrorType::Sending,
                source: Some(Box::new(source)),
            })
    }

    /// Return a stream of events from all shards managed by this Cluster.
    ///
    /// Each item in the stream contains both the shard's ID and the event
    /// itself.
    ///
    /// **Note** that we *highly* recommend specifying only the events that you
    /// need via [`some_events`], especially if performance is a concern. This
    /// will ensure that events you don't care about aren't deserialized from
    /// received websocket messages. Gateway intents only allow specifying
    /// categories of events, but using [`some_events`] will filter it further
    /// on the client side.
    ///
    /// [`some_events`]: Self::some_events
    pub fn events(&self) -> impl Stream<Item = (u64, Event)> {
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
    /// use twilight_gateway::{Cluster, EventTypeFlags, Event, Intents};
    /// use futures::StreamExt;
    /// use std::env;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    /// let cluster = Cluster::new(env::var("DISCORD_TOKEN")?, Intents::GUILD_MESSAGES).await?;
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
    /// [`events`]: Self::events
    pub fn some_events(&self, types: EventTypeFlags) -> impl Stream<Item = (u64, Event)> {
        let shards = self.0.shards.lock().expect("shards poisoned").clone();
        let stream = shards
            .into_iter()
            .map(|(id, shard)| shard.some_events(types).map(move |e| (id, e)));

        // Clippy recommends using bad code here.
        #[allow(clippy::from_iter_instead_of_collect)]
        SelectAll::from_iter(stream)
    }

    /// Queue a request to start a shard by ID and starts it once the queue
    /// accepts the request.
    ///
    /// Accepts weak references to the queue and map of shards, because by the
    /// time the future is polled the cluster may have already dropped, bringing
    /// down the queue and shards with it.
    async fn start(cluster: Arc<ClusterRef>, shard_id: u64) -> Option<Shard> {
        let mut shard = cluster
            .shards
            .lock()
            .expect("shards poisoned")
            .get(&shard_id)?
            .clone();

        shard.start().await.ok()?;

        Some(shard)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Cluster, ClusterCommandError, ClusterCommandErrorType, ClusterSendError,
        ClusterSendErrorType, ClusterStartError, ClusterStartErrorType,
    };
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{error::Error, fmt::Debug};

    assert_impl_all!(ClusterCommandErrorType: Debug, Send, Sync);
    assert_fields!(ClusterCommandErrorType::ShardNonexistent: id);
    assert_impl_all!(ClusterCommandError: Error, Send, Sync);
    assert_impl_all!(ClusterSendErrorType: Debug, Send, Sync);
    assert_fields!(ClusterSendErrorType::ShardNonexistent: id);
    assert_impl_all!(ClusterSendError: Error, Send, Sync);
    assert_impl_all!(ClusterStartErrorType: Debug, Send, Sync);
    assert_impl_all!(ClusterStartError: Error, Send, Sync);
    assert_impl_all!(Cluster: Clone, Debug, Send, Sync);
}
