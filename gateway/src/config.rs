//! Customizable configuration for shards.

use super::session::Session;
use crate::{tls::TlsContainer, EventTypeFlags};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    sync::Arc,
};
use twilight_gateway_queue::{LocalQueue, Queue};
use twilight_model::gateway::{
    payload::outgoing::{identify::IdentifyProperties, update_presence::UpdatePresencePayload},
    Intents,
};

/// Maximum value of an acceptable [large threshold].
///
/// [large threshold]: ConfigBuilder::large_threshold
pub const LARGE_THRESHOLD_MAXIMUM: u64 = 250;

/// Minimum value of an acceptable [large threshold].
///
/// [large threshold]: ConfigBuilder::large_threshold
pub const LARGE_THRESHOLD_MINIMUM: u64 = 50;

/// Identifier of a [shard], including the shard's ID and the total number of
/// shards in use by the bot.
///
/// [shard]: super::Shard
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ShardId {
    /// Current ID of the shard, 0-indexed.
    current: u64,
    /// Total number of shards used by the bot, 1-indexed.
    total: u64,
}

impl ShardId {
    /// ID of a bot that has only one shard.
    ///
    /// This should *only* be used by small bots in under one or two thousand
    /// guilds.
    pub const ONE: ShardId = ShardId::new(0, 1);

    /// Create a new identifier for a shard.
    ///
    /// The current shard is 0-indexed while the total number of shards is
    /// 1-indexed. This means that a current shard of 7 with a total of 8 is
    /// valid, while a current shard value of 8 out of 8 total shards is
    /// invalid.
    ///
    /// # Examples
    ///
    /// Create a new shard with a current index of 13 out of 24 shards:
    ///
    /// ```
    /// use twilight_gateway::config::ShardId;
    ///
    /// let id = ShardId::new(13, 24);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the current shard is greater than or equal to the total number
    /// of shards, or if the total number of shards is zero.
    pub const fn new(current: u64, total: u64) -> Self {
        assert!(total > 0, "total must be greater than zero");
        assert!(
            current < total,
            "current shard (0-indexed) must be less than total (1-indexed)",
        );

        Self { current, total }
    }

    /// Create a new identifier for a shard if the shard indexes are valid.
    ///
    /// The current shard is 0-indexed while the total number of shards is
    /// 1-indexed. This means that a current shard of 7 with a total of 8 is
    /// valid, while a current shard value of 8 out of 8 total shards is
    /// invalid.
    pub const fn new_checked(current: u64, total: u64) -> Option<Self> {
        let is_total_nonzero = total > 0;
        let is_current_valid = current < total;

        if is_total_nonzero && is_current_valid {
            Some(Self { current, total })
        } else {
            None
        }
    }

    /// ID of the shard, 0-indexed.
    pub const fn current(self) -> u64 {
        self.current
    }

    /// Total number of shards, 1-indexed.
    pub const fn total(self) -> u64 {
        self.total
    }
}

/// Display the shard ID.
///
/// Formats as `shard {current}/{total}`.
impl Display for ShardId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("shard ")?;
        Display::fmt(&self.current, f)?;
        f.write_str("/")?;

        Display::fmt(&self.total, f)
    }
}

/// Configuration used by the shard to identify with the gateway and operate.
///
/// Use [`Config::builder`] to start configuring a shard.
#[derive(Clone, Debug)]
pub struct Config {
    /// Event type flags.
    event_types: EventTypeFlags,
    /// URL used to connect to the gateway.
    gateway_url: Option<Box<str>>,
    /// Identification properties the shard will use.
    identify_properties: Option<IdentifyProperties>,
    /// Intents that the shard requests when identifying with the gateway.
    intents: Intents,
    /// When the gateway will stop sending a guild's member list in
    /// Guild Create events.
    large_threshold: u64,
    /// Presence to set when identifying with the gateway.
    presence: Option<UpdatePresencePayload>,
    /// Whether [outgoing message] ratelimiting is enabled.
    ///
    /// [outgoing message]: crate::Shard::send
    ratelimit_messages: bool,
    /// Queue in use by the shard.
    queue: Arc<dyn Queue>,
    /// Session information to resume a shard on initialization.
    session: Option<Session>,
    /// TLS connector for Websocket connections.
    // We need this to be public so [`stream`] can re-use TLS on multiple shards
    // if unconfigured.
    tls: TlsContainer,
    /// Token used to authenticate when identifying with the gateway.
    token: Box<str>,
}

impl Config {
    /// Create a new default configuration for a shard.
    ///
    /// Shortcut for calling [`builder`][`Self::builder`] and immediately
    /// finalizing the builder.
    pub fn new(token: String, intents: Intents) -> Self {
        Self::builder(token, intents).build()
    }

    /// Create a builder to customize the configuration for a shard.
    pub fn builder(token: String, intents: Intents) -> ConfigBuilder {
        ConfigBuilder::new(token, intents)
    }

    /// Copy of the event type flags.
    pub const fn event_types(&self) -> EventTypeFlags {
        self.event_types
    }

    /// Return an immutable reference to the url used to connect to the gateway.
    pub fn gateway_url(&self) -> Option<&str> {
        self.gateway_url.as_deref()
    }

    /// Immutable reference to the identification properties the shard will use.
    pub const fn identify_properties(&self) -> Option<&IdentifyProperties> {
        self.identify_properties.as_ref()
    }

    /// Intents that the shard requests when identifying with the gateway.
    pub const fn intents(&self) -> Intents {
        self.intents
    }

    /// Maximum threshold at which point the gateway will stop sending a guild's
    /// member list in Guild Create events.
    pub const fn large_threshold(&self) -> u64 {
        self.large_threshold
    }

    /// Immutable reference to the queue in use by the shard.
    pub fn queue(&self) -> &Arc<dyn Queue> {
        &self.queue
    }

    /// Return an immutable reference to the presence to set when identifying
    /// with the gateway.
    ///
    /// This will be the bot's presence. For example, setting the online status
    /// to Do Not Disturb will show the status in the bot's presence.
    pub const fn presence(&self) -> Option<&UpdatePresencePayload> {
        self.presence.as_ref()
    }

    /// Whether [outgoing message] ratelimiting is enabled.
    ///
    /// [outgoing message]: crate::Shard::send
    pub const fn ratelimit_messages(&self) -> bool {
        self.ratelimit_messages
    }

    /// Session information to resume a shard on initialization.
    pub const fn session(&self) -> Option<&Session> {
        self.session.as_ref()
    }

    /// Immutable reference to the TLS connector in use by the shard.
    pub(crate) const fn tls(&self) -> &TlsContainer {
        &self.tls
    }

    /// Immutable reference to the token used to authenticate when identifying
    /// with the gateway.
    pub const fn token(&self) -> &str {
        &self.token
    }
}

/// Builder to customize the operation of a shard.
#[derive(Debug)]
#[must_use = "builder must be completed to be used"]
pub struct ConfigBuilder {
    /// Inner configuration being modified.
    inner: Config,
}

impl ConfigBuilder {
    /// Create a new builder to configure and construct a shard.
    ///
    /// Refer to each method to learn their default values.
    pub fn new(mut token: String, intents: Intents) -> Self {
        if !token.starts_with("Bot ") {
            token.insert_str(0, "Bot ");
        }

        Self {
            inner: Config {
                event_types: EventTypeFlags::default(),
                gateway_url: None,
                identify_properties: None,
                intents,
                large_threshold: 50,
                presence: None,
                queue: Arc::new(LocalQueue::new()),
                ratelimit_messages: true,
                session: None,
                tls: TlsContainer::new().expect("failed to build tls"),
                token: token.into_boxed_str(),
            },
        }
    }

    /// Create a new builder from an existing configuration.
    pub const fn with_config(config: Config) -> Self {
        Self { inner: config }
    }

    /// Consume the builder, constructing a shard.
    #[allow(clippy::missing_const_for_fn)]
    pub fn build(self) -> Config {
        self.inner
    }

    /// Set the event types to process.
    ///
    /// This is an optimization technique; all events not included in the
    /// provided event type flags will not be deserialized by the gateway and
    /// will be discarded.
    pub const fn event_types(mut self, event_types: EventTypeFlags) -> Self {
        self.inner.event_types = event_types;

        self
    }

    /// Set the URL used for connecting to Discord's gateway.
    pub fn gateway_url(mut self, gateway_url: Option<String>) -> Self {
        self.inner.gateway_url = gateway_url.map(String::into_boxed_str);

        self
    }

    /// Set the properties to identify with.
    ///
    /// This may be used if you want to set a different operating system, for
    /// example.
    ///
    /// # Examples
    ///
    /// Set the identify properties for a shard:
    ///
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use std::env::{self, consts::OS};
    /// use twilight_gateway::{config::Config, Intents, Shard};
    /// use twilight_model::gateway::payload::outgoing::identify::IdentifyProperties;
    ///
    /// let token = env::var("DISCORD_TOKEN")?;
    /// let properties = IdentifyProperties::new("twilight.rs", "twilight.rs", OS);
    ///
    /// let config = Config::builder(token, Intents::empty())
    ///     .identify_properties(properties)
    ///     .build();
    /// # Ok(()) }
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    pub fn identify_properties(mut self, identify_properties: IdentifyProperties) -> Self {
        self.inner.identify_properties = Some(identify_properties);

        self
    }

    /// Set the maximum number of members in a guild to load the member list.
    ///
    /// Default value is `50`. The minimum value is `50` and the maximum is
    /// `250`.
    ///
    /// # Examples
    ///
    /// If you pass `200`, then if there are 250 members in a guild the member
    /// list won't be sent. If there are 150 members, then the list *will* be
    /// sent.
    ///
    /// # Panics
    ///
    /// Panics if the provided value is below 50 or above 250.
    pub const fn large_threshold(mut self, large_threshold: u64) -> Self {
        assert!(
            large_threshold >= LARGE_THRESHOLD_MINIMUM
                && large_threshold <= LARGE_THRESHOLD_MAXIMUM,
            "large threshold isn't in the accepted range"
        );

        self.inner.large_threshold = large_threshold;

        self
    }

    /// Set the presence to use automatically when starting a new session.
    ///
    /// Default is no presence, which defaults to strictly being "online"
    /// with no special qualities.
    ///
    /// # Examples
    ///
    /// Set the bot user's presence to idle with the status "Not accepting
    /// commands":
    ///
    /// ```no_run
    /// use std::env;
    /// use twilight_gateway::{config::{Config, ShardId}, Intents, Shard};
    /// use twilight_model::gateway::{
    ///     payload::outgoing::update_presence::UpdatePresencePayload,
    ///     presence::{ActivityType, MinimalActivity, Status},
    /// };
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Config::builder(env::var("DISCORD_TOKEN")?, Intents::empty())
    ///     .presence(UpdatePresencePayload::new(
    ///         vec![MinimalActivity {
    ///             kind: ActivityType::Playing,
    ///             name: "Not accepting commands".into(),
    ///             url: None,
    ///         }
    ///         .into()],
    ///         false,
    ///         None,
    ///         Status::Idle,
    ///     )?)
    ///     .build();
    ///
    /// let shard = Shard::with_config(ShardId::ONE, config).await?;
    /// # Ok(()) }
    ///
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    pub fn presence(mut self, presence: UpdatePresencePayload) -> Self {
        self.inner.presence = Some(presence);

        self
    }

    /// Set the queue to use for queueing shard connections.
    ///
    /// You probably don't need to set this yourself unless the application uses
    /// [Very Large Bot] sharding. Refer to the [`queue`] module for more
    /// information.
    ///
    /// [`queue`]: crate::queue
    pub fn queue(mut self, queue: Arc<dyn Queue>) -> Self {
        self.inner.queue = queue;

        self
    }

    /// Set whether or not outgoing messages will be ratelimited.
    ///
    /// Useful when running behind a proxy gateway. Running without a
    /// functional ratelimiter **will** get you ratelimited.
    ///
    /// Defaults to being enabled.
    pub const fn ratelimit_messages(mut self, ratelimit_messages: bool) -> Self {
        self.inner.ratelimit_messages = ratelimit_messages;

        self
    }

    /// Set the gateway session to use when connecting to the gateway.
    ///
    /// In practice this will result in the shard attempting to send a
    /// [`Resume`] to the gateway instead of identifying and creating a new
    /// session. Refer to the documentation for [`Session`] for more
    /// information.
    ///
    /// [`Resume`]: twilight_model::gateway::payload::outgoing::Resume
    #[allow(clippy::missing_const_for_fn)]
    pub fn session(mut self, session: Session) -> Self {
        self.inner.session = Some(session);

        self
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, ShardId};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(Config: Clone, Debug, Send, Sync);

    #[test]
    const fn test_shard_id() {
        let id = ShardId::new(2, 4);

        assert!(id.current() == 2);
        assert!(id.total() == 4);
    }

    #[should_panic]
    #[test]
    const fn test_shard_id_current_equal_invalid() {
        ShardId::new(4, 4);
    }

    #[should_panic]
    #[test]
    const fn test_shard_id_current_greater_invalid() {
        ShardId::new(10, 4);
    }

    #[should_panic]
    #[test]
    const fn test_shard_id_total_zero_invalid() {
        ShardId::new(0, 0);
    }

    #[test]
    const fn test_shard_id_new_checked() {
        assert!(ShardId::new_checked(0, 1).is_some());
        assert!(ShardId::new_checked(1, 1).is_none());
        assert!(ShardId::new_checked(2, 1).is_none());
        assert!(ShardId::new_checked(0, 0).is_none());
    }

    #[test]
    fn test_shard_id_display() {
        assert_eq!("shard 0/1", ShardId::ONE.to_string());
        assert_eq!("shard 2/4", ShardId::new(2, 4).to_string());
        assert_eq!("shard 13/102", ShardId::new(13, 102).to_string());
    }
}
