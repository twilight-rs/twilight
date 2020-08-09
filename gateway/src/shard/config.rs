use crate::queue::{LocalQueue, Queue};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    sync::Arc,
};
use twilight_http::Client as HttpClient;
use twilight_model::gateway::{payload::update_status::UpdateStatusInfo, GatewayIntents};

/// Large threshold configuration is invalid.
///
/// Returned by [`ShardConfigBuilder::large_threshold`].
///
/// [`ShardConfigBuilder::large_threshold`]: struct.ShardConfigBuilder.html#method.large_threshold
#[derive(Debug)]
pub enum LargeThresholdError {
    /// Provided large threshold value is too few in number.
    TooFew {
        /// Provided value.
        value: u64,
    },
    /// Provided large threshold value is too many in number.
    TooMany {
        /// Provided value.
        value: u64,
    },
}

impl Display for LargeThresholdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::TooFew { .. } => f.write_str("provided large threshold value is fewer than 50"),
            Self::TooMany { .. } => f.write_str("provided large threshold value is more than 250"),
        }
    }
}

impl Error for LargeThresholdError {}

/// Shard ID configuration is invalid.
///
/// Returned by [`ShardConfigBuilder::shard`].
///
/// [`ShardConfigBuilder::shard`]: struct.ShardConfigBuilder.html#method.shard
#[derive(Debug)]
pub enum ShardIdError {
    /// Provided shard ID is higher than provided total shard count.
    IdTooLarge {
        /// Shard ID.
        id: u64,
        /// Total shard count.
        total: u64,
    },
}

impl Display for ShardIdError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::IdTooLarge { id, total } => f.write_fmt(format_args!(
                "provided shard ID {} is larger than the total {}",
                id, total,
            )),
        }
    }
}

impl Error for ShardIdError {}

/// The configuration used by the shard to identify with the gateway and
/// operate.
///
/// Use [`ShardConfig::builder`] to start creating a configuration.
///
/// [`ShardConfig::builder`]: #method.builder
#[derive(Clone, Debug)]
pub struct ShardConfig {
    http_client: HttpClient,
    intents: Option<GatewayIntents>,
    large_threshold: u64,
    presence: Option<UpdateStatusInfo>,
    pub(crate) queue: Arc<Box<dyn Queue>>,
    pub(crate) shard: [u64; 2],
    token: String,
    pub(crate) session_id: Option<String>,
    pub(crate) sequence: Option<u64>,
}

impl ShardConfig {
    /// Creates a new builder to create a config.
    ///
    /// This is equivalent to calling [`ShardConfigBuilder::new`] directly.
    ///
    /// [`ShardConfigBuilder::new`]: struct.ShardConfigBuilder.html#method.new
    pub fn builder(token: impl Into<String>) -> ShardConfigBuilder {
        ShardConfigBuilder::new(token)
    }

    /// Returns the `twilight_http` client to be used by the shard.
    pub fn http_client(&self) -> &HttpClient {
        &self.http_client
    }

    /// Returns the intents that the gateway is using.
    pub fn intents(&self) -> Option<&GatewayIntents> {
        self.intents.as_ref()
    }

    /// The maximum threshold at which point the gateway will stop sending
    /// a guild's member list.
    pub fn large_threshold(&self) -> u64 {
        self.large_threshold
    }

    /// The presence to set when connecting to the gateway.
    ///
    /// This will be the bot's presence. For example, setting an activity in
    /// the presence will show the activity in the bot's status.
    pub fn presence(&self) -> Option<&UpdateStatusInfo> {
        self.presence.as_ref()
    }

    /// The shard's ID and the total number of shards used by the bot.
    pub fn shard(&self) -> [u64; 2] {
        self.shard
    }

    /// The token used to authenticate with when identifying with the gateway.
    pub fn token(&self) -> &str {
        &self.token
    }
}

impl From<ShardConfigBuilder> for ShardConfig {
    fn from(builder: ShardConfigBuilder) -> Self {
        builder.build()
    }
}

impl<T: Into<String>> From<T> for ShardConfig {
    fn from(token: T) -> Self {
        ShardConfigBuilder::new(token).build()
    }
}

/// Builder to create a [`ShardConfig`].
///
/// [`ShardConfig`]: struct.ShardConfig.html
#[derive(Debug)]
pub struct ShardConfigBuilder(ShardConfig);

impl ShardConfigBuilder {
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

        Self(ShardConfig {
            http_client: HttpClient::new(token.clone()),
            intents: None,
            large_threshold: 250,
            presence: None,
            queue: Arc::new(Box::new(LocalQueue::new())),
            shard: [0, 1],
            token,
            session_id: None,
            sequence: None,
        })
    }

    /// Consumes the builder and returns the final configuration.
    pub fn build(self) -> ShardConfig {
        self.0
    }

    /// The HTTP client to be used by the shard for getting gateway information.
    pub fn http_client(mut self, http_client: HttpClient) -> Self {
        self.0.http_client = http_client;

        self
    }

    /// Sets the gateway intents.
    pub fn intents(mut self, intents: Option<GatewayIntents>) -> Self {
        self.0.intents = intents;

        self
    }

    /// The maximum number of members in a guild to load the member list.
    ///
    /// If you pass `200`, then if there are 250 members in a guild the member
    /// list won't be sent. If there are 150 members, then the list **will** be
    /// sent.
    ///
    /// The default value is `250`. The minimum value is `50` and the maximum is
    /// `250`.
    ///
    /// # Errors
    ///
    /// Returns [`LargeThresholdError::TooFew`] if the provided value is below
    /// 50.
    ///
    /// Returns [`LargeThresholdError::TooMany`] if the provided value is above
    /// 250.
    ///
    /// [`LargeThresholdError::TooFew`]: enum.LargeThresholdError.html#variant.TooFew
    /// [`LargeThresholdError::TooMany`]: enum.LargeThresholdError.html#variant.TooMany
    pub fn large_threshold(mut self, large_threshold: u64) -> Result<Self, LargeThresholdError> {
        match large_threshold {
            0..=49 => {
                return Err(LargeThresholdError::TooFew {
                    value: large_threshold,
                })
            }
            50..=250 => {}
            251..=u64::MAX => {
                return Err(LargeThresholdError::TooMany {
                    value: large_threshold,
                })
            }
        }

        self.0.large_threshold = large_threshold;

        Ok(self)
    }

    /// Sets the presence to use automatically when starting a new session.
    ///
    /// The default is none, which defaults to strictly being "online" with no
    /// special qualities.
    pub fn presence(mut self, presence: UpdateStatusInfo) -> Self {
        self.0.presence.replace(presence);

        self
    }

    /// Sets the queue to use for queueing shard connections.
    ///
    /// You probably don't need to set this yourself, because the [`Cluster`]
    /// manages that for you. You only need to set this if you're implementing
    /// your only cluster-like support.
    ///
    /// The default value is a queue used only by this shard, or a queue used by
    /// all shards when ran by a [`Cluster`].
    ///
    /// [`Cluster`]: ../../cluster/struct.Cluster.html
    pub fn queue(mut self, queue: Arc<Box<dyn Queue>>) -> Self {
        self.0.queue = queue;

        self
    }

    /// Sets the shard ID to connect as, and the total number of shards used by
    /// the bot.
    ///
    /// The shard ID is 0-indexed, while the total is 1-indexed.
    ///
    /// The default value is a shard ID of 0 and a shard total of 1, which is
    /// good for smaller bots.
    ///
    /// **Note**: If your bot is in over 100'000 guilds then `shard_total`
    /// *should probably* be a multiple of 16 if you're in the "Large Bot
    /// Sharding" program.
    ///
    /// # Errors
    ///
    /// If you have 19 shards, then your last shard will have an ID of 18 out of
    /// a total of 19 shards:
    ///
    /// ```no_run
    /// use twilight_gateway::shard::ShardConfig;
    /// use std::env;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut config = ShardConfig::builder(env::var("DISCORD_TOKEN")?);
    /// config.shard(18, 19)?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ShardIdError::IdTooLarge`] if the shard ID to connect as is
    /// larger than the total.
    ///
    /// [`ShardIdError::IdTooLarge`]: enum.ShardIdError.html#variant.IdTooLarge
    pub fn shard(mut self, shard_id: u64, shard_total: u64) -> Result<Self, ShardIdError> {
        if shard_id >= shard_total {
            return Err(ShardIdError::IdTooLarge {
                id: shard_id,
                total: shard_total,
            });
        }

        self.0.shard = [shard_id, shard_total];

        Ok(self)
    }
}

impl<T: Into<String>> From<T> for ShardConfigBuilder {
    fn from(token: T) -> Self {
        Self::new(token.into())
    }
}
