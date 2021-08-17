//! Type-safe IDs for each resource to avoid mixing the IDs of resources like
//! channels and guilds.
//!
//! # serde
//!
//! These IDs support deserializing from both integers and strings and serialize
//! into strings.

pub(crate) mod string {
    use serde::{
        de::{Deserializer, Error as DeError, Unexpected, Visitor},
        ser::Serializer,
    };
    use std::{
        fmt::{Display, Formatter, Result as FmtResult},
        marker::PhantomData,
        num::NonZeroU64,
    };

    struct IdVisitor<T: From<NonZeroU64>>(PhantomData<T>);

    impl<'de, T: From<NonZeroU64>> Visitor<'de> for IdVisitor<T> {
        type Value = T;

        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("string or integer snowflake")
        }

        fn visit_u64<E: DeError>(self, value: u64) -> Result<Self::Value, E> {
            NonZeroU64::new(value).map(T::from).ok_or_else(|| {
                let unexpected = Unexpected::Unsigned(value);

                DeError::invalid_value(unexpected, &"a non-zero unsigned integer")
            })
        }

        fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
            value.parse().map(T::from).map_err(DeError::custom)
        }
    }

    pub fn serialize<T: Display, S: Serializer>(
        value: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, T: From<NonZeroU64>, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<T, D::Error> {
        deserializer.deserialize_any(IdVisitor(PhantomData))
    }
}

use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    num::NonZeroU64,
};

/// Unique ID of an application.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ApplicationId(#[serde(with = "string")] pub NonZeroU64);

impl ApplicationId {
    /// Create a non-zero application ID without checking the value.
    ///
    /// Equivalent to [`NonZeroU64::new_unchecked`].
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(n: u64) -> Self {
        Self(NonZeroU64::new_unchecked(n))
    }

    /// Create a non-zero application ID if the given value is not zero.
    ///
    /// Equivalent to [`NonZeroU64::new`].
    pub const fn new(n: u64) -> Option<Self> {
        #[allow(clippy::option_if_let_else)]
        if let Some(n) = NonZeroU64::new(n) {
            Some(Self(n))
        } else {
            None
        }
    }

    /// Return the inner primitive value.
    ///
    /// Equivalent to [`NonZeroU64::get`].
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl Display for ApplicationId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<NonZeroU64> for ApplicationId {
    fn from(id: NonZeroU64) -> Self {
        ApplicationId(id)
    }
}

/// Unique ID of an attachment.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AttachmentId(#[serde(with = "string")] pub NonZeroU64);

impl AttachmentId {
    /// Create a non-zero attachment ID without checking the value.
    ///
    /// Equivalent to [`NonZeroU64::new_unchecked`].
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(n: u64) -> Self {
        Self(NonZeroU64::new_unchecked(n))
    }

    /// Create a non-zero attachment ID if the given value is not zero.
    ///
    /// Equivalent to [`NonZeroU64::new`].
    pub const fn new(n: u64) -> Option<Self> {
        #[allow(clippy::option_if_let_else)]
        if let Some(n) = NonZeroU64::new(n) {
            Some(Self(n))
        } else {
            None
        }
    }

    /// Return the inner primitive value.
    ///
    /// Equivalent to [`NonZeroU64::get`].
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl Display for AttachmentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<NonZeroU64> for AttachmentId {
    fn from(id: NonZeroU64) -> Self {
        AttachmentId(id)
    }
}

/// Unique ID of an audit log entry.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AuditLogEntryId(#[serde(with = "string")] pub NonZeroU64);

impl AuditLogEntryId {
    /// Create a non-zero audit log entry ID without checking the value.
    ///
    /// Equivalent to [`NonZeroU64::new_unchecked`].
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(n: u64) -> Self {
        Self(NonZeroU64::new_unchecked(n))
    }

    /// Create a non-zero audit log entry ID if the given value is not zero.
    ///
    /// Equivalent to [`NonZeroU64::new`].
    pub const fn new(n: u64) -> Option<Self> {
        #[allow(clippy::option_if_let_else)]
        if let Some(n) = NonZeroU64::new(n) {
            Some(Self(n))
        } else {
            None
        }
    }

    /// Return the inner primitive value.
    ///
    /// Equivalent to [`NonZeroU64::get`].
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl Display for AuditLogEntryId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<NonZeroU64> for AuditLogEntryId {
    fn from(id: NonZeroU64) -> Self {
        AuditLogEntryId(id)
    }
}

/// Unique ID of a channel.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ChannelId(#[serde(with = "string")] pub NonZeroU64);

impl ChannelId {
    /// Create a non-zero channel ID without checking the value.
    ///
    /// Equivalent to [`NonZeroU64::new_unchecked`].
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(n: u64) -> Self {
        Self(NonZeroU64::new_unchecked(n))
    }

    /// Create a non-zero channel ID if the given value is not zero.
    ///
    /// Equivalent to [`NonZeroU64::new`].
    pub const fn new(n: u64) -> Option<Self> {
        #[allow(clippy::option_if_let_else)]
        if let Some(n) = NonZeroU64::new(n) {
            Some(Self(n))
        } else {
            None
        }
    }

    /// Return the inner primitive value.
    ///
    /// Equivalent to [`NonZeroU64::get`].
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl Display for ChannelId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<NonZeroU64> for ChannelId {
    fn from(id: NonZeroU64) -> Self {
        ChannelId(id)
    }
}

/// Unique ID of a command used in slash commands.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CommandId(#[serde(with = "string")] pub NonZeroU64);

impl CommandId {
    /// Create a non-zero command ID without checking the value.
    ///
    /// Equivalent to [`NonZeroU64::new_unchecked`].
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(n: u64) -> Self {
        Self(NonZeroU64::new_unchecked(n))
    }

    /// Create a non-zero command ID if the given value is not zero.
    ///
    /// Equivalent to [`NonZeroU64::new`].
    pub const fn new(n: u64) -> Option<Self> {
        #[allow(clippy::option_if_let_else)]
        if let Some(n) = NonZeroU64::new(n) {
            Some(Self(n))
        } else {
            None
        }
    }

    /// Return the inner primitive value.
    ///
    /// Equivalent to [`NonZeroU64::get`].
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl Display for CommandId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<NonZeroU64> for CommandId {
    fn from(id: NonZeroU64) -> Self {
        CommandId(id)
    }
}

/// Unique ID of an emoji.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct EmojiId(#[serde(with = "string")] pub NonZeroU64);

impl EmojiId {
    /// Create a non-zero emoji ID without checking the value.
    ///
    /// Equivalent to [`NonZeroU64::new_unchecked`].
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(n: u64) -> Self {
        Self(NonZeroU64::new_unchecked(n))
    }

    /// Create a non-zero emoji ID if the given value is not zero.
    ///
    /// Equivalent to [`NonZeroU64::new`].
    pub const fn new(n: u64) -> Option<Self> {
        #[allow(clippy::option_if_let_else)]
        if let Some(n) = NonZeroU64::new(n) {
            Some(Self(n))
        } else {
            None
        }
    }

    /// Return the inner primitive value.
    ///
    /// Equivalent to [`NonZeroU64::get`].
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl Display for EmojiId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<NonZeroU64> for EmojiId {
    fn from(id: NonZeroU64) -> Self {
        EmojiId(id)
    }
}

/// Unique ID of a generic item.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GenericId(#[serde(with = "string")] pub NonZeroU64);

impl GenericId {
    /// Create a non-zero generic ID without checking the value.
    ///
    /// Equivalent to [`NonZeroU64::new_unchecked`].
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(n: u64) -> Self {
        Self(NonZeroU64::new_unchecked(n))
    }

    /// Create a non-zero generic ID if the given value is not zero.
    ///
    /// Equivalent to [`NonZeroU64::new`].
    pub const fn new(n: u64) -> Option<Self> {
        #[allow(clippy::option_if_let_else)]
        if let Some(n) = NonZeroU64::new(n) {
            Some(Self(n))
        } else {
            None
        }
    }

    /// Return the inner primitive value.
    ///
    /// Equivalent to [`NonZeroU64::get`].
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl Display for GenericId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<NonZeroU64> for GenericId {
    fn from(id: NonZeroU64) -> Self {
        GenericId(id)
    }
}

/// Unique ID of a guild.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GuildId(#[serde(with = "string")] pub NonZeroU64);

impl GuildId {
    /// Create a non-zero guild ID without checking the value.
    ///
    /// Equivalent to [`NonZeroU64::new_unchecked`].
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(n: u64) -> Self {
        Self(NonZeroU64::new_unchecked(n))
    }

    /// Create a non-zero guild ID if the given value is not zero.
    ///
    /// Equivalent to [`NonZeroU64::new`].
    pub const fn new(n: u64) -> Option<Self> {
        #[allow(clippy::option_if_let_else)]
        if let Some(n) = NonZeroU64::new(n) {
            Some(Self(n))
        } else {
            None
        }
    }

    /// Return the inner primitive value.
    ///
    /// Equivalent to [`NonZeroU64::get`].
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl Display for GuildId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<NonZeroU64> for GuildId {
    fn from(id: NonZeroU64) -> Self {
        GuildId(id)
    }
}

/// Unique ID of an integration.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IntegrationId(#[serde(with = "string")] pub NonZeroU64);

impl IntegrationId {
    /// Create a non-zero integration ID without checking the value.
    ///
    /// Equivalent to [`NonZeroU64::new_unchecked`].
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(n: u64) -> Self {
        Self(NonZeroU64::new_unchecked(n))
    }

    /// Create a non-zero integration ID if the given value is not zero.
    ///
    /// Equivalent to [`NonZeroU64::new`].
    pub const fn new(n: u64) -> Option<Self> {
        #[allow(clippy::option_if_let_else)]
        if let Some(n) = NonZeroU64::new(n) {
            Some(Self(n))
        } else {
            None
        }
    }

    /// Return the inner primitive value.
    ///
    /// Equivalent to [`NonZeroU64::get`].
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl Display for IntegrationId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<NonZeroU64> for IntegrationId {
    fn from(id: NonZeroU64) -> Self {
        IntegrationId(id)
    }
}

/// Unique ID of an interaction payload.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct InteractionId(#[serde(with = "string")] pub NonZeroU64);

impl InteractionId {
    /// Create a non-zero interaction ID without checking the value.
    ///
    /// Equivalent to [`NonZeroU64::new_unchecked`].
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(n: u64) -> Self {
        Self(NonZeroU64::new_unchecked(n))
    }

    /// Create a non-zero interaction ID if the given value is not zero.
    ///
    /// Equivalent to [`NonZeroU64::new`].
    pub const fn new(n: u64) -> Option<Self> {
        #[allow(clippy::option_if_let_else)]
        if let Some(n) = NonZeroU64::new(n) {
            Some(Self(n))
        } else {
            None
        }
    }

    /// Return the inner primitive value.
    ///
    /// Equivalent to [`NonZeroU64::get`].
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl Display for InteractionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<NonZeroU64> for InteractionId {
    fn from(id: NonZeroU64) -> Self {
        InteractionId(id)
    }
}

/// Unique ID of a message.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MessageId(#[serde(with = "string")] pub NonZeroU64);

impl MessageId {
    /// Create a non-zero message ID without checking the value.
    ///
    /// Equivalent to [`NonZeroU64::new_unchecked`].
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(n: u64) -> Self {
        Self(NonZeroU64::new_unchecked(n))
    }

    /// Create a non-zero message ID if the given value is not zero.
    ///
    /// Equivalent to [`NonZeroU64::new`].
    pub const fn new(n: u64) -> Option<Self> {
        #[allow(clippy::option_if_let_else)]
        if let Some(n) = NonZeroU64::new(n) {
            Some(Self(n))
        } else {
            None
        }
    }

    /// Return the inner primitive value.
    ///
    /// Equivalent to [`NonZeroU64::get`].
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl Display for MessageId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<NonZeroU64> for MessageId {
    fn from(id: NonZeroU64) -> Self {
        MessageId(id)
    }
}

/// Unique ID of a role.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RoleId(#[serde(with = "string")] pub NonZeroU64);

impl RoleId {
    /// Create a non-zero role ID without checking the value.
    ///
    /// Equivalent to [`NonZeroU64::new_unchecked`].
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(n: u64) -> Self {
        Self(NonZeroU64::new_unchecked(n))
    }

    /// Create a non-zero role ID if the given value is not zero.
    ///
    /// Equivalent to [`NonZeroU64::new`].
    pub const fn new(n: u64) -> Option<Self> {
        #[allow(clippy::option_if_let_else)]
        if let Some(n) = NonZeroU64::new(n) {
            Some(Self(n))
        } else {
            None
        }
    }

    /// Return the inner primitive value.
    ///
    /// Equivalent to [`NonZeroU64::get`].
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl Display for RoleId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<NonZeroU64> for RoleId {
    fn from(id: NonZeroU64) -> Self {
        RoleId(id)
    }
}

/// Unique ID of a stage.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct StageId(#[serde(with = "string")] pub NonZeroU64);

impl StageId {
    /// Create a non-zero stage ID without checking the value.
    ///
    /// Equivalent to [`NonZeroU64::new_unchecked`].
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(n: u64) -> Self {
        Self(NonZeroU64::new_unchecked(n))
    }

    /// Create a non-zero stage ID if the given value is not zero.
    ///
    /// Equivalent to [`NonZeroU64::new`].
    pub const fn new(n: u64) -> Option<Self> {
        #[allow(clippy::option_if_let_else)]
        if let Some(n) = NonZeroU64::new(n) {
            Some(Self(n))
        } else {
            None
        }
    }

    /// Return the inner primitive value.
    ///
    /// Equivalent to [`NonZeroU64::get`].
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl Display for StageId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<NonZeroU64> for StageId {
    fn from(id: NonZeroU64) -> Self {
        StageId(id)
    }
}

/// Unique ID of an user.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct UserId(#[serde(with = "string")] pub NonZeroU64);

impl UserId {
    /// Create a non-zero user ID without checking the value.
    ///
    /// Equivalent to [`NonZeroU64::new_unchecked`].
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(n: u64) -> Self {
        Self(NonZeroU64::new_unchecked(n))
    }

    /// Create a non-zero user ID if the given value is not zero.
    ///
    /// Equivalent to [`NonZeroU64::new`].
    pub const fn new(n: u64) -> Option<Self> {
        #[allow(clippy::option_if_let_else)]
        if let Some(n) = NonZeroU64::new(n) {
            Some(Self(n))
        } else {
            None
        }
    }

    /// Return the inner primitive value.
    ///
    /// Equivalent to [`NonZeroU64::get`].
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<NonZeroU64> for UserId {
    fn from(id: NonZeroU64) -> Self {
        UserId(id)
    }
}

/// Unique ID of a webhook.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WebhookId(#[serde(with = "string")] pub NonZeroU64);

impl WebhookId {
    /// Create a non-zero webhook ID without checking the value.
    ///
    /// Equivalent to [`NonZeroU64::new_unchecked`].
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(n: u64) -> Self {
        Self(NonZeroU64::new_unchecked(n))
    }

    /// Create a non-zero webhook ID if the given value is not zero.
    ///
    /// Equivalent to [`NonZeroU64::new`].
    pub const fn new(n: u64) -> Option<Self> {
        #[allow(clippy::option_if_let_else)]
        if let Some(n) = NonZeroU64::new(n) {
            Some(Self(n))
        } else {
            None
        }
    }

    /// Return the inner primitive value.
    ///
    /// Equivalent to [`NonZeroU64::get`].
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl Display for WebhookId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<NonZeroU64> for WebhookId {
    fn from(id: NonZeroU64) -> Self {
        WebhookId(id)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ApplicationId, AttachmentId, AuditLogEntryId, ChannelId, CommandId, EmojiId, GenericId,
        GuildId, IntegrationId, InteractionId, MessageId, RoleId, StageId, UserId, WebhookId,
    };
    use serde_test::Token;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_id_deser() {
        serde_test::assert_tokens(
            &ApplicationId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct {
                    name: "ApplicationId",
                },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &ApplicationId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct {
                    name: "ApplicationId",
                },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &AttachmentId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct {
                    name: "AttachmentId",
                },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &AttachmentId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct {
                    name: "AttachmentId",
                },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogEntryId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct {
                    name: "AuditLogEntryId",
                },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &AuditLogEntryId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct {
                    name: "AuditLogEntryId",
                },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &ChannelId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &ChannelId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "ChannelId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &CommandId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "CommandId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &CommandId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "CommandId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &EmojiId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "EmojiId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &EmojiId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "EmojiId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &GenericId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "GenericId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &GenericId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "GenericId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &GuildId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &GuildId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "GuildId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &IntegrationId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct {
                    name: "IntegrationId",
                },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &IntegrationId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct {
                    name: "IntegrationId",
                },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &InteractionId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct {
                    name: "InteractionId",
                },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &InteractionId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct {
                    name: "InteractionId",
                },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &MessageId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "MessageId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &MessageId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "MessageId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &RoleId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &RoleId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "RoleId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &StageId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "StageId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &StageId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "StageId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &UserId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &UserId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "UserId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &WebhookId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "WebhookId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &WebhookId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "WebhookId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
    }
}
