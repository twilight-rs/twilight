use reqwest::{Client as ReqwestClient, Proxy};
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct Config {
    pub(crate) proxy: Option<Proxy>,
    pub(crate) reqwest_client: Option<ReqwestClient>,
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

    /// Returns an immutable reference to the reqwest client, if any.
    pub fn reqwest_client(&self) -> Option<&ReqwestClient> {
        self.reqwest_client.as_ref()
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
            reqwest_client: None,
            timeout: Duration::from_secs(10),
            token: None,
        })
    }
}
