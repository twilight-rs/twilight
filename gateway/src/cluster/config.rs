use super::error::{Error, Result};
use crate::{
    queue::{LocalQueue, Queue},
    shard::config::{Config as ShardConfig, ConfigBuilder as ShardConfigBuilder},
};
use twilight_http::Client;
use twilight_model::gateway::{payload::update_status::UpdateStatusInfo, GatewayIntents};
use std::{
    convert::TryFrom,
    ops::{Bound, RangeBounds},
    sync::Arc,
};

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
    /// use twilight_gateway::cluster::config::ShardScheme;
    /// use std::convert::TryFrom;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let _range = ShardScheme::try_from((0..24, 50));
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
    type Error = Error;

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
            return Err(Error::IdTooLarge {
                end,
                start,
                total,
            });
        }

        Ok(Self::Range {
            from: start,
            to: end,
            total,
        })
    }
}

/// Built configuration to be used for creating a [`Cluster`].
///
/// [`Cluster`]: ../struct.Cluster.html
#[derive(Debug)]
pub struct Config {
    http_client: Client,
    shard_config: ShardConfig,
    shard_scheme: ShardScheme,
    queue: Arc<Box<dyn Queue>>,
}

impl Config {
    /// Creates a new builder to create a config.
    ///
    /// This is equivalent to calling [`ConfigBuilder::new`] directly.
    ///
    /// [`ConfigBuilder::new`]: struct.ConfigBuilder.html#method.new
    pub fn builder(token: impl Into<String>) -> ConfigBuilder {
        ConfigBuilder::new(token)
    }

    /// Returns the `twilight_http` client used by the cluster and shards to get the
    /// gateway information.
    ///
    /// Refer to [`ConfigBuilder::http_client`] for the default value.
    ///
    /// [`ConfigBuilder::http_client`]: struct.ConfigBuilder.html#method.http_client
    pub fn http_client(&self) -> &Client {
        &self.http_client
    }

    /// Returns the configuration used to create shards.
    ///
    /// Refer to [`shard::config::ConfigBuilder`]'s methods for the default values.
    ///
    /// [`shard::config::ConfigBuilder`]: ../../shard/config/struct.ConfigBuilder.html#methods
    pub fn shard_config(&self) -> &ShardConfig {
        &self.shard_config
    }

    /// Returns the shard scheme used to start shards.
    ///
    /// Refer to [`ConfigBuilder::shard_scheme`] for the default value.
    ///
    /// [`ConfigBuilder::shard_scheme`]: struct.ConfigBuilder.html#method.shard_scheme
    pub fn shard_scheme(&self) -> ShardScheme {
        self.shard_scheme
    }

    pub fn queue(&self) -> &Arc<Box<dyn Queue>> {
        &self.queue
    }
}

impl From<ConfigBuilder> for Config {
    fn from(builder: ConfigBuilder) -> Self {
        builder.build()
    }
}

impl<T: Into<String>> From<T> for Config {
    fn from(token: T) -> Self {
        Self::builder(token).build()
    }
}

/// Builder to create a [`Config`].
///
/// [`Config`]: struct.Config.html
// Yeah, I mean, we *could* deref to the `ShardConfigBuilder`, but it's not
// clear.
#[derive(Debug)]
pub struct ConfigBuilder(Config, ShardConfigBuilder);

impl ConfigBuilder {
    /// Creates a new builder with default configuration values.
    ///
    /// Refer to each method to learn their default values.
    pub fn new(token: impl Into<String>) -> Self {
        Self::_new(token.into())
    }

    fn _new(mut token: String) -> Self {
        if !token.starts_with("Bot ") {
            token.insert_str(0, "Bot ");
        }

        Self(
            Config {
                http_client: Client::new(token.clone()),
                shard_config: ShardConfig::from(token.clone()),
                shard_scheme: ShardScheme::Auto,
                queue: Arc::new(Box::new(LocalQueue::new())),
            },
            ShardConfigBuilder::new(token),
        )
    }

    /// Consumes the builder and returns the final configuration.
    pub fn build(mut self) -> Config {
        self.0.shard_config = self.1.build();

        self.0
    }

    /// Whether to subscribe shards to "guild subscriptions", which are the
    /// presence update and typing start events.
    ///
    /// Refer to the shard's [`ConfigBuilder::guild_subscriptions`] for more
    /// information.
    ///
    /// [`ConfigBuilder::guild_subscriptions`]: ../../shard/config/struct.ConfigBuilder.html#method.guild_subscriptions
    pub fn guild_subscriptions(mut self, guild_subscriptions: bool) -> Self {
        self.1 = self.1.guild_subscriptions(guild_subscriptions);
        self
    }

    /// Sets the `twilight_http` Client used by the cluster and the shards it
    /// manages.
    ///
    /// This is needed so that the cluster and shards can retrieve gateway
    /// information.
    ///
    /// By default, the default client is used.
    pub fn http_client(mut self, http_client: Client) -> Self {
        self.0.http_client = http_client.clone();
        self.1 = self.1.http_client(http_client);

        self
    }

    /// Sets the "large threshold" of shards.
    ///
    /// Refer to the shard's [`ConfigBuilder::large_threshold`] for more
    /// information.
    ///
    /// # Errors
    ///
    /// Returns [`ShardError::LargeThresholdInvalid`] if the value was not in
    /// the accepted range.
    ///
    /// [`ConfigBuilder::large_threshold`]: ../../shard/config/struct.ConfigBuilder.html#method.large_threshold
    /// [`ShardError::LargeThresholdInvalid`]: ../../shard/error/enum.Error.html#variant.LargeThresholdInvalid
    pub fn large_threshold(mut self, large_threshold: u64) -> Result<Self> {
        self.1 = self.1.large_threshold(large_threshold).map_err(|source| {
            Error::LargeThresholdInvalid {
                source,
            }
        })?;

        Ok(self)
    }

    /// Sets the presence to use when identifying with the gateway.
    ///
    /// Refer to the shard's [`ConfigBuilder::presence`] for more information.
    ///
    /// [`ConfigBuilder::presence`]: ../../shard/config/struct.ConfigBuilder.html#method.presence
    pub fn presence(mut self, presence: UpdateStatusInfo) -> Self {
        self.1 = self.1.presence(presence);

        self
    }

    /// Sets the intents to use when identifying with the gateway.
    ///
    /// Refer to the shard's [`ConfigBuilder::intents`] for more information.
    ///
    /// [`ConfigBuilder::intents`]: ../../shard/config/struct.ConfigBuilder.html#method.intents
    pub fn intents(mut self, intents: Option<GatewayIntents>) -> Self {
        self.1.intents(intents);

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
    /// use twilight_gateway::cluster::config::{Config, ShardScheme};
    /// use std::{
    ///     convert::TryFrom,
    ///     env,
    /// };
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut config = Config::builder(env::var("DISCORD_TOKEN")?);
    ///
    /// let scheme = ShardScheme::try_from((0..=9, 20))?;
    /// config.shard_scheme(scheme);
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
        self.1 = self.1.queue(Arc::clone(&queue));
        self.0.queue = queue;

        self
    }
}

impl<T: Into<String>> From<T> for ConfigBuilder {
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
