pub use reqwest::Proxy;

use reqwest::Client as ReqwestClient;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Config {
    pub(crate) proxy: Option<Proxy>,
    pub(crate) proxy_http: bool,
    pub(crate) reqwest_client: Option<ReqwestClient>,
    pub(crate) skip_ratelimiter: bool,
    pub(crate) timeout: Duration,
    pub(crate) token: Option<String>,
}

impl Config {
    /// Returns a builder to create a `Config`.
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    /// Returns an immutable reference to the proxy.
    pub fn proxy(&self) -> Option<&Proxy> {
        self.proxy.as_ref()
    }

    pub fn proxy_http(&self) -> bool {
        self.proxy_http
    }

    /// Returns an immutable reference to the reqwest client, if any.
    pub fn reqwest_client(&self) -> Option<&ReqwestClient> {
        self.reqwest_client.as_ref()
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
}

#[derive(Clone, Debug)]
pub struct ConfigBuilder(Config);

impl ConfigBuilder {
    /// Creates a new default builder.
    ///
    /// Refer to the methods for the default value of each configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Consumes the builder, returning the inner configuration.
    pub fn build(self) -> Config {
        self.0
    }

    /// Sets the proxy to use for all HTTP requests.
    ///
    /// This accepts a `reqwest::Proxy`.
    pub fn proxy(&mut self, proxy: Proxy) -> &mut Self {
        self.0.proxy.replace(proxy);

        self
    }

    pub fn proxy_http(&mut self, proxy_http: bool) -> &mut Self {
        self.0.proxy_http = proxy_http;

        self
    }

    /// Sets the reqwest client to use.
    ///
    /// All of the settings in the client will be overwritten by the settings
    /// in this configuration, if specified.
    ///
    /// The default client is a RusTLS-backed client.
    pub fn reqwest_client(&mut self, client: ReqwestClient) -> &mut Self {
        self.0.reqwest_client.replace(client);

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
        self.0.token.replace(token.into());

        self
    }
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self(Config {
            proxy: None,
            proxy_http: false,
            reqwest_client: None,
            skip_ratelimiter: false,
            timeout: Duration::from_secs(10),
            token: None,
        })
    }
}
