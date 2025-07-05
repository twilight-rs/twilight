use super::{BytesFuture, Response};
use crate::{
    api_error::ApiError,
    client::connector::Connector,
    error::{Error, ErrorType},
};
use http::{header, HeaderMap, Request, StatusCode};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper_util::client::legacy::{Client, ResponseFuture as HyperResponseFuture};
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
use tokio::time::{self, Sleep, Timeout};
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
    let bucket = headers.get(RateLimitHeaders::BUCKET);
    let limit = headers.get(RateLimitHeaders::LIMIT);
    let remaining = headers.get(RateLimitHeaders::REMAINING);
    let reset_after = headers.get(RateLimitHeaders::RESET_AFTER);

    if bucket.is_none() && limit.is_none() && remaining.is_none() && reset_after.is_none() {
        return Ok(None);
    }

    let bucket = bucket.ok_or("missing bucket header")?.as_bytes().to_vec();
    let limit = limit.ok_or("missing limit header")?.to_str()?.parse()?;
    let remaining = remaining
        .ok_or("missing remaining header")?
        .to_str()?
        .parse()?;
    let reset_after = reset_after
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

enum ResponseFutureStage {
    Chunking {
        fut: BytesFuture,
        status: StatusCode,
    },
    Delay(Pin<Box<Sleep>>),
    Permit(PermitFuture),
    Response {
        fut: Pin<Box<Timeout<HyperResponseFuture>>>,
        permit: Option<Permit>,
    },
}

struct PermitFutureGenerator {
    rate_limiter: RateLimiter,
    path: Path,
}

impl PermitFutureGenerator {
    fn generate(&self) -> PermitFuture {
        self.rate_limiter.acquire(self.path.clone())
    }
}

struct ResponseFutureGenerator {
    client: Arc<Client<Connector, Full<Bytes>>>,
    request: Request<Full<Bytes>>,
    timeout: Duration,
}

impl ResponseFutureGenerator {
    fn generate(&self) -> Pin<Box<Timeout<HyperResponseFuture>>> {
        Box::pin(time::timeout(
            self.timeout,
            self.client.request(self.request.clone()),
        ))
    }
}

/// Future that will resolve to a [`Response`].
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

struct Inner<T> {
    invalid_token: Option<Arc<AtomicBool>>,
    permit_generator: Option<PermitFutureGenerator>,
    phantom: PhantomData<T>,
    pre_flight_check: Option<Box<dyn Fn() -> bool + Send + 'static>>,
    response_generator: ResponseFutureGenerator,
    stage: ResponseFutureStage,
}

impl<T> Inner<T> {
    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Result<Response<T>, Error>> {
        loop {
            match &mut self.stage {
                ResponseFutureStage::Chunking { fut, status } => {
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
                ResponseFutureStage::Delay(fut) => {
                    ready!(Pin::new(fut).poll(cx));
                    self.stage = match &self.permit_generator {
                        Some(gen) => ResponseFutureStage::Permit(gen.generate()),
                        None => ResponseFutureStage::Response {
                            fut: self.response_generator.generate(),
                            permit: None,
                        },
                    };
                }
                ResponseFutureStage::Permit(fut) => {
                    let permit = ready!(Pin::new(fut).poll(cx));
                    if self.pre_flight_check.as_ref().is_some_and(|check| !check()) {
                        return Poll::Ready(Err(Error {
                            kind: ErrorType::RequestCanceled,
                            source: None,
                        }));
                    }

                    self.stage = ResponseFutureStage::Response {
                        fut: self.response_generator.generate(),
                        permit: Some(permit),
                    };
                }
                ResponseFutureStage::Response { fut, permit } => {
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
                        if let Some(invalid) = &self.invalid_token {
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
                    }

                    if response.status() == StatusCode::TOO_MANY_REQUESTS {
                        if let Some(retry_after) = response.headers().get(header::RETRY_AFTER) {
                            if let Ok(str) = retry_after.to_str() {
                                if let Ok(secs) = str.parse() {
                                    let duration = Duration::from_secs(secs);
                                    tracing::debug!(?duration, "retrying request");
                                    self.stage =
                                        ResponseFutureStage::Delay(Box::pin(time::sleep(duration)));
                                    continue;
                                }
                            }
                            tracing::warn!("unknown retry-after header value: {retry_after:?}");
                        }
                    }

                    self.stage = ResponseFutureStage::Chunking {
                        status: response.status(),
                        fut: Response::<()>::new(response).bytes(),
                    }
                }
            }
        }
    }
}

impl<T> ResponseFuture<T> {
    pub(crate) fn new(
        client: Arc<Client<Connector, Full<Bytes>>>,
        invalid_token: Option<Arc<AtomicBool>>,
        request: Request<Full<Bytes>>,
        timeout: Duration,
    ) -> Self {
        let response_generator = ResponseFutureGenerator {
            client,
            request,
            timeout,
        };
        Self(Ok(Inner {
            invalid_token,
            permit_generator: None,
            phantom: PhantomData,
            pre_flight_check: None,
            stage: ResponseFutureStage::Response {
                fut: response_generator.generate(),
                permit: None,
            },
            response_generator,
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
        let Ok(inner) = &mut self.0 else { return false };
        inner.pre_flight_check = Some(Box::new(predicate));

        true
    }

    pub(crate) fn error(source: Error) -> Self {
        Self(Err(ready(source)))
    }

    #[track_caller]
    pub(crate) fn set_rate_limiter(&mut self, rate_limiter: RateLimiter, path: Path) {
        let Ok(inner) = &mut self.0 else {
            panic!("tried setting rate limiter on error variant");
        };
        let permit_generator = PermitFutureGenerator { rate_limiter, path };
        inner.stage = ResponseFutureStage::Permit(permit_generator.generate());
        inner.permit_generator = Some(permit_generator);
    }
}

impl<T: Unpin> Future for ResponseFuture<T> {
    type Output = Result<Response<T>, Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match &mut self.0 {
            Ok(inner) => inner.poll(cx),
            Err(err) => Pin::new(err).poll(cx).map(Err),
        }
    }
}
