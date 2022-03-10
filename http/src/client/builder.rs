use super::Client;
use hyper::header::HeaderMap;
use std::{
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};
use twilight_http_ratelimiting::{InMemoryRatelimiter, Ratelimiter};
use twilight_model::channel::message::allowed_mentions::AllowedMentions;

#[derive(Debug)]
/// A builder for [`Client`].
pub struct ClientBuilder {
    pub(crate) default_allowed_mentions: Option<AllowedMentions>,
    pub(crate) proxy: Option<Box<str>>,
    pub(crate) ratelimiter: Option<Box<dyn Ratelimiter>>,
    remember_invalid_token: bool,
    pub(crate) default_headers: Option<HeaderMap>,
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
        #[cfg(not(feature = "trust-dns"))]
        let mut http_connector = hyper::client::HttpConnector::new();
        #[cfg(feature = "trust-dns")]
        let mut http_connector = hyper_trust_dns::new_trust_dns_http_connector();

        http_connector.enforce_http(false);

        #[cfg(feature = "rustls-native-roots")]
        let connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .wrap_connector(http_connector);

        #[cfg(all(feature = "rustls-webpki-roots", not(feature = "rustls-native-roots")))]
        let connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_or_http()
            .enable_http1()
            .enable_http2()
            .wrap_connector(http_connector);

        #[cfg(all(
            feature = "hyper-tls",
            not(feature = "rustls-native-roots"),
            not(feature = "rustls-webpki-roots")
        ))]
        let connector = hyper_tls::HttpsConnector::new_with_connector(http_connector);

        let http = hyper::client::Builder::default().build(connector);

        let token_invalidated = if self.remember_invalid_token {
            Some(Arc::new(AtomicBool::new(false)))
        } else {
            None
        };

        Client {
            http,
            default_headers: self.default_headers,
            proxy: self.proxy,
            ratelimiter: self.ratelimiter,
            timeout: self.timeout,
            token_invalidated,
            token: self.token,
            default_allowed_mentions: self.default_allowed_mentions,
            use_http: self.use_http,
        }
    }

    /// Set the default allowed mentions setting to use on all messages sent through the HTTP
    /// client.
    #[must_use = "has no effect if not built into a Client"]
    pub fn default_allowed_mentions(mut self, allowed_mentions: AllowedMentions) -> Self {
        self.default_allowed_mentions.replace(allowed_mentions);

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
    /// ```
    /// use twilight_http::Client;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::builder()
    ///     .proxy("twilight_http_proxy.internal".to_owned(), true)
    ///     .build();
    /// # Ok(()) }
    /// ```
    ///
    /// [twilight's HTTP proxy server]: https://github.com/twilight-rs/http-proxy
    #[must_use = "has no effect if not built into a Client"]
    pub fn proxy(mut self, proxy_url: String, use_http: bool) -> Self {
        self.proxy.replace(proxy_url.into_boxed_str());
        self.use_http = use_http;

        self
    }

    /// Set a ratelimiter to use.
    ///
    /// If the argument is `None` then the client's ratelimiter will be skipped
    /// before making a request.
    ///
    /// If this method is not called at all then a default [`InMemoryRatelimiter`] will be
    /// created by [`ClientBuilder::build`].
    #[allow(clippy::missing_const_for_fn)]
    #[must_use = "has no effect if not built into a Client"]
    pub fn ratelimiter(mut self, ratelimiter: Option<Box<dyn Ratelimiter>>) -> Self {
        self.ratelimiter = ratelimiter;

        self
    }

    /// Set the timeout for HTTP requests.
    ///
    /// The default is 10 seconds.
    #[must_use = "has no effect if not built into a Client"]
    pub const fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;

        self
    }

    /// Set a group headers which are sent in every request.
    #[must_use = "has no effect if not built into a Client"]
    pub fn default_headers(mut self, headers: HeaderMap) -> Self {
        self.default_headers.replace(headers);

        self
    }

    /// Whether to remember whether the client has encountered an Unauthorized
    /// response status.
    ///
    /// If the client remembers encountering an Unauthorized response, then it
    /// will not process future requests.
    ///
    /// Defaults to true.
    #[must_use = "has no effect if not built into a Client"]
    pub const fn remember_invalid_token(mut self, remember: bool) -> Self {
        self.remember_invalid_token = remember;

        self
    }

    /// Set the token to use for HTTP requests.
    #[must_use = "has no effect if not built into a Client"]
    pub fn token(mut self, mut token: String) -> Self {
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
            default_headers: None,
            proxy: None,
            ratelimiter: Some(Box::new(InMemoryRatelimiter::default())),
            remember_invalid_token: true,
            timeout: Duration::from_secs(10),
            token: None,
            use_http: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ClientBuilder;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(ClientBuilder: Debug, Default, Send, Sync);
}
