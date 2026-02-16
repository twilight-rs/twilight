#[cfg(not(target_os = "wasi"))]
use super::Token;
use crate::Client;
#[cfg(not(target_os = "wasi"))]
use crate::client::connector;
use http::header::HeaderMap;
#[cfg(not(target_os = "wasi"))]
use hyper_util::rt::TokioExecutor;
use std::{
    sync::{Arc, atomic::AtomicBool},
    time::Duration,
};
#[cfg(not(target_os = "wasi"))]
use twilight_http_ratelimiting::RateLimiter;
use twilight_model::channel::message::AllowedMentions;

/// A builder for [`Client`].
#[derive(Debug)]
#[must_use = "has no effect if not built into a Client"]
#[cfg(not(target_os = "wasi"))]
pub struct ClientBuilder {
    pub(crate) default_allowed_mentions: Option<AllowedMentions>,
    pub(crate) proxy: Option<Box<str>>,
    ratelimiter: Option<RateLimiter>,
    remember_invalid_token: bool,
    pub(crate) default_headers: Option<HeaderMap>,
    pub(crate) timeout: Duration,
    pub(super) token: Option<Token>,
    pub(crate) use_http: bool,
}

/// A builder for [`Client`].
#[derive(Debug)]
#[must_use = "has no effect if not built into a Client"]
#[cfg(target_os = "wasi")]
pub struct ClientBuilder {
    pub(crate) default_allowed_mentions: Option<AllowedMentions>,
}

impl ClientBuilder {
    /// Create a new builder to create a [`Client`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Build the [`Client`].
    #[cfg(not(target_os = "wasi"))]
    pub fn build(self) -> Client {
        let connector = connector::create();

        let http =
            hyper_util::client::legacy::Client::builder(TokioExecutor::new()).build(connector);

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

    /// Build the [`Client`].
    #[cfg(target_os = "wasi")]
    pub fn build(self) -> Client {
        Client {
            default_allowed_mentions: self.default_allowed_mentions,
        }
    }

    /// Set the default allowed mentions setting to use on all messages sent through the HTTP
    /// client.
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
    /// ```no_run
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
    #[cfg(not(target_os = "wasi"))]
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
    /// If not called, then a default [`RateLimiter`] will be created by
    /// [`ClientBuilder::build`].
    #[allow(clippy::missing_const_for_fn)]
    #[cfg(not(target_os = "wasi"))]
    pub fn ratelimiter(mut self, ratelimiter: Option<RateLimiter>) -> Self {
        self.ratelimiter = ratelimiter;

        self
    }

    /// Set the timeout for HTTP requests.
    ///
    /// The default is 10 seconds.
    #[cfg(not(target_os = "wasi"))]
    pub const fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;

        self
    }

    /// Set a group headers which are sent in every request.
    #[cfg(not(target_os = "wasi"))]
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
    #[cfg(not(target_os = "wasi"))]
    pub const fn remember_invalid_token(mut self, remember: bool) -> Self {
        self.remember_invalid_token = remember;

        self
    }

    /// Set the token to use for HTTP requests.
    #[cfg(not(target_os = "wasi"))]
    pub fn token(mut self, mut token: String) -> Self {
        let is_bot = token.starts_with("Bot ");
        let is_bearer = token.starts_with("Bearer ");

        // Make sure it is either a bot or bearer token, and assume it's a bot
        // token if no prefix is given
        if !is_bot && !is_bearer {
            token.insert_str(0, "Bot ");
        }

        self.token.replace(Token::new(token.into_boxed_str()));

        self
    }
}

impl Default for ClientBuilder {
    #[cfg(not(target_os = "wasi"))]
    fn default() -> Self {
        Self {
            default_allowed_mentions: None,
            default_headers: None,
            proxy: None,
            ratelimiter: (!cfg!(test)).then(RateLimiter::default),
            remember_invalid_token: true,
            timeout: Duration::from_secs(10),
            token: None,
            use_http: false,
        }
    }

    #[cfg(target_os = "wasi")]
    fn default() -> Self {
        Self {
            default_allowed_mentions: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ClientBuilder;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(ClientBuilder: Debug, Default, Send, Sync);

    #[test]
    fn client_debug() {
        assert!(
            format!("{:?}", ClientBuilder::new().token("Bot foo".to_owned()))
                .contains("token: Some(<redacted>)")
        );
        assert!(format!("{:?}", ClientBuilder::new()).contains("token: None"));
    }
}
