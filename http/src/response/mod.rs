//! Response utility type and related types.
//!
//! The heart of the response module is the [`Response`] itself: it's a wrapper
//! over the underlying HTTP client's response, containing helper methods for
//! things like [getting the raw bytes][`bytes`] of the response body,
//! [deserializing the body into a model][`model`], or
//! [creating a lazily deserializing iterator][`iter`] over a list of models in
//! the response body.
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
//! # let user_id = twilight_model::id::UserId(1);
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
//! Get a list of the guild's roles and lazily iterate and deserialize them via
//! [`Response::iter`][`iter`], printing their name:
//!
//! ```no_run
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let guild_id = twilight_model::id::GuildId(1);
//! use std::env;
//! use twilight_http::Client;
//!
//! let client = Client::new(env::var("DISCORD_TOKEN")?);
//! let response = client.roles(guild_id).await?;
//!
//! // Create an iterator over the roles. Each role in the response body is only
//! // deserialized as the iterator advances.
//! let mut roles = response.iter().await?;
//!
//! while let Some(maybe_role) = roles.next() {
//!     // The role may not have deserialized properly, so we need to check.
//!     match maybe_role {
//!         Ok(role) => println!("role {}'s name: {}", role.id, role.name),
//!         Err(source) => println!("failed to deserialize role: {:?}", source),
//!     }
//! }
//! # Ok(()) }
//! ```
//!
//! [`bytes`]: Response::bytes
//! [`iter`]: Response::iter
//! [`model`]: Response::model

pub mod iter;
pub mod marker;

mod status_code;

pub use self::{
    iter::{MemberIter, ModelIter},
    status_code::StatusCode,
};

use self::marker::{ListBody, MemberBody, MemberListBody};
use super::json::JsonDeserializer;
use hyper::{Body, Error as HyperError, Response as HyperResponse, body::{self, Buf, Bytes}, header::{HeaderValue, Iter as HeaderMapIter}};
use serde::de::{DeserializeOwned, DeserializeSeed};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    future::Future,
    iter::{self as std_iter, FusedIterator},
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use twilight_model::{
    guild::member::{Member, MemberDeserializer, MemberListDeserializer},
    id::GuildId,
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
/// # let user_id = twilight_model::id::UserId(1);
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
    guild_id: Option<GuildId>,
    inner: HyperResponse<Body>,
    phantom: PhantomData<T>,
}

impl<T> Response<T> {
    pub(crate) const fn new(inner: HyperResponse<Body>) -> Self {
        Self {
            guild_id: None,
            inner,
            phantom: PhantomData,
        }
    }

    /// Iterator of the response headers.
    pub fn headers(&self) -> HeaderIter<'_> {
        HeaderIter(self.inner.headers().iter())
    }

    /// Status code of the response.
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
    /// # let user_id = twilight_model::id::UserId(1);
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
        let body = self.inner.into_body();

        // We need to do additional processing here due to the return type of
        // `body::aggregate` being `impl Buf`.
        let fut = async {
            let mut aggregate = body::aggregate(body).await?;

            // Create a buffer filled with zeroes so we can copy the aggregate
            // body into it.
            let mut buf = std_iter::repeat(0)
                .take(aggregate.remaining())
                .collect::<Vec<_>>();
            aggregate.copy_to_slice(&mut buf);

            Ok(buf)
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
    /// println!("body: {}", text);
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

    /// Set the ID of the relevant guild.
    ///
    /// Necessary for [`MemberBody`] and [`MemberListBody`] deserialization.
    pub(crate) fn set_guild_id(&mut self, guild_id: GuildId) {
        self.guild_id.replace(guild_id);
    }

    /// ID of the configured guild.
    ///
    /// # Panics
    ///
    /// Panics if the guild ID hasn't been configured.
    fn guild_id(&self) -> GuildId {
        self.guild_id.expect("guild id has not been configured")
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
    /// If performance is a concern then you may prefer [`iter`] as it is lazily
    /// deserializing and can short circuit.
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeBodyErrorType::Chunking`] error type if the
    /// response body could not be entirely read.
    ///
    /// Returns a [`DeserializeBodyErrorType::Deserializing`] error type if the
    /// response body could not be deserialized into a list of something.
    ///
    /// [`iter`]: Self::<ListBody<T>>::iter
    pub fn models(self) -> ModelFuture<Vec<T>> {
        Response::<Vec<T>>::new(self.inner).model()
    }

    /// Consume the response and create a lazily deserializing iterator over the
    /// response body.
    ///
    /// This has a slight performance advantage over [`models`] as each entry in
    /// the list is lazily deserialized; entries are deserialized on-demand as
    /// they are iterated over.
    ///
    /// # Examples
    ///
    /// Get the pins of a channel and lazily iterate over the messages:
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let channel_id = twilight_model::id::ChannelId(1);
    /// use std::env;
    /// use twilight_http::Client;
    ///
    /// let client = Client::new(env::var("DISCORD_TOKEN")?);
    /// let response = client.pins(channel_id).await?;
    ///
    /// let mut messages = response.iter().await?;
    ///
    /// while let Some(message) = messages.next() {
    ///     match message {
    ///         Ok(message) => println!("message content: {}", message.content),
    ///         Err(source) => println!("failed to deserialize: {:?}", source),
    ///     }
    /// }
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeBodyErrorType::Chunking`] error type if the
    /// response body could not be entirely read.
    ///
    /// [`models`]: Self::<ListBody<T>>::models
    pub fn iter(self) -> ModelIterFuture<T> {
        ModelIterFuture::new(self.bytes())
    }
}

impl Response<MemberBody> {
    /// Consume the response, chunking the body and then deserializing it into
    /// a member.
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeBodyErrorType::Chunking`] error type if the
    /// response body could not be entirely read.
    ///
    /// Returns a [`DeserializeBodyErrorType::Deserializing`] error type if the
    /// response body could not be deserialized into a member.
    pub fn model(self) -> MemberFuture {
        let guild_id = self.guild_id();

        MemberFuture::new(self.bytes(), guild_id)
    }
}

impl Response<MemberListBody> {
    /// Consume the response, chunking the body and then deserializing it into
    /// a list of members.
    ///
    /// If performance is a concern then you may prefer [`iter`] as it is lazily
    /// deserializing and can short circuit.
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeBodyErrorType::Chunking`] error type if the
    /// response body could not be entirely read.
    ///
    /// Returns a [`DeserializeBodyErrorType::Deserializing`] error type if the
    /// response body could not be deserialized into a list of members.
    ///
    /// [`iter`]: Self::<MemberListBody>::iter
    pub fn models(self) -> MemberListFuture {
        let guild_id = self.guild_id();

        MemberListFuture::new(self.bytes(), guild_id)
    }

    /// Consume the response and create a lazily deserializing iterator over the
    /// response body's models.
    ///
    /// This has a slight performance advantage over [`models`] as each member
    /// in the list is lazily deserialized; members are deserialized on-demand
    /// as they are iterated over.
    ///
    /// # Examples
    ///
    /// Search up to 500 members of a guild that start with the name "te":
    ///
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let guild_id = twilight_model::id::GuildId(1);
    /// use std::env;
    /// use twilight_http::Client;
    ///
    /// let client = Client::new(env::var("DISCORD_TOKEN")?);
    /// let response = client.search_guild_members(guild_id, "te")
    ///     .limit(500)?
    ///     .await?;
    ///
    /// let mut members = response.iter().await?;
    ///
    /// while let Some(Ok(member)) = members.next() {
    ///     println!(
    ///         "user {}#{} has nick: {:?}",
    ///         member.user.name,
    ///         member.user.discriminator,
    ///         member.nick,
    ///     );
    /// }
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`DeserializeBodyErrorType::Chunking`] error type if the
    /// response body could not be entirely read.
    ///
    /// [`models`]: Self::<MemberListBody>::models
    pub fn iter(self) -> MemberIterFuture {
        let guild_id = self.guild_id();

        MemberIterFuture::new(self.bytes(), guild_id)
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
/// # let channel_id = twilight_model::id::ChannelId(1);
/// use std::env;
/// use twilight_http::Client;
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let response = client.create_message(channel_id).content("test")?.await?;
/// let mut headers = response.headers();
///
/// while let Some((name, value)) = headers.next() {
///     println!("{}: {}", name, String::from_utf8_lossy(value));
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
/// # let channel_id = twilight_model::id::ChannelId(1);
/// # let message_id = twilight_model::id::MessageId(2);
/// use std::env;
/// use twilight_http::Client;
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let response = client.message(channel_id, message_id).await?;
/// let bytes = response.bytes().await?;
///
/// println!("bytes of the body: {:?}", bytes);
/// # Ok(()) }
/// ```
///
/// # Errors
///
/// Returns a [`DeserializeBodyErrorType::Chunking`] error type if the
/// response body could not be entirely read.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct BytesFuture {
    inner: Pin<Box<dyn Future<Output = Result<Vec<u8>, HyperError>> + Send + Sync + 'static>>,
}

impl Future for BytesFuture {
    type Output = Result<Vec<u8>, DeserializeBodyError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Poll::Ready(result) = Pin::new(&mut self.inner).poll(cx) {
            // Convert `hyper`'s error type to our own in order to avoid
            // directly exposing a dependency.
            Poll::Ready(result.map_err(|source| DeserializeBodyError {
                kind: DeserializeBodyErrorType::Chunking,
                source: Some(Box::new(source)),
            }))
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
/// # let guild_id = twilight_model::id::GuildId(1);
/// # let emoji_id = twilight_model::id::EmojiId(2);
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
                Poll::Ready(crate::json::from_bytes(&Bytes::from(bytes)).map_err(|source| {
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

/// Future resolving to an iterator over a list of models.
///
/// # Examples
///
/// Iterate over the emojis in a guild and print the name of each:
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let guild_id = twilight_model::id::GuildId(1);
/// use std::env;
/// use twilight_http::Client;
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let response = client.emojis(guild_id)
///     .await?;
///
/// let mut emojis = response.iter().await?;
///
/// while let Some(Ok(emoji)) = emojis.next() {
///     println!("emoji {} is called: {}", emoji.id, emoji.name);
/// }
/// # Ok(()) }
/// ```
///
/// # Errors
///
/// Returns a [`DeserializeBodyErrorType::Chunking`] error type if the
/// response body could not be entirely read.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct ModelIterFuture<T> {
    future: BytesFuture,
    phantom: PhantomData<T>,
}

impl<T> ModelIterFuture<T> {
    const fn new(bytes: BytesFuture) -> Self {
        Self {
            future: bytes,
            phantom: PhantomData,
        }
    }
}

impl<T: DeserializeOwned + Unpin> Future for ModelIterFuture<T> {
    type Output = Result<ModelIter<T>, DeserializeBodyError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.future).poll(cx) {
            Poll::Ready(Ok(bytes)) => Poll::Ready(Ok(ModelIter::new(bytes))),
            Poll::Ready(Err(source)) => Poll::Ready(Err(source)),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Future resolving to an iterator over a list of members.
///
/// # Examples
///
/// Iterate over the first 500 members in a guild and print the nickname of each:
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let guild_id = twilight_model::id::GuildId(1);
/// use std::env;
/// use twilight_http::Client;
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let response = client.guild_members(guild_id)
///     .limit(500)?
///     .await?;
///
/// let mut members = response.iter().await?;
///
/// while let Some(Ok(member)) = members.next() {
///     println!(
///         "user {}#{} has nick: {:?}",
///         member.user.name,
///         member.user.discriminator,
///         member.nick,
///     );
/// }
/// # Ok(()) }
/// ```
///
/// # Errors
///
/// Returns a [`DeserializeBodyErrorType::Chunking`] error type if the
/// response body could not be entirely read.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct MemberIterFuture {
    future: BytesFuture,
    guild_id: GuildId,
}

impl MemberIterFuture {
    const fn new(bytes: BytesFuture, guild_id: GuildId) -> Self {
        Self {
            future: bytes,
            guild_id,
        }
    }
}

impl Future for MemberIterFuture {
    type Output = Result<MemberIter, DeserializeBodyError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.future).poll(cx) {
            Poll::Ready(Ok(bytes)) => Poll::Ready(Ok(MemberIter::new(bytes, self.guild_id))),
            Poll::Ready(Err(source)) => Poll::Ready(Err(source)),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Future resolving to a deserialized [`Member`].
///
/// # Examples
///
/// Get a member by guild and user ID and print whether the user is deafened:
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let guild_id = twilight_model::id::GuildId(1);
/// # let user_id = twilight_model::id::UserId(2);
/// use std::env;
/// use twilight_http::Client;
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
///
/// let member = client.guild_member(guild_id, user_id).await?.model().await?;
///
/// println!("is member deaf?: {}", member.deaf);
/// # Ok(()) }
/// ```
///
/// # Errors
///
/// Returns a [`DeserializeBodyErrorType::Chunking`] error type if the
/// response body could not be entirely read.
///
/// Returns a [`DeserializeBodyErrorType::Deserializing`] error type if the
/// response body could not be deserialized into a member.
///
/// [`Member`]: twilight_model::guild::Member
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct MemberFuture {
    future: BytesFuture,
    guild_id: GuildId,
}

impl MemberFuture {
    const fn new(bytes: BytesFuture, guild_id: GuildId) -> Self {
        Self {
            future: bytes,
            guild_id,
        }
    }
}

impl Future for MemberFuture {
    type Output = Result<Member, DeserializeBodyError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.future).poll(cx) {
            Poll::Ready(Ok(mut bytes)) => {
                let mut deserializer = match json_deserializer(&mut bytes) {
                    Ok(deserializer) => deserializer,
                    Err(source) => return Poll::Ready(Err(source)),
                };
                let member_deserializer = MemberDeserializer::new(self.guild_id);

                let result = member_deserializer
                    .deserialize(&mut deserializer)
                    .map_err(|source| DeserializeBodyError {
                        kind: DeserializeBodyErrorType::Deserializing,
                        source: Some(Box::new(source)),
                    });

                Poll::Ready(result)
            }
            Poll::Ready(Err(source)) => Poll::Ready(Err(source)),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Future resolving to a deserialized list of [`Member`]s.
///
/// # Examples
///
/// Get the first 100 members of a guild and print their names:
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let guild_id = twilight_model::id::GuildId(1);
/// use std::env;
/// use twilight_http::Client;
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let members = client.guild_members(guild_id)
///     .limit(100)?
///     .await?
///     .models()
///     .await?;
///
/// for member in members {
///     println!("member: {}#{}", member.user.name, member.user.discriminator);
/// }
/// # Ok(()) }
/// ```
///
/// # Errors
///
/// Returns a [`DeserializeBodyErrorType::Chunking`] error type if the
/// response body could not be entirely read.
///
/// Returns a [`DeserializeBodyErrorType::Deserializing`] error type if the
/// response body could not be deserialized into a list of members.
///
/// [`Member`]: twilight_model::guild::Member
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct MemberListFuture(MemberFuture);

impl MemberListFuture {
    const fn new(bytes: BytesFuture, guild_id: GuildId) -> Self {
        Self(MemberFuture {
            future: bytes,
            guild_id,
        })
    }
}

impl Future for MemberListFuture {
    type Output = Result<Vec<Member>, DeserializeBodyError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match Pin::new(&mut self.0.future).poll(cx) {
            Poll::Ready(Ok(mut bytes)) => {
                let mut deserializer = match json_deserializer(&mut bytes) {
                    Ok(deserializer) => deserializer,
                    Err(source) => return Poll::Ready(Err(source)),
                };
                let member_list_deserializer = MemberListDeserializer::new(self.0.guild_id);

                let result = member_list_deserializer
                    .deserialize(&mut deserializer)
                    .map_err(|source| DeserializeBodyError {
                        kind: DeserializeBodyErrorType::Deserializing,
                        source: Some(Box::new(source)),
                    });

                Poll::Ready(result)
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
/// # let channel_id = twilight_model::id::ChannelId(1);
/// # let message_id = twilight_model::id::MessageId(2);
/// use std::env;
/// use twilight_http::Client;
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let response = client.message(channel_id, message_id).await?;
/// let text = response.text().await?;
///
/// println!("body: {}", text);
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

/// Create a `simd-json` Deserializer instance.
///
/// # Errors
///
/// Returns a [`DeserializeBodyErrorType::Deserializing`] error type if the
/// input is not valid JSON.
#[cfg(feature = "simd-json")]
fn json_deserializer(input: &mut [u8]) -> Result<JsonDeserializer<'_>, DeserializeBodyError> {
    JsonDeserializer::from_slice(input).map_err(|source| DeserializeBodyError {
        kind: DeserializeBodyErrorType::Deserializing,
        source: Some(Box::new(source)),
    })
}

/// Create a `serde` Deserializer instance.
#[cfg(not(feature = "simd-json"))]
fn json_deserializer(
    input: &mut [u8],
) -> Result<JsonDeserializer<serde_json::de::SliceRead<'_>>, DeserializeBodyError> {
    Ok(JsonDeserializer::from_slice(input))
}

#[cfg(test)]
mod tests {
    use super::{
        marker::{EmptyBody, ListBody, MemberBody, MemberListBody},
        BytesFuture, DeserializeBodyError, DeserializeBodyErrorType, HeaderIter, MemberFuture,
        MemberIterFuture, MemberListFuture, ModelFuture, ModelIterFuture, Response, TextFuture,
    };
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, future::Future, iter::FusedIterator};
    use twilight_model::{channel::Message, guild::Emoji};

    assert_impl_all!(BytesFuture: Future);
    assert_impl_all!(DeserializeBodyErrorType: Debug, Send, Sync);
    assert_impl_all!(DeserializeBodyError: Debug, Send, Sync);
    assert_impl_all!(HeaderIter<'_>: Debug, FusedIterator, Iterator, Send, Sync);
    assert_impl_all!(MemberFuture: Future);
    assert_impl_all!(MemberIterFuture: Future);
    assert_impl_all!(MemberListFuture: Future);
    assert_impl_all!(ModelFuture<Emoji>: Future);
    assert_impl_all!(ModelIterFuture<Emoji>: Future);
    assert_impl_all!(Response<EmptyBody>: Debug, Send, Sync);
    assert_impl_all!(Response<ListBody<Message>>: Debug, Send, Sync);
    assert_impl_all!(Response<MemberBody>: Debug, Send, Sync);
    assert_impl_all!(Response<MemberListBody>: Debug, Send, Sync);
    assert_impl_all!(TextFuture: Future);
}
