//! Response utility type and related types.
//!
//! The heart of the response module is the [`Response`] itself: it's a wrapper
//! over the underlying HTTP client's response, containing helper methods for
//! things like [getting the raw bytes][`bytes`] of the response body, getting
//! an [iterator of the response headers][`headers`], or
//! [deserializing the body into a model][`model`].
//!
//! The [`ResponseFuture`] is a type implementing [`Future`] that resolves to a
//! [`Response`] when polled or `.await`ed to completion.
//!
//! # Examples
//!
//! Get a user by ID, check if the request was successful, and if so deserialize
//! the response body via [`Response::model`][`model`] and print the user's
//! name:
//!
//! ```no_run
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let user_id = twilight_model::id::Id::new(1);
//! use std::env;
//! use twilight_http::Client;
//!
//! let client = Client::new(env::var("DISCORD_TOKEN")?);
//! let response = client.user(user_id).await?;
//!
//! if !response.status().is_success() {
//!     println!("failed to get user");
//!
//!     return Ok(());
//! }
//!
//! // Twilight already knows to deserialize it into a
//! // `twilight_model::user::User`.
//! let user = response.model().await?;
//!
//! println!("user's name: {}:{}", user.name, user.discriminator);
//! # Ok(()) }
//! ```
//!
//! [`Future`]: std::future::Future
//! [`bytes`]: Response::bytes
//! [`headers`]: Response::headers
//! [`model`]: Response::model

pub mod marker;

pub(crate) mod future;

mod status_code;

pub use self::{future::ResponseFuture, status_code::StatusCode};

use self::marker::ListBody;
use hyper::{
    body::{self, Bytes},
    header::{HeaderValue, Iter as HeaderMapIter},
    Body, Response as HyperResponse,
};
use serde::de::DeserializeOwned;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    future::Future,
    iter::FusedIterator,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

/// Failure when processing a response body.
#[derive(Debug)]
pub struct DeserializeBodyError {
    kind: DeserializeBodyErrorType,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl DeserializeBodyError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &DeserializeBodyErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        DeserializeBodyErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, self.source)
    }
}

impl Display for DeserializeBodyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            &DeserializeBodyErrorType::BodyNotUtf8 { .. } => {
                f.write_str("response body is not a utf-8 valid string")
            }
            DeserializeBodyErrorType::Chunking { .. } => {
                f.write_str("failed to chunk response body")
            }
            #[cfg(feature = "decompression")]
            DeserializeBodyErrorType::Decompressing { .. } => {
                f.write_str("failed to decompress response body")
            }
            DeserializeBodyErrorType::Deserializing { .. } => {
                f.write_str("failed to deserialize response body")
            }
        }
    }
}

impl Error for DeserializeBodyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source
            .as_ref()
            .map(|source| &**source as &(dyn Error + 'static))
    }
}

/// Type of [`DeserializeBodyError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum DeserializeBodyErrorType {
    /// Response body is not a UTF-8 valid string.
    BodyNotUtf8 {
        /// Raw response body bytes that could not be converted into a UTF-8
        /// valid string.
        bytes: Vec<u8>,
    },
    /// Response body couldn't be chunked.
    Chunking,
    /// Decompressing the response failed.
    #[cfg(feature = "decompression")]
    Decompressing,
    /// Deserializing the model failed.
    Deserializing,
}

/// Response wrapper containing helper functions over the HTTP client's
/// response.
///
/// This exists so that it can be determined whether to deserialize the body.
/// This is useful when you don't need the body and therefore don't want to
/// spend the time to deserialize it.
///
/// # Examples
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let user_id = twilight_model::id::Id::new(1);
/// use std::env;
/// use twilight_http::Client;
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let response = client.user(user_id).await?;
/// println!("status code: {}", response.status());
///
/// let user = response.model().await?;
/// println!("username: {}#{:04}", user.name, user.discriminator);
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct Response<T> {
    inner: HyperResponse<Body>,
    phantom: PhantomData<T>,
}

impl<T> Response<T> {
    pub(crate) const fn new(inner: HyperResponse<Body>) -> Self {
        Self {
            inner,
            phantom: PhantomData,
        }
    }

    /// Iterator of the response headers.
    #[must_use = "creating an iterator of the headers has no use on its own"]
    pub fn headers(&self) -> HeaderIter<'_> {
        HeaderIter(self.inner.headers().iter())
    }

    /// Status code of the response.
    #[must_use = "retrieving the status code has no use on its own"]
    pub fn status(&self) -> StatusCode {
        // Convert the `hyper` status code into its raw form in order to return
        // our own.
        let raw = self.inner.status().as_u16();

        StatusCode::new(raw)
    }

    /// Consume the response and accumulate the chunked body into bytes.
    ///
    /// For a textual representation of the response body [`text`] should be
    /// preferred.
    ///
    /// # Examples
    ///
    /// Count the number of bytes in a response body:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let user_id = twilight_model::id::Id::new(1);
    /// use std::env;
    /// use twilight_http::Client;
    ///
    /// let client = Client::new(env::var("DISCORD_TOKEN")?);
    /// let response = client.user(user_id).await?;
    /// let bytes = response.bytes().await?;
    ///
    /// println!("size of body: {}", bytes.len());
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeBodyErrorType::Chunking`] error type if the
    /// response body could not be entirely read.
    ///
    /// [`text`]: Self::text
    pub fn bytes(self) -> BytesFuture {
        #[cfg(feature = "decompression")]
        let compressed = self
            .inner
            .headers()
            .get(hyper::header::CONTENT_ENCODING)
            .is_some();

        let body = self.inner.into_body();

        let fut = async move {
            {
                #[cfg(feature = "decompression")]
                if compressed {
                    return decompress(body).await;
                }

                body::to_bytes(body)
                    .await
                    .map_err(|source| DeserializeBodyError {
                        kind: DeserializeBodyErrorType::Chunking,
                        source: Some(Box::new(source)),
                    })
            }
        };

        BytesFuture {
            inner: Box::pin(fut),
        }
    }

    /// Consume the response and accumulate the body into a string.
    ///
    /// For the raw bytes of the response body [`bytes`] should be preferred.
    ///
    /// # Examples
    ///
    /// Print the textual response from getting the current user:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use std::env;
    /// use twilight_http::Client;
    ///
    /// let client = Client::new(env::var("DISCORD_TOKEN")?);
    /// let response = client.current_user().await?;
    /// let text = response.text().await?;
    ///
    /// println!("body: {text}");
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeBodyErrorType::BodyNotUtf8`] error type if the
    /// response body is not UTF-8 valid.
    ///
    /// Returns a [`DeserializeBodyErrorType::Chunking`] error type if the
    /// response body could not be entirely read.
    ///
    /// [`bytes`]: Self::bytes
    pub fn text(self) -> TextFuture {
        TextFuture(self.bytes())
    }
}

impl<T: DeserializeOwned> Response<T> {
    /// Consume the response, chunking the body and then deserializing it into
    /// the request's matching model.
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeBodyErrorType::Chunking`] error type if the
    /// response body could not be entirely read.
    ///
    /// Returns a [`DeserializeBodyErrorType::Deserializing`] error type if the
    /// response body could not be deserialized into the target model.
    pub fn model(self) -> ModelFuture<T> {
        ModelFuture::new(self.bytes())
    }
}

impl<T: DeserializeOwned> Response<ListBody<T>> {
    /// Consume the response, chunking the body and then deserializing it into
    /// a list of something.
    ///
    /// This is an alias for [`models`].
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeBodyErrorType::Chunking`] error type if the
    /// response body could not be entirely read.
    ///
    /// Returns a [`DeserializeBodyErrorType::Deserializing`] error type if the
    /// response body could not be deserialized into a list of something.
    ///
    /// [`models`]: Self::models
    pub fn model(self) -> ModelFuture<Vec<T>> {
        self.models()
    }

    /// Consume the response, chunking the body and then deserializing it into
    /// a list of something.
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeBodyErrorType::Chunking`] error type if the
    /// response body could not be entirely read.
    ///
    /// Returns a [`DeserializeBodyErrorType::Deserializing`] error type if the
    /// response body could not be deserialized into a list of something.
    pub fn models(self) -> ModelFuture<Vec<T>> {
        Response::<Vec<T>>::new(self.inner).model()
    }
}

/// Iterator over the headers of a [`Response`].
///
/// Header names are returned as a string slice and header values are returned
/// as a slice of bytes. If a header has multiple values then the same header
/// name may be returned multiple times.
///
/// Obtained via [`Response::headers`].
///
/// # Examples
///
/// Iterate over all of the header names and values of the response from
/// creating a message:
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let channel_id = twilight_model::id::Id::new(1);
/// use std::env;
/// use twilight_http::Client;
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let response = client.create_message(channel_id)
//      .content("test")?
///     .await?;
/// let mut headers = response.headers();
///
/// while let Some((name, value)) = headers.next() {
///     println!("{name}: {}", String::from_utf8_lossy(value));
/// }
/// # Ok(()) }
/// ```
#[derive(Debug)]
#[must_use = "iterators do nothing unless used"]
pub struct HeaderIter<'a>(HeaderMapIter<'a, HeaderValue>);

// `hyper::header::Iter` implements `FusedIterator` so this is a free impl.
impl FusedIterator for HeaderIter<'_> {}

impl<'a> Iterator for HeaderIter<'a> {
    // Header names are UTF-8 valid but values aren't, so the value item has
    // to be a slice of the bytes.
    type Item = (&'a str, &'a [u8]);

    fn next(&mut self) -> Option<Self::Item> {
        let (name, value) = self.0.next()?;

        Some((name.as_str(), value.as_bytes()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // `hyper::header::Iter` implements `Iterator::size_hint`.
        self.0.size_hint()
    }
}

/// Future resolving to the bytes of a response body.
///
/// The body of the response is chunked and aggregated into a `Vec` of bytes.
///
/// Obtained via [`Response::bytes`].
///
/// # Examples
///
/// Print the bytes of the body of the response from creating a message:
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let channel_id = twilight_model::id::Id::new(1);
/// # let message_id = twilight_model::id::Id::new(2);
/// use std::env;
/// use twilight_http::Client;
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let response = client.message(channel_id, message_id).await?;
/// let bytes = response.bytes().await?;
///
/// println!("bytes of the body: {bytes:?}");
/// # Ok(()) }
/// ```
///
/// # Errors
///
/// Returns a [`DeserializeBodyErrorType::Chunking`] error type if the
/// response body could not be entirely read.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct BytesFuture {
    inner:
        Pin<Box<dyn Future<Output = Result<Bytes, DeserializeBodyError>> + Send + Sync + 'static>>,
}

impl Future for BytesFuture {
    type Output = Result<Vec<u8>, DeserializeBodyError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Poll::Ready(result) = Pin::new(&mut self.inner).poll(cx) {
            Poll::Ready(result.map(|b| b.into_iter().collect()))
        } else {
            Poll::Pending
        }
    }
}

/// Future resolving to a deserialized model.
///
/// Obtained via [`Response::model`].
///
/// # Examples
///
/// Get an emoji by its ID and print its name:
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let guild_id = twilight_model::id::Id::new(1);
/// # let emoji_id = twilight_model::id::Id::new(2);
/// use std::env;
/// use twilight_http::Client;
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let emoji = client.emoji(guild_id, emoji_id).await?.model().await?;
///
/// println!("emoji name: {}", emoji.name);
/// # Ok(()) }
/// ```
///
/// # Errors
///
/// Returns a [`DeserializeBodyErrorType::Chunking`] error type if the
/// response body could not be entirely read.
///
/// Returns a [`DeserializeBodyErrorType::Deserializing`] error type if the
/// response body could not be deserialized into a model.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct ModelFuture<T> {
    future: BytesFuture,
    phantom: PhantomData<T>,
}

impl<T> ModelFuture<T> {
    const fn new(bytes: BytesFuture) -> Self {
        Self {
            future: bytes,
            phantom: PhantomData,
        }
    }
}

impl<T: DeserializeOwned + Unpin> Future for ModelFuture<T> {
    type Output = Result<T, DeserializeBodyError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.future).poll(cx) {
            Poll::Ready(Ok(bytes)) => Poll::Ready(
                crate::json::from_bytes(&Bytes::from(bytes)).map_err(|source| {
                    DeserializeBodyError {
                        kind: DeserializeBodyErrorType::Deserializing,
                        source: Some(Box::new(source)),
                    }
                }),
            ),
            Poll::Ready(Err(source)) => Poll::Ready(Err(source)),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Future resolving to the text of a response body.
///
/// The body of the response is chunked and aggregated into a string.
///
/// Obtained via [`Response::text`].
///
/// # Examples
///
/// Print the textual body of the response from creating a message:
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let channel_id = twilight_model::id::Id::new(1);
/// # let message_id = twilight_model::id::Id::new(2);
/// use std::env;
/// use twilight_http::Client;
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let response = client.message(channel_id, message_id).await?;
/// let text = response.text().await?;
///
/// println!("body: {text}");
/// # Ok(()) }
/// ```
///
/// # Errors
///
/// Returns a [`DeserializeBodyErrorType::BodyNotUtf8`] error type if the
/// response body is not UTF-8 valid.
///
/// Returns a [`DeserializeBodyErrorType::Chunking`] error type if the
/// response body could not be entirely read.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct TextFuture(BytesFuture);

impl Future for TextFuture {
    type Output = Result<String, DeserializeBodyError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.0).poll(cx) {
            Poll::Ready(Ok(bytes)) => Poll::Ready(String::from_utf8(bytes).map_err(|source| {
                // This is a very cold path. Converting a response body to a
                // UTF-8 valid string should basically never fail anyway; it's
                // worth it to have the context readily available for the user.
                let copy = source.as_bytes().to_owned();

                DeserializeBodyError {
                    kind: DeserializeBodyErrorType::BodyNotUtf8 { bytes: copy },
                    source: Some(Box::new(source)),
                }
            })),
            Poll::Ready(Err(source)) => Poll::Ready(Err(source)),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(feature = "decompression")]
async fn decompress(body: Body) -> Result<Bytes, DeserializeBodyError> {
    use brotli::Decompressor;
    use hyper::body::Buf;
    use std::io::Read;

    let aggregate = body::aggregate(body)
        .await
        .map_err(|source| DeserializeBodyError {
            kind: DeserializeBodyErrorType::Chunking,
            source: Some(Box::new(source)),
        })?;

    // Determine the size of the entire buffer, in order to create the
    // decompressed and compressed buffers.
    let size = aggregate.remaining();

    let mut buf = Vec::with_capacity(size);

    Decompressor::new(aggregate.reader(), size)
        .read_to_end(&mut buf)
        .map_err(|_| DeserializeBodyError {
            kind: DeserializeBodyErrorType::Decompressing,
            source: None,
        })?;

    Ok(buf.into())
}

#[cfg(test)]
mod tests {
    use super::{
        marker::{EmptyBody, ListBody},
        BytesFuture, DeserializeBodyError, DeserializeBodyErrorType, HeaderIter, ModelFuture,
        Response, TextFuture,
    };
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, future::Future, iter::FusedIterator};
    use twilight_model::{channel::Message, guild::Emoji};

    #[cfg(feature = "decompression")]
    use std::error::Error;

    assert_impl_all!(BytesFuture: Future);
    assert_impl_all!(DeserializeBodyErrorType: Debug, Send, Sync);
    assert_impl_all!(DeserializeBodyError: Debug, Send, Sync);
    assert_impl_all!(HeaderIter<'_>: Debug, FusedIterator, Iterator, Send, Sync);
    assert_impl_all!(ModelFuture<Emoji>: Future);
    assert_impl_all!(Response<EmptyBody>: Debug, Send, Sync);
    assert_impl_all!(Response<ListBody<Message>>: Debug, Send, Sync);
    assert_impl_all!(TextFuture: Future);

    #[cfg(feature = "decompression")]
    #[tokio::test]
    async fn test_decompression() -> Result<(), Box<dyn Error + Send + Sync>> {
        use super::decompress;
        use hyper::Body;
        use twilight_model::guild::invite::Invite;

        const COMPRESSED: [u8; 685] = [
            3, 160, 2, 0, 228, 178, 169, 189, 190, 59, 251, 86, 18, 36, 232, 63, 98, 235, 98, 82,
            176, 41, 41, 85, 35, 35, 17, 115, 161, 13, 136, 164, 79, 156, 11, 128, 135, 156, 152,
            152, 180, 129, 13, 171, 193, 102, 195, 157, 71, 218, 254, 219, 18, 240, 113, 244, 96,
            217, 32, 111, 115, 13, 223, 109, 125, 34, 131, 147, 3, 127, 186, 220, 156, 31, 172, 6,
            165, 49, 2, 52, 31, 77, 219, 188, 30, 204, 209, 52, 35, 1, 124, 93, 154, 86, 97, 4, 63,
            216, 40, 140, 0, 125, 199, 13, 172, 48, 176, 29, 26, 216, 190, 107, 7, 44, 64, 2, 216,
            203, 78, 99, 4, 88, 126, 24, 232, 0, 9, 224, 60, 182, 114, 62, 96, 4, 24, 42, 223, 218,
            75, 165, 246, 161, 37, 109, 47, 244, 67, 139, 186, 178, 210, 129, 148, 74, 239, 67,
            166, 145, 0, 46, 112, 24, 16, 65, 191, 180, 45, 1, 84, 155, 245, 199, 70, 211, 12, 253,
            11, 170, 169, 52, 235, 65, 43, 8, 107, 219, 213, 174, 178, 36, 11, 234, 218, 9, 153,
            47, 253, 58, 96, 174, 242, 105, 237, 208, 10, 9, 224, 232, 58, 198, 8, 30, 177, 220,
            228, 156, 139, 117, 124, 47, 202, 77, 206, 227, 181, 136, 243, 213, 38, 185, 225, 72,
            0, 111, 249, 229, 106, 123, 197, 69, 177, 202, 57, 79, 5, 79, 227, 211, 75, 190, 70, 2,
            152, 242, 219, 111, 166, 236, 120, 126, 149, 20, 69, 178, 77, 11, 36, 96, 51, 20, 175,
            139, 251, 109, 87, 252, 234, 148, 231, 226, 134, 231, 201, 191, 100, 21, 151, 201, 54,
            21, 103, 113, 201, 47, 168, 3, 252, 44, 143, 211, 164, 188, 23, 215, 249, 165, 59, 208,
            248, 255, 36, 233, 77, 82, 114, 81, 236, 46, 227, 98, 131, 4, 48, 78, 147, 171, 184,
            228, 107, 145, 172, 182, 41, 18, 192, 93, 206, 111, 18, 126, 139, 245, 140, 213, 246,
            234, 234, 58, 77, 202, 123, 124, 38, 128, 57, 133, 137, 159, 125, 253, 33, 24, 129, 77,
            0, 223, 101, 223, 152, 47, 177, 76, 173, 32, 188, 240, 67, 183, 213, 208, 105, 113,
            124, 127, 144, 121, 3, 46, 10, 177, 88, 198, 113, 152, 204, 65, 26, 53, 27, 218, 43,
            58, 221, 27, 24, 106, 224, 223, 75, 78, 76, 201, 235, 106, 168, 246, 19, 32, 95, 102,
            3, 109, 179, 159, 228, 212, 232, 178, 95, 109, 221, 204, 213, 48, 41, 136, 119, 201,
            49, 28, 140, 25, 231, 232, 228, 36, 7, 3, 142, 97, 33, 211, 131, 4, 35, 120, 252, 65,
            141, 71, 228, 101, 59, 165, 52, 100, 212, 98, 148, 185, 174, 47, 70, 232, 233, 120,
            126, 115, 174, 62, 232, 118, 132, 143, 198, 28, 64, 203, 145, 0, 234, 110, 248, 223,
            152, 124, 65, 96, 51, 207, 243, 3, 230, 185, 142, 31, 184, 212, 182, 220, 94, 72, 178,
            55, 48, 45, 179, 193, 63, 2, 112, 182, 198, 168, 231, 250, 97, 232, 50, 47, 176, 100,
            72, 41, 219, 55, 144, 251, 97, 49, 129, 211, 5, 84, 227, 17, 181, 123, 141, 144, 122,
            142, 27, 184, 14, 245, 125, 47, 100, 30, 38, 36, 114, 214, 112, 109, 207, 102, 142,
            239, 91, 174, 19, 80, 230, 120, 34, 228, 90, 42, 154, 20, 95, 176, 140, 74, 26, 189,
            240, 44, 210, 14, 175, 179, 83, 112, 168, 112, 157, 26, 192, 216, 201, 167, 69, 5, 182,
            122, 90, 84, 165, 237, 240, 108, 205, 9, 188, 128, 134, 140, 81, 215, 113, 68, 114,
            161, 245, 8, 11, 73, 137, 158, 53, 102, 179, 236, 155, 29, 44, 203, 41, 15, 255, 158,
            255, 8, 96, 63, 215, 31, 24, 65, 45, 219, 89, 171, 208, 234, 16, 173, 63, 2, 65, 179,
            58, 215, 1, 108, 54, 67, 211, 215, 3, 18, 64, 243, 53, 106, 140, 192, 250, 251, 51, 0,
            3,
        ];

        let decompressed = decompress(Body::from(COMPRESSED.as_ref())).await?;

        let deserialized = serde_json::from_slice::<Invite>(&decompressed)?;

        assert_eq!(deserialized.code, "twilight-rs");

        Ok(())
    }
}
