//! Iterators for lazily deserializing entries.
//!
//! # Advantages
//!
//! Iterators have two primary advantages over immediately deserializing into
//! a `Vec` of entries via [`Response::models`].
//!
//! ## Lazy deserialization
//!
//! Entries are not deserialized upfront; instead they are deserialized as
//! the iterator is advanced. This can save processing time if the iterator is
//! short-circuited or if a large number of entries are included in the response
//! body.
//!
//! ## Individual deserialization failure
//!
//! When using [`Response::models`] the entire operation fails if a single entry
//! fails to deserialize. When using the iterator results are returned on a
//! per-entry basis. This means that if only one entry fails to deserialize when
//! iterated over then the other entries can still be deserialized and
//! processed.
//!
//! [`Response::models`]: super::Response::models

use super::{DeserializeBodyError, DeserializeBodyErrorType};
use serde::de::DeserializeOwned;
use serde_json::{de::IoRead, Deserializer, StreamDeserializer};
use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    io::Cursor,
    iter::{FusedIterator, Iterator},
};
use twilight_model::{
    guild::member::{Member, MemberIntermediary},
    id::GuildId,
};

/// Lazily evaluating iterator over a response body's list of entries.
///
/// Obtained via [`Response::iter`].
///
/// Refer to the [crate-level docs][`self`] for why iterators are useful.
///
/// # Examples
///
/// Iterate over the bans of a guild:
///
/// ```no_run
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let guild_id = twilight_model::id::GuildId(1);
/// use std::env;
/// use twilight_http::Client;
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let mut response = client.bans(guild_id).await?;
///
/// // Check first to make sure the response is successful and return early if it
/// // is not.
/// if !response.status().is_success() {
///     println!("getting bans failed: {:?}", response);
///
///     return Ok(());
/// }
///
/// let mut bans = response.iter().await?;
///
/// while let Some(maybe_ban) = bans.next() {
///     // Make use of per-entry deserialization failure by handling results for
///     // each iteration.
///     match maybe_ban {
///         Ok(ban) => println!(
///             "{}#{} was banned for: {:?}",
///             ban.user.name,
///             ban.user.discriminator,
///             ban.reason,
///         ),
///         Err(source) => println!("ban failed to deserialize: {:?}", source),
///     }
/// }
/// # Ok(()) }
/// ```
///
/// [`Response::iter`]: super::Response::<ListBody<T>>::iter
/// [`self`]: self
pub struct ModelIter<T> {
    // `StreamDeserializer` is a slightly confusing name in the world of async;
    // this in fact does not implement the common async trait `Stream` but
    // instead implements `Iterator`.
    stream: StreamDeserializer<'static, IoRead<Cursor<Vec<u8>>>, T>,
}

impl<T: DeserializeOwned> ModelIter<T> {
    pub(super) fn new(bytes: Vec<u8>) -> Self {
        // `Deserializer` has a few options for how to read over input: a string
        // slice, a slice of bytes, or a reader. We want to give it ownership
        // because we want to avoid having a self-referential struct so we need
        // to give it a reader.
        //
        // `Vec` implements `Write` but not `Read`. We can instead put it into a
        // `Cursor`, which is a cheap enough wrapping type.
        let reader = Cursor::new(bytes);

        Self {
            stream: Deserializer::from_reader(reader).into_iter(),
        }
    }
}

impl<T> Debug for ModelIter<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("ModelIter").finish()
    }
}

// `StreamDeserializer` implements `FusedIterator` so this is a free impl.
impl<T: DeserializeOwned> FusedIterator for ModelIter<T> {}

impl<T: DeserializeOwned> Iterator for ModelIter<T> {
    type Item = Result<T, DeserializeBodyError>;

    /// Deserialize the next entry in the iterator.
    ///
    /// Every iteration returns a result for that individual entry in the
    /// response body. If a response has many entries and an entry fails to
    /// deserialize in the middle of the iterator then succeeding entries are
    /// unaffected by a failure and can still successfully deserialize.
    ///
    /// # Examples
    ///
    /// Returns a [`DeserializeBodyErrorType::Deserializing`] error type if the
    /// entry could not be deserialized into something.
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.stream.next()?;

        // Convert `serde_json`'s error type to our own in order to avoid
        // directly exposing a dependency.
        Some(result.map_err(|source| DeserializeBodyError {
            kind: DeserializeBodyErrorType::Deserializing,
            source: Some(Box::new(source)),
        }))
    }
}

/// Lazily evaluating iterator over a response body's list of members.
///
/// Obtained via [`Response::iter`].
///
/// Refer to the [crate-level docs][`self`] for why iterators are useful.
///
/// [`Response::iter`]: super::Response::<MemberListBody>::iter
/// [`self`]: self
pub struct MemberIter {
    guild_id: GuildId,
    stream: StreamDeserializer<'static, IoRead<Cursor<Vec<u8>>>, MemberIntermediary>,
}

impl MemberIter {
    pub(super) fn new(bytes: Vec<u8>, guild_id: GuildId) -> Self {
        let reader = Cursor::new(bytes);

        Self {
            guild_id,
            stream: Deserializer::from_reader(reader).into_iter(),
        }
    }
}

impl Debug for MemberIter {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("ResponseMemberIter").finish()
    }
}

impl FusedIterator for MemberIter {}

impl Iterator for MemberIter {
    type Item = Result<Member, DeserializeBodyError>;

    /// Deserialize the next member in the iterator.
    ///
    /// Every iteration returns a result for that individual member in the
    /// response body. If a response has many members and an member fails to
    /// deserialize in the middle of the iterator then succeeding members are
    /// unaffected by a failure and can still successfully deserialize.
    ///
    /// # Examples
    ///
    /// Returns a [`DeserializeBodyErrorType::Deserializing`] error type if the
    /// member could not be deserialized.
    fn next(&mut self) -> Option<Self::Item> {
        let result = self.stream.next()?;

        // We need to map the intermediary member - which has no guild ID - to a
        // member, with the guild ID supplied by the iterator.
        Some(
            result
                .map(|member| Member {
                    deaf: member.deaf,
                    guild_id: self.guild_id,
                    hoisted_role: member.hoisted_role,
                    joined_at: member.joined_at,
                    mute: member.mute,
                    nick: member.nick,
                    pending: member.pending,
                    premium_since: member.premium_since,
                    roles: member.roles,
                    user: member.user,
                })
                .map_err(|source| DeserializeBodyError {
                    kind: DeserializeBodyErrorType::Deserializing,
                    source: Some(Box::new(source)),
                }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{MemberIter, ModelIter};
    use static_assertions::assert_impl_all;
    use std::iter::FusedIterator;
    use twilight_model::channel::Message;

    assert_impl_all!(MemberIter: FusedIterator, Iterator, Send, Sync);
    assert_impl_all!(ModelIter<Message>: FusedIterator, Iterator, Send, Sync);
}
