pub use reqwest::Proxy;

use crate::request::channel::message::allowed_mentions::AllowedMentions;
use reqwest::Client as ReqwestClient;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct ClientConfig {
    pub(crate) proxy: Option<Proxy>,
    pub(crate) proxy_http: bool,
    pub(crate) reqwest_client: Option<ReqwestClient>,
    pub(crate) skip_ratelimiter: bool,
    pub(crate) timeout: Duration,
    pub(crate) token: Option<String>,
    pub(crate) default_allowed_mentions: Option<AllowedMentions>,
}

impl ClientConfig {
    /// Create a builder to create a `ClientConfig`.
    pub fn builder() -> ClientConfigBuilder {
        ClientConfigBuilder::new()
    }

    /// Retrieve an immutable reference to the proxy.
    pub fn proxy(&self) -> Option<&Proxy> {
        self.proxy.as_ref()
    }

    /// Retrieve whether to proxy over HTTP.
    pub fn proxy_http(&self) -> bool {
        self.proxy_http
    }

    /// Retrieve an immutable reference to the reqwest client, if any.
    pub fn reqwest_client(&self) -> Option<&ReqwestClient> {
        self.reqwest_client.as_ref()
    }

    /// Retrieve whether to skip the ratelimiter.
    ///
    /// This should only be used when you're doing this elsewhere, such as when
    /// using a ratelimited proxy.
    pub fn skip_ratelimiter(&self) -> bool {
        self.skip_ratelimiter
    }

    /// Retrieve an immutable reference to the token.
    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    /// Retrieve an immutable reference to the token.
    pub fn token(&self) -> Option<&str> {
        self.token.as_ref().map(AsRef::as_ref)
    }

    /// Retrieve an immutable reference to the default allowed mentions setting
    /// to use on all messages sent through the HTTP client.
    pub fn default_allowed_mention(&self) -> Option<&AllowedMentions> {
        self.default_allowed_mentions.as_ref()
    }
}

#[derive(Clone, Debug)]
pub struct ClientConfigBuilder(ClientConfig);

impl ClientConfigBuilder {
    /// Create a new default builder.
    ///
    /// Refer to the methods for the default value of each configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Consume the builder, returning the inner configuration.
    pub fn build(self) -> ClientConfig {
        self.0
    }

    /// Sets the proxy to use for all HTTP requests.
    ///
    /// This accepts a `reqwest::Proxy`.
    pub fn proxy(&mut self, proxy: Proxy) -> &mut Self {
        self.0.proxy.replace(proxy);

        self
    }

    /// Set whether to proxy over HTTP.
    ///
    /// The default is `false`.
    pub fn proxy_http(&mut self, proxy_http: bool) -> &mut Self {
        self.0.proxy_http = proxy_http;

        self
    }

    /// Set the reqwest client to use.
    ///
    /// All of the settings in the client will be overwritten by the settings
    /// in this configuration, if specified.
    ///
    /// The default client is a RusTLS-backed client.
    pub fn reqwest_client(&mut self, client: ReqwestClient) -> &mut Self {
        self.0.reqwest_client.replace(client);

        self
    }

    /// Set whether to skip the client's ratelimiter before making the request.
    ///
    /// The default is `false`.
    pub fn skip_ratelimiter(&mut self, skip_ratelimiter: bool) -> &mut Self {
        self.0.skip_ratelimiter = skip_ratelimiter;

        self
    }

    /// Set the timeout for HTTP requests.
    ///
    /// The default is 10 seconds.
    pub fn timeout(&mut self, duration: Duration) -> &mut Self {
        self.0.timeout = duration;

        self
    }

    /// Set the token to use for HTTP requests.
    pub fn token(&mut self, token: impl Into<String>) -> &mut Self {
        let mut token = token.into();

        // Make sure it is a bot token.
        if !token.starts_with("Bot ") {
            token.insert_str(0, "Bot ");
        }

        self.0.token.replace(token);

        self
    }

    /// Set the default allowed mentions setting to use on all messages sent
    /// through the HTTP client.
    pub fn default_allowed_mentions(&mut self, allowed_mentions: AllowedMentions) -> &mut Self {
        self.0.default_allowed_mentions.replace(allowed_mentions);
        self
    }
}

impl Default for ClientConfigBuilder {
    fn default() -> Self {
        Self(ClientConfig {
            proxy: None,
            proxy_http: false,
            reqwest_client: None,
            skip_ratelimiter: false,
            timeout: Duration::from_secs(10),
            token: None,
            default_allowed_mentions: None,
        })
    }
}
