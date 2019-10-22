use super::{Error, Result};
use crate::queue::{LocalQueue, Queue};
use dawn_http::Client as HttpClient;
use dawn_model::gateway::payload::update_status::UpdateStatusInfo;

/// The configuration used by the shard to identify with the gateway and
/// operate.
///
/// Use [`Config::builder`] to start creating a configuration.
///
/// [`Config::builder`]: #method.builder
#[derive(Debug)]
pub struct Config {
    guild_subscriptions: bool,
    http_client: HttpClient,
    large_threshold: u64,
    presence: Option<UpdateStatusInfo>,
    pub(crate) queue: Box<dyn Queue + Send + Sync>,
    shard: [u64; 2],
    token: String,
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

    /// Returns whether to subscribe to guilds' presence updates and typing
    /// events.
    pub fn guild_subscriptions(&self) -> bool {
        self.guild_subscriptions
    }

    /// Returns the `dawn_http` client to be used by the shard.
    pub fn http_client(&self) -> &HttpClient {
        &self.http_client
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

impl From<ConfigBuilder> for Config {
    fn from(builder: ConfigBuilder) -> Self {
        builder.build()
    }
}

impl<T: Into<String>> From<T> for Config {
    fn from(token: T) -> Self {
        ConfigBuilder::new(token).build()
    }
}

/// Builder to create a [`Config`].
///
/// [`Config`]: struct.Config.html
#[derive(Debug)]
pub struct ConfigBuilder(Config);

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

        Self(Config {
            guild_subscriptions: true,
            http_client: HttpClient::new(token.clone()),
            large_threshold: 250,
            presence: None,
            queue: Box::new(LocalQueue::new()),
            shard: [0, 1],
            token,
        })
    }

    /// Consumes the builder and returns the final configuration.
    pub fn build(self) -> Config {
        self.0
    }

    /// Whether to subscribe to guilds' presence updates and typing events.
    ///
    /// Many bots don't need these, so it would be beneficial to turn this off.
    /// Presence updates alone account for about 85% of all event traffic.
    ///
    /// The default value is `true`.
    pub fn guild_subscriptions(&mut self, guild_subscriptions: bool) -> &mut Self {
        self.0.guild_subscriptions = guild_subscriptions;

        self
    }

    /// The HTTP client to be used by the shard for getting gateway information.
    pub fn http_client(&mut self, http_client: HttpClient) -> &mut Self {
        self.0.http_client = http_client;

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
    /// Returns [`Error::LargeThresholdInvalid`] if the provided value was below
    /// 50 or above 250.
    ///
    /// [`Error::LargeThresholdInvalid`]: ../error/enum.Error.html#variant.LargeThresholdInvalid
    pub fn large_threshold(&mut self, large_threshold: u64) -> Result<&mut Self> {
        if large_threshold > 250 || large_threshold < 50 {
            return Err(Error::LargeThresholdInvalid {
                value: large_threshold,
            });
        }

        self.0.large_threshold = large_threshold;

        Ok(self)
    }

    /// Sets the presence to use automatically when starting a new session.
    ///
    /// The default is none, which defaults to strictly being "online" with no
    /// special qualities.
    pub fn presence(&mut self, presence: UpdateStatusInfo) -> &mut Self {
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
    /// all shards when ran by a `Cluster`.
    pub fn queue(&mut self, queue: impl Into<Box<dyn Queue + Send + Sync>>) -> &mut Self {
        self.0.queue = queue.into();

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
    /// use dawn_gateway::shard::Config;
    /// use std::env;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut config = Config::builder(env::var("DISCORD_TOKEN")?);
    /// config.shard(18, 19)?;
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::IdTooLarge`] if the shard ID to connect as is larger
    /// than the total.
    ///
    /// [`Error::IdTooLarge`]: ../error/enum.Error.html#variant.IdTooLarge
    pub fn shard(&mut self, shard_id: u64, shard_total: u64) -> Result<&mut Self> {
        if shard_id >= shard_total {
            return Err(Error::IdTooLarge {
                id: shard_id,
                total: shard_total,
            });
        }

        self.0.shard = [shard_id, shard_total];

        Ok(self)
    }
}

impl<T: Into<String>> From<T> for ConfigBuilder {
    fn from(token: T) -> Self {
        Self::new(token.into())
    }
}
