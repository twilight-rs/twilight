use super::{
    config::Config as ClusterConfig,
    r#impl::{Cluster, ClusterStartError},
};
use crate::{
    queue::{LocalQueue, Queue},
    shard::{LargeThresholdError, ResumeSession, ShardBuilder},
};
use std::{
    collections::HashMap,
    convert::TryFrom,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Bound, RangeBounds},
    sync::Arc,
};
use twilight_http::Client;
use twilight_model::gateway::{payload::update_status::UpdateStatusInfo, GatewayIntents};

/// Starting a cluster failed.
#[derive(Debug)]
pub enum ShardSchemeRangeError {
    /// The start of the shard range was greater than the end or total.
    IdTooLarge {
        /// The last shard in the range to manage.
        end: u64,
        /// The first shard in the range to manage.
        start: u64,
        /// The total number of shards used by the bot.
        total: u64,
    },
}

impl Display for ShardSchemeRangeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::IdTooLarge { end, start, total } => f.write_fmt(format_args!(
                "The shard ID range {}-{}/{} is larger than the total",
                start, end, total
            )),
        }
    }
}

impl Error for ShardSchemeRangeError {}

/// The method of sharding to use.
///
/// By default this is [`Auto`].
///
/// [`Auto`]: #variant.Auto
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub enum ShardScheme {
    /// Specifies to retrieve the amount of shards recommended by Discord and
    /// then start all of them.
    ///
    /// For example, if Discord recommends 10 shards, then all 10 shards will be
    /// started.
    Auto,
    /// Specifies to start a range of shards.
    ///
    /// # Examples
    ///
    /// For example, if your bot uses 50 shards, then you might specify to start
    /// shards 0 through 24:
    ///
    /// ```
    /// use twilight_gateway::cluster::ShardScheme;
    /// use std::convert::TryFrom;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let range = ShardScheme::try_from((0..24, 50))?;
    /// # Ok(()) }
    /// ```
    Range {
        /// The first shard ID to spawn.
        from: u64,
        /// The last shard ID to spawn.
        ///
        /// This doesn't necessarily have to be up to the `total`.
        to: u64,
        /// The total number of shards used by the bot.
        total: u64,
    },
}

impl Default for ShardScheme {
    fn default() -> Self {
        Self::Auto
    }
}

impl<T: RangeBounds<u64>> TryFrom<(T, u64)> for ShardScheme {
    type Error = ShardSchemeRangeError;

    fn try_from((range, total): (T, u64)) -> Result<Self, Self::Error> {
        let start = match range.start_bound() {
            Bound::Excluded(num) => *num - 1,
            Bound::Included(num) => *num,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Excluded(num) => *num - 1,
            Bound::Included(num) => *num,
            Bound::Unbounded => total - 1,
        };

        if start > end {
            return Err(ShardSchemeRangeError::IdTooLarge { end, start, total });
        }

        Ok(Self::Range {
            from: start,
            to: end,
            total,
        })
    }
}

/// Builder to configure and construct a [`Cluster`].
///
/// [`Cluster`]: ./struct.Cluster.html
#[derive(Debug)]
pub struct ClusterBuilder(ClusterConfig, ShardBuilder);

impl ClusterBuilder {
    /// Create a new builder to construct and configure a cluster.
    pub fn new(token: impl Into<String>) -> Self {
        Self::_new(token.into())
    }

    fn _new(mut token: String) -> Self {
        if !token.starts_with("Bot ") {
            token.insert_str(0, "Bot ");
        }

        Self(
            ClusterConfig {
                http_client: Client::new(token.clone()),
                shard_config: ShardBuilder::new(token.clone()).0,
                shard_scheme: ShardScheme::Auto,
                queue: Arc::new(Box::new(LocalQueue::new())),
                resume_sessions: HashMap::new(),
            },
            ShardBuilder::new(token),
        )
    }

    /// Consume the builder and create the cluster.
    ///
    /// # Errors
    ///
    /// Returns [`ClusterStartError::RetrievingGatewayInfo`] if there was an HTTP error Retrieving
    /// the gateway information.
    ///
    /// [`ClusterStartError::RetrievingGatewayInfo`]: enum.ClusterStartError.html#variant.RetrievingGatewayInfo
    pub async fn build(mut self) -> Result<Cluster, ClusterStartError> {
        self.0.shard_config = (self.1).0;

        Cluster::new_with_config(self.0).await
    }

    /// Sets the `twilight_http` Client used by the cluster and the shards it
    /// manages.
    ///
    /// This is needed so that the cluster and shards can retrieve gateway
    /// information.
    ///
    /// By default, the default client is used.
    pub fn http_client(mut self, http_client: Client) -> Self {
        self.1 = self.1.http_client(http_client);

        self
    }

    /// Sets the "large threshold" of shards.
    ///
    /// Refer to the shard's [`ShardBuilder::large_threshold`] for more
    /// information.
    ///
    /// # Errors
    ///
    /// Returns [`LargeThresholdError::TooFew`] if the provided value is below
    /// 50.
    ///
    /// Returns [`LargeThresholdError::TooMany`] if the provided value is above
    /// 250.
    ///
    /// [`LargeThresholdError::TooFew`]: ../../shard/config/enum.LargeThresholdError.html#variant.TooFew
    /// [`LargeThresholdError::TooMany`]: ../../shard/config/enum.LargeThresholdError.html#variant.TooMany
    /// [`ShardBuilder::large_treshold`]: ../shard/ShardBuilder.html#method.large_threshold
    pub fn large_threshold(mut self, large_threshold: u64) -> Result<Self, LargeThresholdError> {
        self.1 = self.1.large_threshold(large_threshold)?;

        Ok(self)
    }

    /// Sets the presence to use when identifying with the gateway.
    ///
    /// Refer to the shard's [`ShardConfigBuilder::presence`] for more information.
    ///
    /// [`ShardConfigBuilder::presence`]: ../../shard/config/struct.ShardConfigBuilder.html#method.presence
    pub fn presence(mut self, presence: UpdateStatusInfo) -> Self {
        self.1 = self.1.presence(presence);

        self
    }

    /// Sets the intents to use when identifying with the gateway.
    ///
    /// Refer to the shard's [`ShardConfigBuilder::intents`] for more information.
    ///
    /// [`ShardConfigBuilder::intents`]: ../../shard/config/struct.ShardConfigBuilder.html#method.intents
    pub fn intents(mut self, intents: Option<GatewayIntents>) -> Self {
        self.1 = self.1.intents(intents);

        self
    }

    /// Sets the scheme to use for shard managing.
    ///
    /// For example, [`ShardScheme::Auto`] means that the cluster will
    /// automatically manage all of the shards that Discord recommends you use.
    /// [`ShardScheme::Range`] means that it will manage a range of shards, but
    /// not necessarily all of the shards that your bot uses.
    ///
    /// The default value is [`ShardScheme::Auto`]. For most setups this is an
    /// acceptable default.
    ///
    /// # Examples
    ///
    /// Configure a cluster to manage shards 0-9 out of 20 shards total:
    ///
    /// ```no_run
    /// use twilight_gateway::cluster::{Cluster, ShardScheme};
    /// use std::{
    ///     convert::TryFrom,
    ///     env,
    /// };
    ///
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let token = env::var("DISCORD_TOKEN")?;
    /// let scheme = ShardScheme::try_from((0..=9, 20))?;
    ///
    /// let cluster = Cluster::builder(token).shard_scheme(scheme).build().await?;
    /// # Ok(()) }
    /// ```
    ///
    /// [`ShardScheme::Auto`]: enum.ShardScheme.html#variant.Auto
    /// [`ShardScheme::Range`]: enum.ShardScheme.html#variant.Range
    pub fn shard_scheme(mut self, scheme: ShardScheme) -> Self {
        self.0.shard_scheme = scheme;

        self
    }

    /// Sets the queue to use for queueing shard connections.
    ///
    /// This can be used when having advanced setups with multiple
    /// binaries connecting at the same time.
    pub fn queue(mut self, queue: Arc<Box<dyn Queue>>) -> Self {
        self.0.queue = Arc::clone(&queue);
        self.1 = self.1.queue(queue);

        self
    }

    /// Sets the session information to resume shards with
    ///
    /// This requires having recovered the resume data when shutting down the cluster
    /// NOTE: this does not guarantee these shards will be able to resume. If their sessions are invalid they will have to re-identify as normal
    pub fn resume_sessions(mut self, resume_sessions: HashMap<u64, ResumeSession>) -> Self {
        self.0.resume_sessions = resume_sessions;
        self
    }
}

impl<T: Into<String>> From<T> for ClusterBuilder {
    fn from(token: T) -> Self {
        Self::new(token)
    }
}

#[cfg(test)]
mod tests {
    use super::ShardScheme;
    use std::{convert::TryFrom, error::Error};

    #[test]
    fn test_shard_scheme() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            ShardScheme::Range {
                from: 0,
                to: 9,
                total: 10,
            },
            ShardScheme::try_from((0..=9, 10))?
        );

        Ok(())
    }
}
