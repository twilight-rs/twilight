//! User configuration for shards.

use crate::{tls::TlsContainer, EventTypeFlags, Session};
use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    sync::Arc,
};
use twilight_gateway_queue::{LocalQueue, Queue};
use twilight_model::gateway::{
    payload::outgoing::{identify::IdentifyProperties, update_presence::UpdatePresencePayload},
    Intents,
};

/// Wrapper for an authorization token with a debug implementation that redacts
/// the string.
#[derive(Clone, Default)]
struct Token {
    /// Authorization token that is redacted in the Debug implementation.
    inner: Box<str>,
}

impl Token {
    /// Create a new authorization wrapper.
    const fn new(token: Box<str>) -> Self {
        Self { inner: token }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("<redacted>")
    }
}

/// [`Shard`] identifier to calculate if it receivies a given event.
///
/// A shard ID consist of two fields: `number` and `total`. These values do not
/// need to be unique, and are used by Discord for calculating which events to
/// send to which shard. Shards should in general share the same `total` value
/// and have an unique `number` value, but users may deviate from this when
/// resharding/migrating to a new set of shards.
///
/// # Advanced use
///
/// Incoming events are split by their originating guild and are received by the
/// shard with the id calculated from the following formula:
///
/// > `number = (guild_id >> 22) % total`.
///
/// `total` is in other words unrelated to the total number of shards and is
/// only used to specify the share of events a shard will receive. The formula
/// is independently calculated for all shards, which means that events may be
/// duplicated or lost if it's determined that an event should be sent to
/// multiple or no shard.
///
/// It may be helpful to visualize the logic in code:
///
/// ```
/// use twilight_gateway::Shard;
///
/// fn send(shards: &[Shard], guild_id: u64) {
///     for shard in shards {
///         if shard.id().number() == (guild_id >> 22) % shard.id().total() {
///             unimplemented!("send event to shard");
///         }
///     }
/// }
/// ```
///
/// See [Discord Docs/Sharding].
///
/// [`shard`]: crate::Shard
/// [Discord Docs/Sharding]: https://discord.com/developers/docs/topics/gateway#sharding
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ShardId {
    /// Number of the shard, 0-indexed.
    number: u64,
    /// Total number of shards used by the bot, 1-indexed.
    total: u64,
}

impl ShardId {
    /// ID of a bot that has only one shard.
    ///
    /// Should *only* be used by small bots in under one or two thousand guilds.
    pub const ONE: ShardId = ShardId::new(0, 1);

    /// Create a new shard identifier.
    ///
    /// The shard number is 0-indexed while the total number of shards is
    /// 1-indexed. A shard number of 7 with a total of 8 is therefore valid,
    /// whilst a shard number of 8 out of 8 total shards is invalid.
    ///
    /// # Examples
    ///
    /// Create a new shard with a shard number of 13 out of a total of 24
    /// shards:
    ///
    /// ```
    /// use twilight_gateway::ShardId;
    ///
    /// let id = ShardId::new(13, 24);
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the shard number is greater than or equal to the total number
    /// of shards, or if the total number of shards is zero.
    pub const fn new(number: u64, total: u64) -> Self {
        assert!(total > 0, "total must be non-zero");
        assert!(
            number < total,
            "shard number (0-indexed) must be less than total (1-indexed)",
        );

        Self { number, total }
    }

    /// Create a new shard identifier if the shard indexes are valid.
    pub const fn new_checked(number: u64, total: u64) -> Option<Self> {
        if total > 0 && number < total {
            Some(Self { number, total })
        } else {
            None
        }
    }

    /// Identifying number of the shard, 0-indexed.
    pub const fn number(self) -> u64 {
        self.number
    }

    /// Total number of shards, 1-indexed.
    pub const fn total(self) -> u64 {
        self.total
    }
}

/// Display the shard ID.
///
/// Formats as `[{number}, {total}]`.
impl Display for ShardId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("[")?;
        Display::fmt(&self.number, f)?;
        f.write_str(", ")?;
        Display::fmt(&self.total, f)?;

        f.write_str("]")
    }
}

/// Configuration used by the shard to identify with the gateway and operate.
///
/// Use [`ConfigBuilder`] to configure all settings, easily created through the
/// [`Config::builder`] or [`ConfigBuilder::with_config`] functions, and turn it
/// into a config through the [`ConfigBuilder::build`] method.
///
/// May be reused by cloning, also reusing the hidden TLS context---reducing
/// memory usage. The TLS context may still be reused with an otherwise
/// different config by turning it into to a [`ConfigBuilder`] through the
/// [`ConfigBuilder::with_config`] function and then rebuilding it into a new
/// config.
#[derive(Clone, Debug)]
pub struct Config {
    /// Event type flags.
    event_types: EventTypeFlags,
    /// URL used to connect to the gateway.
    gateway_url: Option<String>,
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
    ///
    /// The token is prefixed with "Bot ", which is required by Discord for
    /// authentication.
    token: Token,
}

impl Config {
    /// Create a new default shard configuration.
    ///
    /// Shortcut for calling [`builder`][`Self::builder`] and immediately
    /// finalizing the builder.
    ///
    /// # Panics
    ///
    /// Panics if loading TLS certificates fails.
    pub fn new(token: String, intents: Intents) -> Self {
        Self::builder(token, intents).build()
    }

    /// Create a builder to customize a shard's configuration.
    ///
    /// # Panics
    ///
    /// Panics if loading TLS certificates fails.
    pub fn builder(token: String, intents: Intents) -> ConfigBuilder {
        ConfigBuilder::new(token, intents)
    }

    /// Event type flags.
    pub const fn event_types(&self) -> EventTypeFlags {
        self.event_types
    }

    /// Immutable reference to the URL used to connect to the gateway.
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

    /// Immutable reference to the presence to set when identifying
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

    /// Immutable reference to the TLS connector in use by the shard.
    pub(crate) const fn tls(&self) -> &TlsContainer {
        &self.tls
    }

    /// Immutable reference to the token used to authenticate when identifying
    /// with the gateway.
    pub const fn token(&self) -> &str {
        &self.token.inner
    }

    /// Session information to resume a shard on initialization.
    pub(crate) fn take_session(&mut self) -> Option<Session> {
        self.session.take()
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
    ///
    /// # Panics
    ///
    /// Panics if loading TLS certificates fails.
    pub fn new(mut token: String, intents: Intents) -> Self {
        if !token.starts_with("Bot ") {
            token.insert_str(0, "Bot ");
        }

        Self {
            inner: Config {
                event_types: EventTypeFlags::all(),
                gateway_url: None,
                identify_properties: None,
                intents,
                large_threshold: 50,
                presence: None,
                queue: Arc::new(LocalQueue::new()),
                ratelimit_messages: true,
                session: None,
                tls: TlsContainer::new().unwrap(),
                token: Token::new(token.into_boxed_str()),
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

    /// Set the proxy URL for connecting to the gateway.
    ///
    /// When reconnecting, the shard will always use this URL instead of
    /// [`resume_gateway_url`]. Proper reconnection is left to the proxy.
    ///
    /// [`resume_gateway_url`]: twilight_model::gateway::payload::incoming::Ready::resume_gateway_url
    #[allow(clippy::missing_const_for_fn)]
    pub fn gateway_url(mut self, gateway_url: String) -> Self {
        self.inner.gateway_url = Some(gateway_url);

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
    /// use twilight_gateway::{Config, Intents, Shard};
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
    #[track_caller]
    pub const fn large_threshold(mut self, large_threshold: u64) -> Self {
        /// Maximum acceptable large threshold.
        const MAXIMUM: u64 = 250;

        /// Minimum acceptable large threshold.
        const MINIMUM: u64 = 50;

        assert!(
            large_threshold >= MINIMUM && large_threshold <= MAXIMUM,
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
    /// use twilight_gateway::{Config, Intents, Shard, ShardId};
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
    /// let shard = Shard::new(ShardId::ONE, config);
    /// # Ok(()) }
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    pub fn presence(mut self, presence: UpdatePresencePayload) -> Self {
        self.inner.presence = Some(presence);

        self
    }

    /// Set the queue to use for queueing shard sessions.
    ///
    /// Defaults to a [`LocalQueue`].
    ///
    /// Refer to the [`queue`] module for more information.
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
    use super::{Config, ConfigBuilder, ShardId};
    use static_assertions::{assert_impl_all, const_assert_eq};
    use std::{fmt::Debug, hash::Hash};
    use twilight_model::gateway::Intents;

    const_assert_eq!(ShardId::ONE.number(), 0);
    const_assert_eq!(ShardId::ONE.total(), 1);
    assert_impl_all!(Config: Clone, Debug, Send, Sync);
    assert_impl_all!(ConfigBuilder: Debug, Send, Sync);
    assert_impl_all!(ShardId: Clone, Copy, Debug, Eq, Hash, PartialEq, Send, Sync);

    fn builder() -> ConfigBuilder {
        ConfigBuilder::new("test".to_owned(), Intents::empty())
    }

    #[tokio::test]
    async fn large_threshold() {
        const INPUTS: &[u64] = &[50, 100, 150, 200, 250];

        for input in INPUTS {
            assert_eq!(
                builder().large_threshold(*input).build().large_threshold(),
                *input,
            );
        }
    }

    #[should_panic]
    #[tokio::test]
    async fn large_threshold_minimum() {
        drop(builder().large_threshold(49));
    }

    #[should_panic]
    #[tokio::test]
    async fn large_threshold_maximum() {
        drop(builder().large_threshold(251));
    }

    #[test]
    const fn shard_id() {
        let id = ShardId::new(2, 4);

        assert!(id.number() == 2);
        assert!(id.total() == 4);
    }

    #[should_panic]
    #[test]
    const fn shard_id_number_equal_invalid() {
        ShardId::new(4, 4);
    }

    #[should_panic]
    #[test]
    const fn shard_id_number_greater_invalid() {
        ShardId::new(10, 4);
    }

    #[should_panic]
    #[test]
    const fn shard_id_total_zero_invalid() {
        ShardId::new(0, 0);
    }

    #[test]
    const fn shard_id_new_checked() {
        assert!(ShardId::new_checked(0, 1).is_some());
        assert!(ShardId::new_checked(1, 1).is_none());
        assert!(ShardId::new_checked(2, 1).is_none());
        assert!(ShardId::new_checked(0, 0).is_none());
    }

    #[test]
    fn shard_id_display() {
        assert_eq!("[0, 1]", ShardId::ONE.to_string());
        assert_eq!("[2, 4]", ShardId::new(2, 4).to_string());
        assert_eq!("[13, 102]", ShardId::new(13, 102).to_string());
    }

    #[tokio::test]
    async fn config_prefixes_bot_to_token() {
        const WITHOUT: &str = "test";
        const WITH: &str = "Bot test";

        assert_eq!(
            ConfigBuilder::new(WITHOUT.to_owned(), Intents::empty())
                .build()
                .token
                .inner
                .as_ref(),
            WITH
        );
        assert_eq!(
            ConfigBuilder::new(WITH.to_owned(), Intents::empty())
                .build()
                .token
                .inner
                .as_ref(),
            WITH
        );
    }

    #[tokio::test]
    async fn config_debug() {
        let config = Config::new("Bot foo".to_owned(), Intents::empty());

        assert!(format!("{config:?}").contains("token: <redacted>"));
    }
}
