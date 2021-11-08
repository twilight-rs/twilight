//! ID with type-safe markers for each resource.
//!
//! When IDs are simple 64-bit integers then it may be easy to accidentally use
//! the ID of a role in place of where one means to use the ID of a user. This
//! is a programmatic error; it's on the programmer to notice. By using IDs with
//! typed markers, it can be ensured that only an ID with a guild marker is used
//! where an ID with a guild marker is requested.
//!
//! # Parsing
//!
//! IDs may be initialized or parsed in a variety of manners depending on the
//! context:
//!
//! - `serde` deserialization
//! - [`std::str::FromStr`]
//! - [`std::convert::TryFrom`]\<i64>
//! - [`std::convert::TryFrom`]\<u64>
//! - [`Id::new`]
//! - [`Id::new_unchecked`]
//! - [`std::convert::From`]<[`std::num::NonZeroU64`]>
//!
//! # Casting between resource types
//!
//! Discord may have constraints where IDs are the same across resources. For
//! example, the `@everyone` role of a guild has the same ID as the guild
//! itself. In this case, all one needs to do is use the guild's ID in place of
//! a role in order to operate on the `@everyone` role of the guild. IDs can be
//! easily casted in order to fulfill this:
//!
//! ```
//! use twilight_model::id::{marker::{GuildMarker, RoleMarker}, Id};
//!
//! // Often Rust's type inference will be able to infer the type of ID.
//! let guild_id = Id::<GuildMarker>::new(123).expect("non zero id");
//! let role_id = guild_id.cast::<RoleMarker>();
//!
//! assert_eq!(guild_id.get(), role_id.get());
//! ```

pub mod marker {
    //! Markers for various resource types, such as channels or users.
    //!
    //! Markers themselves perform no logical action, and are only used to
    //! ensure that IDs of incorrect types aren't used. If IDs were only 64-bit
    //! integers then a role's ID may be erroneously used in the place of where
    //! a user's ID is required; by using markers it can be ensured that only an
    //! ID with a [`RoleMarker`] can be used where a role's ID is required.

    /// Marker for application IDs.
    ///
    /// Types such as [`Message::application_id`] or [`Guild::application_id`]
    /// use this ID marker.
    ///
    /// [`Guild::application_id`]: crate::guild::Guild::application_id
    /// [`Message::application_id`]: crate::channel::Message::application_id
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct ApplicationMarker;

    /// Marker for attachment IDs.
    ///
    /// Types such as [`Attachment`] use this ID marker.
    ///
    /// [`Attachment`]: crate::channel::Attachment
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct AttachmentMarker;

    /// Marker for audit log entry IDs.
    ///
    /// Types such as [`AuditLogEntry`] use this ID marker.
    ///
    /// [`AuditLogEntry`]: crate::guild::audit_log::AuditLogEntry
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct AuditLogEntryMarker;

    /// Marker for channel IDs.
    ///
    /// Types such as [`PrivateChannel`] or [`TextChannel`] use this ID marker.
    ///
    /// [`PrivateChannel`]: crate::channel::PrivateChannel
    /// [`TextChannel`]: crate::channel::TextChannel
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct ChannelMarker;

    /// Marker for command IDs.
    ///
    /// Types such as [`Command`] use this ID marker.
    ///
    /// [`Command`]: crate::application::command::Command
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct CommandMarker;

    /// Marker for command versions.
    ///
    /// Types such as [`Command`] use this ID marker.
    ///
    /// [`Command`]: crate::application::command::Command
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct CommandVersionMarker;

    /// Marker for emoji IDs.
    ///
    /// Types such as [`Emoji`] or [`ReactionType`] use this ID marker.
    ///
    /// [`Emoji`]: crate::guild::Emoji
    /// [`ReactionType`]: crate::channel::ReactionType
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct EmojiMarker;

    /// Marker for generic IDs.
    ///
    /// Types such as [`AuditLogChange::Id`] or [`CommandOptionValue`] use this
    /// ID marker.
    ///
    /// [`AuditLogChange::Id`]: crate::guild::audit_log::AuditLogChange::Id
    /// [`CommandOptionValue`]: crate::application::interaction::application_command::CommandOptionValue
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct GenericMarker;

    /// Marker for guild IDs.
    ///
    /// Types such as [`Guild`] or [`Message`] use this ID marker.
    ///
    /// [`Guild`]: crate::guild::Guild
    /// [`Message`]: crate::channel::Message
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct GuildMarker;

    /// Marker for integration IDs.
    ///
    /// Types such as [`GuildIntegration`] or [`RoleTags`] use this ID marker.
    ///
    /// [`GuildIntegration`]: crate::guild::GuildIntegration
    /// [`RoleTags`]: crate::guild::RoleTags
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct IntegrationMarker;

    /// Marker for interaction IDs.
    ///
    /// Types such as [`Interaction`] or [`MessageInteraction`] use this ID
    /// marker.
    ///
    /// [`Interaction`]: crate::application::interaction::Interaction
    /// [`MessageInteraction`]: crate::channel::message::MessageInteraction
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct InteractionMarker;

    /// Marker for message IDs.
    ///
    /// Types such as [`Message`] or [`Reaction`] use this ID marker.
    ///
    /// [`Message`]: crate::channel::Message
    /// [`Reaction`]: crate::channel::Reaction
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct MessageMarker;

    /// Marker for OAuth SKU IDs.
    ///
    /// Types such as [`CurrentApplicationInfo`] use this ID marker.
    ///
    /// [`CurrentApplicationInfo`]: crate::oauth::CurrentApplicationInfo
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct OauthSkuMarker;

    /// Marker for OAuth team IDs.
    ///
    /// Types such as [`Team`] or [`TeamMember`] use this ID marker.
    ///
    /// [`Team`]: crate::oauth::team::Team
    /// [`TeamMember`]: crate::oauth::team::TeamMember
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct OauthTeamMarker;

    /// Marker for role IDs.
    ///
    /// Types such as [`Member`] or [`Role`] use this ID marker.
    ///
    /// [`Member`]: crate::guild::Member
    /// [`Role`]: crate::guild::Role
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct RoleMarker;

    /// Marker for stage IDs.
    ///
    /// Types such as [`StageInstance`] use this ID marker.
    ///
    /// [`StageInstance`]: crate::channel::StageInstance
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct StageMarker;

    /// Marker for sticker banner asset IDs.
    ///
    /// Types such as [`StickerPack`] use this ID marker.
    ///
    /// [`StickerPack`]: crate::channel::message::sticker::StickerPack
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct StickerBannerAssetMarker;

    /// Marker for sticker IDs.
    ///
    /// Types such as [`Message`] or [`Sticker`] use this ID marker.
    ///
    /// [`Message`]: crate::channel::Message
    /// [`Sticker`]: crate::channel::message::sticker::Sticker
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct StickerMarker;

    /// Marker for sticker pack IDs.
    ///
    /// Types such as [`Sticker`] or [`StickerPack`] use this ID marker.
    ///
    /// [`Sticker`]: crate::channel::message::sticker::Sticker
    /// [`StickerPack`]: crate::channel::message::sticker::StickerPack
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct StickerPackMarker;

    /// Marker for sticker pack SKU IDs.
    ///
    /// Types such as [`StickerPack`] use this ID marker.
    ///
    /// [`StickerPack`]: crate::channel::message::sticker::StickerPack
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct StickerPackSkuMarker;

    /// Marker for user IDs.
    ///
    /// Types such as [`PublicThread`] or [`User`] use this ID marker.
    ///
    /// [`PublicThread`]: crate::channel::thread::PublicThread
    /// [`User`]: crate::user::User
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct UserMarker;

    /// Marker for webhook IDs.
    ///
    /// Types such as [`Webhook`] use this ID marker.
    ///
    /// [`Webhook`]: crate::channel::webhook::Webhook
    #[derive(Clone, Copy, Debug)]
    #[non_exhaustive]
    pub struct WebhookMarker;
}

use serde::{
    de::{Deserialize, Deserializer, Error as DeError, Unexpected, Visitor},
    ser::{Serialize, Serializer},
};
use std::{
    convert::TryFrom,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
    marker::PhantomData,
    num::{NonZeroI64, NonZeroU64, ParseIntError, TryFromIntError},
    str::FromStr,
};

/// ID of a resource, such as the ID of a [channel] or [user].
///
/// Markers themselves perform no logical action, and are only used to ensure
/// that IDs of incorrect types aren't used. Read the [marker documentation] for
/// additional information.
///
/// # serde
///
/// This ID deserializes from both integers and strings and serializes into a
/// string.
///
/// [channel]: marker::ChannelMarker
/// [marker documentation]: marker
/// [user]: marker::UserMarker
#[derive(Clone, Copy)]
pub struct Id<T> {
    phantom: PhantomData<T>,
    value: NonZeroU64,
}

impl<T> Id<T> {
    const fn from_nonzero(value: NonZeroU64) -> Self {
        Self {
            phantom: PhantomData,
            value,
        }
    }

    /// Create a non-zero application ID without checking the value.
    ///
    /// Equivalent to [`NonZeroU64::new_unchecked`].
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(n: u64) -> Self {
        Self::from_nonzero(NonZeroU64::new_unchecked(n))
    }

    /// Create a non-zero application ID if the given value is not zero.
    ///
    /// Equivalent to [`NonZeroU64::new`].
    pub const fn new(n: u64) -> Option<Self> {
        #[allow(clippy::option_if_let_else)]
        if let Some(n) = NonZeroU64::new(n) {
            Some(Self::from_nonzero(n))
        } else {
            None
        }
    }

    /// Return the inner primitive value.
    ///
    /// Equivalent to [`NonZeroU64::get`].
    ///
    /// # Examples
    ///
    /// Create an ID with a value and then confirm its inner value:
    ///
    /// ```
    /// use twilight_model::id::{marker::ChannelMarker, Id};
    ///
    /// # fn try_main() -> Option<()> {
    /// let channel_id = Id::<ChannelMarker>::new(7)?;
    ///
    /// assert_eq!(7, channel_id.get());
    /// # Some(()) }
    /// #
    /// # fn main() { try_main().unwrap(); }
    /// ```
    pub const fn get(self) -> u64 {
        self.value.get()
    }

    /// Cast an ID from one type to another.
    ///
    /// # Examples
    ///
    /// Cast a role ID to a guild ID, useful for the `@everyone` role:
    ///
    /// ```
    /// use twilight_model::id::{marker::{GuildMarker, RoleMarker}, Id};
    ///
    /// let role_id: Id<RoleMarker> = Id::new(1).expect("non zero id");
    ///
    /// let guild_id: Id<GuildMarker> = role_id.cast();
    /// assert_eq!(1, guild_id.get());
    /// ```
    pub const fn cast<New>(self) -> Id<New> {
        Id::from_nonzero(self.value)
    }
}

impl<T> Debug for Id<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&self.value, f)
    }
}

impl<'de, T> Deserialize<'de> for Id<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct IdVisitor<T> {
            phantom: PhantomData<T>,
        }

        impl<T> IdVisitor<T> {
            const fn new() -> Self {
                Self {
                    phantom: PhantomData,
                }
            }
        }

        impl<'de, T> Visitor<'de> for IdVisitor<T> {
            type Value = Id<T>;

            fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
                f.write_str("a discord snowflake")
            }

            fn visit_u64<E: DeError>(self, value: u64) -> Result<Self::Value, E> {
                let value = NonZeroU64::new(value).ok_or_else(|| {
                    DeError::invalid_value(Unexpected::Unsigned(value), &"non zero u64")
                })?;

                Ok(Id::from(value))
            }

            fn visit_newtype_struct<D: Deserializer<'de>>(
                self,
                deserializer: D,
            ) -> Result<Self::Value, D::Error> {
                deserializer.deserialize_any(IdVisitor::new())
            }

            fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
                let value = v.parse().map_err(|_| {
                    let unexpected = Unexpected::Str(v);

                    DeError::invalid_value(unexpected, &"non zero u64 string")
                })?;

                self.visit_u64(value)
            }
        }

        deserializer.deserialize_any(IdVisitor::new())
    }
}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.value.get(), f)
    }
}

impl<T> From<NonZeroU64> for Id<T> {
    fn from(id: NonZeroU64) -> Self {
        Self::from_nonzero(id)
    }
}

impl<T> FromStr for Id<T> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nonzero = NonZeroU64::from_str(s)?;

        Ok(Self::from_nonzero(nonzero))
    }
}

impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    fn hash<U: Hasher>(&self, state: &mut U) {
        state.write_u64(self.value.get());
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T> Serialize for Id<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Avoid requiring a Copy trait bound by simply reconstructing self.
        let copy = Self::from_nonzero(self.value);
        let formatter = IdStringDisplay::new(copy);

        serializer.serialize_newtype_struct("Id", &formatter)
    }
}

impl<T> TryFrom<i64> for Id<T> {
    type Error = TryFromIntError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let signed_nonzero = NonZeroI64::try_from(value)?;
        let unsigned_nonzero = NonZeroU64::try_from(signed_nonzero)?;

        Ok(Self::from_nonzero(unsigned_nonzero))
    }
}

impl<T> TryFrom<u64> for Id<T> {
    type Error = TryFromIntError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let nonzero = NonZeroU64::try_from(value)?;

        Ok(Self::from_nonzero(nonzero))
    }
}

/// Display implementation to format an ID as a string.
struct IdStringDisplay<T> {
    inner: Id<T>,
}

impl<T> IdStringDisplay<T> {
    /// Create a new formatter.
    const fn new(id: Id<T>) -> Self {
        Self { inner: id }
    }
}

impl<T> Display for IdStringDisplay<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.inner.value, f)
    }
}

impl<T> Serialize for IdStringDisplay<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        marker::{
            ApplicationMarker, AttachmentMarker, AuditLogEntryMarker, ChannelMarker, CommandMarker,
            CommandVersionMarker, EmojiMarker, GenericMarker, GuildMarker, IntegrationMarker,
            InteractionMarker, MessageMarker, RoleMarker, StageMarker, UserMarker, WebhookMarker,
        },
        Id,
    };
    use serde_test::Token;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_id_deser() {
        serde_test::assert_tokens(
            &Id::<ApplicationMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<ApplicationMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<AttachmentMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<AttachmentMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<AuditLogEntryMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<AuditLogEntryMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<ChannelMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<ChannelMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<CommandMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<CommandMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<CommandVersionMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<CommandVersionMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<EmojiMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<EmojiMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<GenericMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<GenericMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<GuildMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<GuildMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<IntegrationMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<IntegrationMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<InteractionMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<InteractionMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<MessageMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<MessageMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<RoleMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<RoleMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<StageMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<StageMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<UserMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<UserMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<WebhookMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<WebhookMarker>::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
    }
}
