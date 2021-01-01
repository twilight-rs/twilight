use super::{Client, HttpsConnector, State};
use crate::{
    ratelimiting::Ratelimiter, request::channel::message::allowed_mentions::AllowedMentions,
};
use hyper::client::{Client as HyperClient, HttpConnector};
use std::{
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

#[derive(Debug)]
/// A builder for [`Client`].
pub struct ClientBuilder {
    pub(crate) default_allowed_mentions: Option<AllowedMentions>,
    pub(crate) proxy: Option<Box<str>>,
    pub(crate) ratelimiter: Option<Ratelimiter>,
    pub(crate) hyper_client: Option<HyperClient<HttpsConnector<HttpConnector>>>,
    pub(crate) timeout: Duration,
    pub(crate) token: Option<Box<str>>,
    pub(crate) use_http: bool,
}

impl ClientBuilder {
    /// Create a new builder to create a [`Client`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the [`Client`].
    pub fn build(self) -> Client {
        let http = self.hyper_client.unwrap_or_else(|| {
            #[cfg(feature = "hyper-rustls")]
            let connector = hyper_rustls::HttpsConnector::with_native_roots();
            #[cfg(all(feature = "hyper-tls", not(feature = "hyper-rustls")))]
            let connector = hyper_tls::HttpsConnector::new();

            hyper::client::Builder::default().build(connector)
        });

        Client {
            state: Arc::new(State {
                http,
                proxy: self.proxy,
                ratelimiter: self.ratelimiter,
                timeout: self.timeout,
                token_invalid: AtomicBool::new(false),
                token: self.token,
                default_allowed_mentions: self.default_allowed_mentions,
                use_http: self.use_http,
            }),
        }
    }

    /// Set the default allowed mentions setting to use on all messages sent through the HTTP
    /// client.
    pub fn default_allowed_mentions(mut self, allowed_mentions: AllowedMentions) -> Self {
        self.default_allowed_mentions.replace(allowed_mentions);

        self
    }

    /// Set a pre-configured Hyper client builder to build off of.
    ///
    /// The proxy and timeout settings in the Hyper client will be overridden by
    /// those in this builder.
    ///
    /// The default client uses Rustls as its TLS backend.
    pub fn hyper_client(mut self, client: HyperClient<HttpsConnector<HttpConnector>>) -> Self {
        self.hyper_client.replace(client);

        self
    }

    /// Set the proxy to use for all HTTP(S) requests.
    ///
    /// **Note** that this isn't currently a traditional proxy, but is for
    /// working with something like [twilight's HTTP proxy server].
    ///
    /// # Examples
    ///
    /// Set the proxy to `twilight_http_proxy.internal`:
    ///
    /// ```rust
    /// use twilight_http::Client;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder()
    ///     .proxy("twilight_http_proxy.internal", true)
    ///     .build();
    /// # Ok(()) }
    /// ```
    ///
    /// [twilight's HTTP proxy server]: https://github.com/twilight-rs/http-proxy
    pub fn proxy(mut self, proxy_url: impl Into<String>, use_http: bool) -> Self {
        self.proxy.replace(proxy_url.into().into_boxed_str());
        self.use_http = use_http;

        self
    }

    /// Set a ratelimiter to use.
    ///
    /// If the argument is `None` then the client's ratelimiter will be skipped
    /// before making a request.
    ///
    /// If this method is not called at all then a default ratelimiter will be
    /// created by [`ClientBuilder::build`].
    pub fn ratelimiter(mut self, ratelimiter: impl Into<Option<Ratelimiter>>) -> Self {
        self.ratelimiter = ratelimiter.into();

        self
    }

    /// Set the timeout for HTTP requests.
    ///
    /// The default is 10 seconds.
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;

        self
    }

    /// Set the token to use for HTTP requests.
    pub fn token(mut self, token: impl Into<String>) -> Self {
        let mut token = token.into();

        let is_bot = token.starts_with("Bot ");
        let is_bearer = token.starts_with("Bearer ");

        // Make sure it is either a bot or bearer token, and assume it's a bot
        // token if no prefix is given
        if !is_bot && !is_bearer {
            token.insert_str(0, "Bot ");
        }

        self.token.replace(token.into_boxed_str());

        self
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            default_allowed_mentions: None,
            hyper_client: None,
            proxy: None,
            ratelimiter: Some(Ratelimiter::new()),
            timeout: Duration::from_secs(10),
            token: None,
            use_http: false,
        }
    }
}
