pub use isahc::http::Uri;

use crate::request::channel::message::allowed_mentions::AllowedMentions;
use isahc::HttpClient;
use std::time::Duration;

#[derive(Debug)]
pub struct ClientConfig {
    pub(crate) isahc_client: Option<HttpClient>,
    pub(crate) proxy: Option<Uri>,
    pub(crate) proxy_http: bool,
    pub(crate) skip_ratelimiter: bool,
    pub(crate) timeout: Duration,
    pub(crate) token: Option<String>,
    pub(crate) default_allowed_mentions: Option<AllowedMentions>,
}

impl ClientConfig {
    /// Returns a builder to create a `ClientConfig`.
    pub fn builder() -> ClientConfigBuilder {
        ClientConfigBuilder::new()
    }

    /// Returns an immutable reference to the proxy.
    pub fn proxy(&self) -> Option<&Uri> {
        self.proxy.as_ref()
    }

    pub fn proxy_http(&self) -> bool {
        self.proxy_http
    }

    /// Returns an immutable reference to the isahc client, if any.
    pub fn isahc_client(&self) -> Option<&HttpClient> {
        self.isahc_client.as_ref()
    }

    pub fn skip_ratelimiter(&self) -> bool {
        self.skip_ratelimiter
    }

    /// Returns an immutable reference to the token.
    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    /// Returns an immutable reference to the token.
    pub fn token(&self) -> Option<&str> {
        self.token.as_ref().map(AsRef::as_ref)
    }

    /// The default allowed mentions setting to use on all messages send through this httpclient
    pub fn default_allowed_mention(&self) -> Option<&AllowedMentions> {
        self.default_allowed_mentions.as_ref()
    }
}

#[derive(Debug)]
pub struct ClientConfigBuilder(ClientConfig);

impl ClientConfigBuilder {
    /// Creates a new default builder.
    ///
    /// Refer to the methods for the default value of each configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Consumes the builder, returning the inner configuration.
    pub fn build(self) -> ClientConfig {
        self.0
    }

    /// Sets the proxy to use for all HTTP requests.
    ///
    /// This accepts a `http::Uri`.
    pub fn proxy(&mut self, proxy: Uri) -> &mut Self {
        self.0.proxy.replace(proxy);

        self
    }

    pub fn proxy_http(&mut self, proxy_http: bool) -> &mut Self {
        self.0.proxy_http = proxy_http;

        self
    }

    /// Sets the isahc client to use.
    ///
    /// All of the settings in the client will be overwritten by the settings
    /// in this configuration, if specified.
    ///
    /// The default client is a RusTLS-backed client.
    pub fn isahc_client(&mut self, client: HttpClient) -> &mut Self {
        self.0.isahc_client.replace(client);

        self
    }

    /// Sets whether to skip the client's ratelimiter before making the request.
    ///
    /// The default is `false`.
    pub fn skip_ratelimiter(&mut self, skip_ratelimiter: bool) -> &mut Self {
        self.0.skip_ratelimiter = skip_ratelimiter;

        self
    }

    /// Sets the timeout for HTTP requests.
    ///
    /// The default is 10 seconds.
    pub fn timeout(&mut self, duration: Duration) -> &mut Self {
        self.0.timeout = duration;

        self
    }

    /// Sets the token to use for HTTP requests.
    pub fn token(&mut self, token: impl Into<String>) -> &mut Self {
        let mut token = token.into();

        // Make sure it is a bot token.
        if !token.starts_with("Bot ") {
            token.insert_str(0, "Bot ");
        }

        self.0.token.replace(token);

        self
    }

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
            isahc_client: None,
            skip_ratelimiter: false,
            timeout: Duration::from_secs(10),
            token: None,
            default_allowed_mentions: None,
        })
    }
}
