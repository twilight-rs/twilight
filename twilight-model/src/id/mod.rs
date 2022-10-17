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
//! - [`Id::new_checked`]
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
//! use twilight_model::id::{
//!     marker::{GuildMarker, RoleMarker},
//!     Id,
//! };
//!
//! // Often Rust's type inference will be able to infer the type of ID.
//! let guild_id = Id::<GuildMarker>::new(123);
//! let role_id = guild_id.cast::<RoleMarker>();
//!
//! assert_eq!(guild_id.get(), role_id.get());
//! ```

pub mod marker;

mod r#type;

pub use self::r#type::*;

use serde::{
    de::{Deserialize, Deserializer, Error as DeError, Unexpected, Visitor},
    ser::{Serialize, Serializer},
};
use std::{
    any,
    cmp::Ordering,
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
#[repr(transparent)]
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
    /// Create a new ID, panicking if the value is zero.
    ///
    /// This is primarily useful in const contexts where you are passing a
    /// hardcoded value.
    ///
    /// Refer to [`new_checked`] for a checked alternative to this method.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::id::{marker::GenericMarker, Id};
    ///
    /// const ID: Id<GenericMarker> = Id::new(123);
    ///
    /// println!("id: {ID}");
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the value is 0.
    ///
    /// [`new_checked`]: Self::new_checked
    #[track_caller]
    pub const fn new(n: u64) -> Self {
        if let Some(id) = Self::new_checked(n) {
            id
        } else {
            panic!("value is zero");
        }
    }

    /// Create an ID without checking the value.
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

    /// Create an ID, checking if the provided value is zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::id::{marker::GenericMarker, Id};
    ///
    /// assert!(Id::<GenericMarker>::new_checked(123).is_some());
    /// assert!(Id::<GenericMarker>::new_checked(0).is_none());
    /// ```
    ///
    /// Equivalent to [`NonZeroU64::new`].
    pub const fn new_checked(n: u64) -> Option<Self> {
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
    /// let channel_id = Id::<ChannelMarker>::new(7);
    ///
    /// assert_eq!(7, channel_id.get());
    /// ```
    pub const fn get(self) -> u64 {
        self.value.get()
    }

    /// Return the [`NonZeroU64`] representation of the ID.
    ///
    /// # Examples
    ///
    /// Create an ID with a value and then confirm its nonzero value:
    ///
    /// ```
    /// use std::num::NonZeroU64;
    /// use twilight_model::id::{marker::ChannelMarker, Id};
    ///
    /// let channel_id = Id::<ChannelMarker>::new(7);
    ///
    /// assert_eq!(NonZeroU64::new(7).unwrap(), channel_id.into_nonzero());
    /// ```
    pub const fn into_nonzero(self) -> NonZeroU64 {
        self.value
    }

    /// Cast an ID from one type to another.
    ///
    /// # Examples
    ///
    /// Cast a role ID to a guild ID, useful for the `@everyone` role:
    ///
    /// ```
    /// use twilight_model::id::{
    ///     marker::{GuildMarker, RoleMarker},
    ///     Id,
    /// };
    ///
    /// let role_id: Id<RoleMarker> = Id::new(1);
    ///
    /// let guild_id: Id<GuildMarker> = role_id.cast();
    /// assert_eq!(1, guild_id.get());
    /// ```
    pub const fn cast<New>(self) -> Id<New> {
        Id::from_nonzero(self.value)
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self::from_nonzero(self.value)
    }
}

impl<T> Copy for Id<T> {}

impl<T> Debug for Id<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("Id")?;
        let type_name = any::type_name::<T>();

        // `any::type_name` will usually provide an FQN, so we'll do our best
        // (and simplest) method here of removing it to only get the type name
        // itself.
        if let Some(position) = type_name.rfind("::") {
            if let Some(slice) = type_name.get(position + 2..) {
                f.write_str("<")?;
                f.write_str(slice)?;
                f.write_str(">")?;
            }
        }

        f.write_str("(")?;
        Debug::fmt(&self.value, f)?;

        f.write_str(")")
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

impl<T> From<Id<T>> for u64 {
    fn from(id: Id<T>) -> Self {
        id.get()
    }
}

impl<T> From<NonZeroU64> for Id<T> {
    fn from(id: NonZeroU64) -> Self {
        Self::from_nonzero(id)
    }
}

impl<T> From<Id<T>> for NonZeroU64 {
    fn from(id: Id<T>) -> Self {
        id.into_nonzero()
    }
}

impl<T> FromStr for Id<T> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NonZeroU64::from_str(s).map(Self::from_nonzero)
    }
}

impl<T> Eq for Id<T> {}

impl<T> Hash for Id<T> {
    fn hash<U: Hasher>(&self, state: &mut U) {
        state.write_u64(self.value.get());
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> PartialEq<i64> for Id<T> {
    fn eq(&self, other: &i64) -> bool {
        u64::try_from(*other)
            .map(|v| v == self.value.get())
            .unwrap_or_default()
    }
}

impl<T> PartialEq<Id<T>> for i64 {
    fn eq(&self, other: &Id<T>) -> bool {
        u64::try_from(*self)
            .map(|v| v == other.value.get())
            .unwrap_or_default()
    }
}

impl<T> PartialEq<u64> for Id<T> {
    fn eq(&self, other: &u64) -> bool {
        self.value.get() == *other
    }
}

impl<T> PartialEq<Id<T>> for u64 {
    fn eq(&self, other: &Id<T>) -> bool {
        other.value.get() == *self
    }
}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T> Serialize for Id<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_newtype_struct("Id", &self.to_string())
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
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::{
        collections::hash_map::DefaultHasher,
        error::Error,
        fmt::{Debug, Display},
        hash::{Hash, Hasher},
        num::NonZeroU64,
        str::FromStr,
    };

    assert_impl_all!(ApplicationMarker: Debug, Send, Sync);
    assert_impl_all!(AttachmentMarker: Debug, Send, Sync);
    assert_impl_all!(AuditLogEntryMarker: Debug, Send, Sync);
    assert_impl_all!(ChannelMarker: Debug, Send, Sync);
    assert_impl_all!(CommandMarker: Debug, Send, Sync);
    assert_impl_all!(CommandVersionMarker: Debug, Send, Sync);
    assert_impl_all!(EmojiMarker: Debug, Send, Sync);
    assert_impl_all!(GenericMarker: Debug, Send, Sync);
    assert_impl_all!(GuildMarker: Debug, Send, Sync);
    assert_impl_all!(IntegrationMarker: Debug, Send, Sync);
    assert_impl_all!(InteractionMarker: Debug, Send, Sync);
    assert_impl_all!(MessageMarker: Debug, Send, Sync);
    assert_impl_all!(RoleMarker: Debug, Send, Sync);
    assert_impl_all!(StageMarker: Debug, Send, Sync);
    assert_impl_all!(UserMarker: Debug, Send, Sync);
    assert_impl_all!(WebhookMarker: Debug, Send, Sync);
    assert_impl_all!(Id<GenericMarker>:
        Clone, Copy, Debug, Deserialize<'static>, Display, Eq, From<NonZeroU64>,
        FromStr, Hash, Into<NonZeroU64>, Into<u64>, Ord, PartialEq, PartialEq<i64>, PartialEq<u64>, PartialOrd, Send, Serialize, Sync,
        TryFrom<i64>, TryFrom<u64>
    );

    /// Test that various methods of initializing IDs are correct, such as via
    /// [`Id::new`] or [`Id`]'s [`TryFrom`] implementations.
    #[test]
    fn initializers() -> Result<(), Box<dyn Error>> {
        // `Id::new_checked`
        assert!(Id::<GenericMarker>::new_checked(0).is_none());
        assert_eq!(Some(1), Id::<GenericMarker>::new_checked(1).map(Id::get));

        // `Id::new`
        assert_eq!(1, Id::<GenericMarker>::new(1).get());

        // `From`
        assert_eq!(
            123_u64,
            Id::<GenericMarker>::from(NonZeroU64::new(123).expect("non zero"))
        );

        // `FromStr`
        assert_eq!(123_u64, Id::<GenericMarker>::from_str("123")?);
        assert!(Id::<GenericMarker>::from_str("0").is_err());
        assert!(Id::<GenericMarker>::from_str("123a").is_err());

        // `TryFrom`
        assert!(Id::<GenericMarker>::try_from(-123_i64).is_err());
        assert!(Id::<GenericMarker>::try_from(0_i64).is_err());
        assert_eq!(123_u64, Id::<GenericMarker>::try_from(123_i64)?);
        assert!(Id::<GenericMarker>::try_from(0_u64).is_err());
        assert_eq!(123_u64, Id::<GenericMarker>::try_from(123_u64)?);

        Ok(())
    }

    /// Test that conversion methods are correct.
    #[test]
    fn conversions() {
        // `Into`
        assert_eq!(1, u64::from(Id::<GenericMarker>::new(1)));
        assert_eq!(
            NonZeroU64::new(1).expect("non zero"),
            NonZeroU64::from(Id::<GenericMarker>::new(1))
        );
    }

    /// Test that creating an ID via [`Id::new`] with a value of zero panics.
    #[should_panic]
    #[test]
    const fn test_new_checked_zero() {
        let _ = Id::<GenericMarker>::new(0);
    }

    /// Test that casting IDs maintains the original value.
    #[test]
    fn cast() {
        let id = Id::<GenericMarker>::new(123);
        assert_eq!(123_u64, id.cast::<RoleMarker>());
    }

    /// Test that debugging IDs formats the generic and value as a newtype.
    #[test]
    fn debug() {
        let id = Id::<RoleMarker>::new(114_941_315_417_899_012);

        assert_eq!("Id<RoleMarker>(114941315417899012)", format!("{id:?}"));
    }

    /// Test that display formatting an ID formats the value.
    #[test]
    fn display() {
        let id = Id::<GenericMarker>::new(114_941_315_417_899_012);

        assert_eq!("114941315417899012", id.to_string());
    }

    /// Test that hashing an ID is equivalent to hashing only its inner value.
    #[test]
    fn hash() {
        let id = Id::<GenericMarker>::new(123);

        let mut id_hasher = DefaultHasher::new();
        id.hash(&mut id_hasher);

        let mut value_hasher = DefaultHasher::new();
        123_u64.hash(&mut value_hasher);

        assert_eq!(id_hasher.finish(), value_hasher.finish());
    }

    /// Test that IDs are ordered exactly like their inner values.
    #[test]
    fn ordering() {
        let lesser = Id::<GenericMarker>::new(911_638_235_594_244_096);
        let center = Id::<GenericMarker>::new(911_638_263_322_800_208);
        let greater = Id::<GenericMarker>::new(911_638_287_939_166_208);

        assert!(center.cmp(&greater).is_lt());
        assert!(center.cmp(&center).is_eq());
        assert!(center.cmp(&lesser).is_gt());
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn serde() {
        serde_test::assert_tokens(
            &Id::<ApplicationMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<ApplicationMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<AttachmentMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<AttachmentMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<AuditLogEntryMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<AuditLogEntryMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<ChannelMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<ChannelMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<CommandMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<CommandMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<CommandVersionMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<CommandVersionMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<EmojiMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<EmojiMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<GenericMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<GenericMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<GuildMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<GuildMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<IntegrationMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<IntegrationMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<InteractionMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<InteractionMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<MessageMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<MessageMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<RoleMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<RoleMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<StageMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<StageMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<UserMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<UserMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &Id::<WebhookMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &Id::<WebhookMarker>::new(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "Id" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
    }
}
