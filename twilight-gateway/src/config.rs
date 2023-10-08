//! User configuration for shards.

use crate::{tls::TlsContainer, EventTypeFlags, Session};
use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
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

/// Configuration used by the shard to identify with the gateway and operate.
///
/// Use [`Config::builder`] to configure a shard's configuration.
///
/// May be reused by cloning, also reusing the hidden TLS context---reducing
/// memory usage. The TLS context may still be reused with an otherwise
/// different config by turning it into to a [`ConfigBuilder`] through the
/// [`From<Config>`] implementation and then rebuilding it into a rew config.
#[derive(Clone, Debug)]
pub struct Config {
    /// Event type flags.
    event_types: EventTypeFlags,
    /// Identification properties the shard will use.
    identify_properties: Option<IdentifyProperties>,
    /// Intents that the shard requests when identifying with the gateway.
    intents: Intents,
    /// When the gateway will stop sending a guild's member list in
    /// Guild Create events.
    large_threshold: u64,
    /// Presence to set when identifying with the gateway.
    presence: Option<UpdatePresencePayload>,
    /// Gateway proxy URL.
    proxy_url: Option<Box<str>>,
    /// Queue in use by the shard.
    queue: Arc<dyn Queue>,
    /// Whether [outgoing message] ratelimiting is enabled.
    ///
    /// [outgoing message]: crate::Shard::send
    ratelimit_messages: bool,
    /// Session information to resume a shard on initialization.
    session: Option<Session>,
    /// TLS connector for Websocket connections.
    // We need this to be public so [`stream`] can reuse TLS on multiple shards
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

    /// Immutable reference to the presence to set when identifying
    /// with the gateway.
    ///
    /// This will be the bot's presence. For example, setting the online status
    /// to Do Not Disturb will show the status in the bot's presence.
    pub const fn presence(&self) -> Option<&UpdatePresencePayload> {
        self.presence.as_ref()
    }

    /// Immutable reference to the gateway proxy URL.
    pub fn proxy_url(&self) -> Option<&str> {
        self.proxy_url.as_deref()
    }

    /// Immutable reference to the queue in use by the shard.
    pub fn queue(&self) -> &Arc<dyn Queue> {
        &self.queue
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
                identify_properties: None,
                intents,
                large_threshold: 50,
                presence: None,
                proxy_url: None,
                queue: Arc::new(LocalQueue::new()),
                ratelimit_messages: true,
                session: None,
                tls: TlsContainer::new().unwrap(),
                token: Token::new(token.into_boxed_str()),
            },
        }
    }

    /// Create a new builder from an existing configuration.
    #[deprecated(since = "0.15.3", note = "use From<Config> instead")]
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
    /// The active presence of a session is maintained across re-connections
    /// when a session can be [successfully resumed], and when a new session has
    /// to be made shards will send the configured presence. Manually updating
    /// the presence after a disconnection isn't necessary.
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
    /// let shard = Shard::with_config(ShardId::ONE, config);
    /// # Ok(()) }
    /// ```
    ///
    /// [successfully resumed]: twilight_model::gateway::event::Event::Resumed
    #[allow(clippy::missing_const_for_fn)]
    pub fn presence(mut self, presence: UpdatePresencePayload) -> Self {
        self.inner.presence = Some(presence);

        self
    }

    /// Set the proxy URL for connecting to the gateway.
    ///
    /// Resumes are always done to the URL specified in [`resume_gateway_url`].
    ///
    /// [`resume_gateway_url`]: twilight_model::gateway::payload::incoming::Ready::resume_gateway_url
    #[allow(clippy::missing_const_for_fn)]
    pub fn proxy_url(mut self, proxy_url: String) -> Self {
        self.inner.proxy_url = Some(proxy_url.into_boxed_str());

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

impl From<Config> for ConfigBuilder {
    fn from(value: Config) -> Self {
        Self { inner: value }
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, ConfigBuilder};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;
    use twilight_model::gateway::Intents;

    assert_impl_all!(Config: Clone, Debug, Send, Sync);
    assert_impl_all!(ConfigBuilder: Debug, Send, Sync);

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
