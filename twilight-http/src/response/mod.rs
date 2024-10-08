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
use http::{
    header::{HeaderValue, Iter as HeaderMapIter},
    Response as HyperResponse,
};
use http_body_util::BodyExt;
use hyper::body::{Bytes, Incoming};
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
    inner: HyperResponse<Incoming>,
    phantom: PhantomData<T>,
}

impl<T> Response<T> {
    pub(crate) const fn new(inner: HyperResponse<Incoming>) -> Self {
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
            .get(http::header::CONTENT_ENCODING)
            .is_some();

        let body = self.inner.into_body();

        let fut = async move {
            {
                #[cfg(feature = "decompression")]
                if compressed {
                    return decompress(body).await;
                }

                Ok(body
                    .collect()
                    .await
                    .map_err(|source| DeserializeBodyError {
                        kind: DeserializeBodyErrorType::Chunking,
                        source: Some(Box::new(source)),
                    })?
                    .to_bytes())
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
//      .content("test")
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
            Poll::Ready(result.map(|b| b.to_vec()))
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
            Poll::Ready(Ok(bytes)) => {
                Poll::Ready(crate::json::from_bytes(&bytes).map_err(|source| {
                    DeserializeBodyError {
                        kind: DeserializeBodyErrorType::Deserializing,
                        source: Some(Box::new(source)),
                    }
                }))
            }
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
                let utf8_error = source.utf8_error();
                let bytes = source.into_bytes();

                DeserializeBodyError {
                    kind: DeserializeBodyErrorType::BodyNotUtf8 { bytes },
                    source: Some(Box::new(utf8_error)),
                }
            })),
            Poll::Ready(Err(source)) => Poll::Ready(Err(source)),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[cfg(feature = "decompression")]
async fn decompress<B: hyper::body::Body>(body: B) -> Result<Bytes, DeserializeBodyError>
where
    <B as hyper::body::Body>::Error: Send + Sync + Error + 'static,
{
    use brotli_decompressor::Decompressor;
    use hyper::body::Buf;
    use std::io::Read;

    let aggregate = body
        .collect()
        .await
        .map_err(|source| DeserializeBodyError {
            kind: DeserializeBodyErrorType::Chunking,
            source: Some(Box::new(source)),
        })?
        .aggregate();

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
        use http_body_util::Full;
        use twilight_model::guild::invite::Invite;

        const COMPRESSED: [u8; 553] = [
            27, 235, 4, 0, 44, 10, 99, 99, 102, 244, 145, 235, 87, 95, 83, 76, 203, 31, 27, 6, 65,
            20, 107, 75, 245, 103, 243, 139, 3, 81, 204, 15, 49, 13, 177, 83, 150, 163, 53, 249,
            217, 44, 58, 93, 125, 117, 56, 81, 249, 9, 5, 129, 64, 112, 146, 109, 175, 185, 252,
            39, 174, 169, 143, 248, 160, 111, 79, 250, 15, 22, 21, 139, 72, 171, 182, 215, 97, 1,
            109, 52, 192, 105, 131, 236, 70, 240, 211, 16, 175, 237, 1, 164, 242, 21, 250, 7, 182,
            87, 200, 84, 121, 177, 139, 184, 62, 86, 239, 221, 212, 206, 23, 176, 184, 173, 182,
            83, 250, 176, 218, 222, 73, 192, 165, 108, 20, 233, 138, 102, 8, 186, 0, 34, 79, 212,
            190, 139, 237, 164, 11, 13, 236, 223, 90, 18, 161, 105, 219, 189, 211, 233, 56, 100,
            27, 53, 61, 230, 220, 103, 22, 220, 157, 206, 198, 33, 124, 46, 160, 49, 72, 66, 109,
            130, 156, 126, 25, 231, 164, 31, 17, 102, 112, 78, 240, 195, 215, 22, 58, 199, 29, 244,
            246, 17, 248, 182, 159, 244, 231, 2, 187, 178, 212, 133, 198, 226, 154, 196, 194, 109,
            105, 237, 98, 73, 70, 73, 174, 133, 214, 16, 22, 165, 73, 132, 37, 25, 78, 185, 13, 20,
            226, 205, 111, 76, 80, 87, 156, 171, 130, 243, 102, 245, 66, 54, 21, 241, 150, 144,
            113, 204, 11, 45, 205, 147, 31, 35, 223, 39, 159, 14, 134, 11, 233, 90, 91, 234, 149,
            220, 63, 225, 191, 155, 78, 23, 26, 233, 239, 12, 87, 75, 185, 112, 53, 5, 218, 162,
            88, 143, 73, 163, 240, 198, 80, 106, 205, 225, 201, 11, 211, 102, 187, 59, 131, 4, 18,
            68, 104, 61, 114, 222, 250, 243, 104, 191, 186, 190, 228, 118, 222, 138, 144, 82, 50,
            65, 20, 233, 128, 139, 237, 52, 175, 75, 228, 168, 57, 75, 2, 210, 98, 28, 86, 21, 106,
            108, 25, 67, 189, 94, 185, 253, 174, 74, 73, 20, 161, 213, 76, 117, 19, 241, 59, 175,
            156, 167, 74, 184, 148, 214, 21, 90, 95, 105, 76, 80, 157, 146, 182, 184, 240, 89, 31,
            94, 80, 68, 218, 177, 126, 147, 26, 184, 109, 211, 32, 123, 49, 11, 120, 16, 190, 124,
            255, 23, 39, 117, 103, 82, 62, 214, 102, 187, 195, 122, 245, 115, 31, 4, 29, 84, 181,
            80, 204, 22, 61, 140, 159, 161, 228, 241, 229, 231, 219, 229, 202, 193, 72, 193, 139,
            151, 179, 135, 40, 217, 140, 251, 3, 18, 106, 142, 249, 255, 73, 62, 156, 133, 5, 28,
            112, 57, 94, 73, 161, 245, 238, 26, 20, 197, 81, 11, 225, 137, 62, 144, 221, 198, 148,
            35, 107, 194, 189, 8, 41, 125, 129, 244, 238, 35, 213, 254, 254, 246, 176, 184, 172,
            112, 85, 54, 235, 239, 79, 250, 151, 27, 34, 79, 149, 124, 0, 103, 230, 132, 251, 122,
            82, 46, 52, 132, 228, 234, 159, 186, 221, 203, 94, 0, 236, 182, 125, 236, 47, 243, 7,
            38, 9, 241, 2, 45, 199, 19, 230, 15, 178, 197, 116, 37, 88, 0, 215, 103, 13, 104, 114,
            248, 15, 240, 7,
        ];

        let decompressed = decompress(Full::new(COMPRESSED.as_slice())).await?;

        let deserialized = serde_json::from_slice::<Invite>(&decompressed)?;

        assert_eq!(deserialized.code, "twilight-rs");

        Ok(())
    }
}
