use super::{BytesFuture, Response};
use crate::{
    api_error::ApiError,
    client::connector::Connector,
    error::{Error, ErrorType},
};
use http::{header, HeaderMap, HeaderValue, Request, StatusCode};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper_util::client::legacy::{Client as HyperClient, ResponseFuture as HyperResponseFuture};
use std::{
    future::{ready, Future, Ready},
    marker::PhantomData,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    task::{ready, Context, Poll},
    time::{Duration, Instant},
};
use tokio::time::{self, Timeout};
use twilight_http_ratelimiting::{Path, Permit, PermitFuture, RateLimitHeaders, RateLimiter};

/// Parse ratelimit headers from a map of headers.
///
/// # Errors
///
/// Errors if a required header is missing or if a header value is of an
/// invalid type.
fn parse_ratelimit_headers(
    headers: &HeaderMap,
) -> Result<Option<RateLimitHeaders>, Box<dyn std::error::Error>> {
    match headers
        .get(RateLimitHeaders::SCOPE)
        .map(HeaderValue::as_bytes)
    {
        Some(b"global") => {
            tracing::info!("globally rate limited");

            Ok(None)
        }
        Some(b"shared") => {
            let bucket = headers
                .get(RateLimitHeaders::BUCKET)
                .ok_or("missing bucket header")?
                .as_bytes()
                .to_vec();
            let retry_after = headers
                .get(header::RETRY_AFTER)
                .ok_or("missing retry-after header")?
                .to_str()?
                .parse()?;

            Ok(Some(RateLimitHeaders::shared(bucket, retry_after)))
        }
        Some(b"user") => {
            let bucket = headers
                .get(RateLimitHeaders::BUCKET)
                .ok_or("missing bucket header")?
                .as_bytes()
                .to_vec();
            let limit = headers
                .get(RateLimitHeaders::LIMIT)
                .ok_or("missing limit header")?
                .to_str()?
                .parse()?;
            let remaining = headers
                .get(RateLimitHeaders::REMAINING)
                .ok_or("missing remaining header")?
                .to_str()?
                .parse()?;
            let reset_after = headers
                .get(RateLimitHeaders::RESET_AFTER)
                .ok_or("missing reset-after header")?
                .to_str()?
                .parse()?;

            Ok(Some(RateLimitHeaders {
                bucket,
                limit,
                remaining,
                reset_at: Instant::now() + Duration::from_secs_f32(reset_after),
            }))
        }
        _ => Ok(None),
    }
}

/// Sub-futures of [`ResponseFuture`].
enum ResponseStageFuture {
    /// Future that completes with an error response body.
    Error {
        /// Inner response body future.
        fut: BytesFuture,
        /// Erroneous response status code.
        status: StatusCode,
    },
    /// Future that completes when a rate limit permit is ready.
    RateLimitPermit(PermitFuture),
    /// Future that completes with a response or timeout.
    Response {
        /// Inner timed response future.
        fut: Pin<Box<Timeout<HyperResponseFuture>>>,
        /// Optional rate limit permit.
        permit: Option<Permit>,
    },
}

/// [`PermitFuture`] generator.
struct PermitFutureGenerator {
    /// Rate limiter to acquire permits from.
    rate_limiter: RateLimiter,
    /// Rate limiter path to acquire permits for.
    path: Path,
}

impl PermitFutureGenerator {
    /// Generates a permit future.
    fn generate(&self) -> PermitFuture {
        self.rate_limiter.acquire(self.path.clone())
    }
}

/// [`Timeout<HyperResponseFuture>`] generator.
struct TimedResponseFutureGenerator {
    /// HTTP client to send requests from.
    client: HyperClient<Connector, Full<Bytes>>,
    /// HTTP request to send.
    request: Request<Full<Bytes>>,
    /// Duration after which the request times out.
    timeout: Duration,
}

impl TimedResponseFutureGenerator {
    /// Generates a timeout response future.
    fn generate(&self) -> Pin<Box<Timeout<HyperResponseFuture>>> {
        Box::pin(time::timeout(
            self.timeout,
            self.client.request(self.request.clone()),
        ))
    }
}

/// Future that completes when a [`Response`] is received.
///
/// # Rate limits
///
/// Requests that exceed a rate limit are automatically and immediately retried
/// until they succeed or fail with another error. If configured without a
/// [`RateLimiter`], care must be taken that an external service intercepts and
/// delays these retry requests.
///
/// # Canceling a response future pre-flight
///
/// Response futures can be canceled pre-flight via
/// [`ResponseFuture::set_pre_flight`]. This allows you to cancel requests that
/// are no longer necessary once they have been cleared by the ratelimit queue,
/// which may be necessary in scenarios where requests are being spammed. Refer
/// to its documentation for more information.
///
/// # Errors
///
/// Returns an [`ErrorType::Parsing`] error type if the request failed and the
/// error in the response body could not be deserialized.
///
/// Returns an [`ErrorType::RequestCanceled`] error type if the request was
/// canceled by the user.
///
/// Returns an [`ErrorType::RequestError`] error type if creating the request
/// failed.
///
/// Returns an [`ErrorType::RequestTimedOut`] error type if the request timed
/// out. The timeout value is configured via [`ClientBuilder::timeout`].
///
/// Returns an [`ErrorType::Response`] error type if the request failed.
///
/// [`ClientBuilder::timeout`]: crate::client::ClientBuilder::timeout
/// [`ErrorType::Json`]: crate::error::ErrorType::Json
/// [`ErrorType::Parsing`]: crate::error::ErrorType::Parsing
/// [`ErrorType::RequestCanceled`]: crate::error::ErrorType::RequestCanceled
/// [`ErrorType::RequestError`]: crate::error::ErrorType::RequestError
/// [`ErrorType::RequestTimedOut`]: crate::error::ErrorType::RequestTimedOut
/// [`ErrorType::Response`]: crate::error::ErrorType::Response
/// [`Response`]: super::Response
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct ResponseFuture<T>(Result<Inner<T>, Ready<Error>>);

impl<T> ResponseFuture<T> {
    pub(crate) fn new(
        client: HyperClient<Connector, Full<Bytes>>,
        invalid_token: Option<Arc<AtomicBool>>,
        request: Request<Full<Bytes>>,
        span: tracing::Span,
        timeout: Duration,
        rate_limiter: Option<RateLimiter>,
        path: Path,
    ) -> Self {
        let permit_generator =
            rate_limiter.map(|rate_limiter| PermitFutureGenerator { rate_limiter, path });
        let response_generator = TimedResponseFutureGenerator {
            client,
            request,
            timeout,
        };
        let stage = permit_generator.as_ref().map_or_else(
            || ResponseStageFuture::Response {
                fut: response_generator.generate(),
                permit: None,
            },
            |gen| ResponseStageFuture::RateLimitPermit(gen.generate()),
        );
        Self(Ok(Inner {
            invalid_token,
            permit_generator,
            phantom: PhantomData,
            pre_flight_check: None,
            response_generator,
            span,
            stage,
        }))
    }

    /// Set a function to call after clearing the ratelimiter but prior to
    /// sending the request to determine if the request is still valid.
    ///
    /// This function will be a no-op if the request has failed, has already
    /// passed the ratelimiter, or if there is no ratelimiter configured.
    ///
    /// Returns whether the pre flight function was set.
    ///
    /// # Examples
    ///
    /// Delete a message, but immediately before sending the request check if
    /// the request should still be sent:
    ///
    /// ```no_run
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use std::{
    ///     collections::HashSet,
    ///     env,
    ///     future::IntoFuture,
    ///     sync::{Arc, Mutex},
    /// };
    /// use twilight_http::{error::ErrorType, Client};
    /// use twilight_model::id::Id;
    ///
    /// let channel_id = Id::new(1);
    /// let message_id = Id::new(2);
    ///
    /// let channels_ignored = {
    ///     let mut map = HashSet::new();
    ///     map.insert(channel_id);
    ///
    ///     Arc::new(Mutex::new(map))
    /// };
    ///
    /// let client = Client::new(env::var("DISCORD_TOKEN")?);
    /// let mut req = client.delete_message(channel_id, message_id).into_future();
    ///
    /// let channels_ignored_clone = channels_ignored.clone();
    /// req.set_pre_flight(move || {
    ///     // imagine you have some logic here to external state that checks
    ///     // whether the request should still be performed
    ///     let channels_ignored = channels_ignored_clone.lock().expect("channels poisoned");
    ///
    ///     !channels_ignored.contains(&channel_id)
    /// });
    ///
    /// // the pre-flight check will cancel the request
    /// assert!(matches!(
    ///     req.await.unwrap_err().kind(),
    ///     ErrorType::RequestCanceled,
    /// ));
    /// # Ok(()) }
    /// ```
    pub fn set_pre_flight<P>(&mut self, predicate: P) -> bool
    where
        P: Fn() -> bool + Send + 'static,
    {
        if let Ok(inner) = &mut self.0 {
            if inner.permit_generator.is_some() && inner.pre_flight_check.is_none() {
                inner.pre_flight_check = Some(Box::new(predicate));
                return true;
            }
        }
        false
    }

    /// Creates a future that is immediately ready with an error.
    pub(crate) fn error(source: Error) -> Self {
        Self(Err(ready(source)))
    }
}

impl<T: Unpin> Future for ResponseFuture<T> {
    type Output = Result<Response<T>, Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let inner = match &mut self.0 {
            Ok(inner) => inner,
            Err(err) => return Pin::new(err).poll(cx).map(Err),
        };

        let _entered = inner.span.enter();

        loop {
            match &mut inner.stage {
                ResponseStageFuture::Error { fut, status } => {
                    let body = ready!(Pin::new(fut).poll(cx)).map_err(|source| Error {
                        kind: ErrorType::RequestError,
                        source: Some(Box::new(source)),
                    })?;

                    return Poll::Ready(Err(match crate::json::from_bytes::<ApiError>(&body) {
                        Ok(error) => Error {
                            kind: ErrorType::Response {
                                body,
                                error,
                                status: super::StatusCode::new(status.as_u16()),
                            },
                            source: None,
                        },
                        Err(source) => Error {
                            kind: ErrorType::Parsing { body },
                            source: Some(Box::new(source)),
                        },
                    }));
                }
                ResponseStageFuture::RateLimitPermit(fut) => {
                    let permit = ready!(Pin::new(fut).poll(cx));
                    if inner
                        .pre_flight_check
                        .as_ref()
                        .is_some_and(|check| !check())
                    {
                        return Poll::Ready(Err(Error {
                            kind: ErrorType::RequestCanceled,
                            source: None,
                        }));
                    }

                    inner.stage = ResponseStageFuture::Response {
                        fut: inner.response_generator.generate(),
                        permit: Some(permit),
                    };
                }
                ResponseStageFuture::Response { fut, permit } => {
                    let response = ready!(Pin::new(fut).poll(cx))
                        .map_err(|source| Error {
                            kind: ErrorType::RequestTimedOut,
                            source: Some(Box::new(source)),
                        })?
                        .map_err(|source| Error {
                            kind: ErrorType::RequestError,
                            source: Some(Box::new(source)),
                        })?;

                    if response.status() == StatusCode::UNAUTHORIZED {
                        if let Some(invalid) = &inner.invalid_token {
                            invalid.store(true, Ordering::Relaxed);
                        }
                    }

                    if let Some(permit) = permit.take() {
                        match parse_ratelimit_headers(response.headers()) {
                            Ok(v) => permit.complete(v),
                            Err(source) => {
                                tracing::warn!("header parsing failed: {source}; {response:?}");

                                permit.complete(None);
                            }
                        }
                    }

                    if response.status().is_success() {
                        #[cfg(feature = "decompression")]
                        let mut response = response;
                        // Inaccurate since end-users can only access the decompressed body.
                        #[cfg(feature = "decompression")]
                        response.headers_mut().remove(header::CONTENT_LENGTH);

                        return Poll::Ready(Ok(Response::new(response)));
                    } else if response.status() == StatusCode::TOO_MANY_REQUESTS {
                        inner.stage = match &inner.permit_generator {
                            Some(gen) => ResponseStageFuture::RateLimitPermit(gen.generate()),
                            None => ResponseStageFuture::Response {
                                fut: inner.response_generator.generate(),
                                permit: None,
                            },
                        };
                    } else {
                        inner.stage = ResponseStageFuture::Error {
                            status: response.status(),
                            fut: Response::<()>::new(response).bytes(),
                        };
                    }
                }
            }
        }
    }
}

/// Internal response future fields.
struct Inner<T> {
    /// Whether the client's token is invalidated.
    invalid_token: Option<Arc<AtomicBool>>,
    /// Optional [`PermitFuture`] generator, if registered.
    permit_generator: Option<PermitFutureGenerator>,
    phantom: PhantomData<T>,
    /// Predicate to check after completing [`ResponseFutureStage::Permit`].
    pre_flight_check: Option<Box<dyn Fn() -> bool + Send + 'static>>,
    /// [`Timeout<HyperResponseFuture>`] generator.
    response_generator: TimedResponseFutureGenerator,
    /// This future's span.
    span: tracing::Span,
    /// This future's current stage.
    stage: ResponseStageFuture,
}
