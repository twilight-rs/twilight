//! Type-safe IDs for each resource to avoid mixing the IDs of resources like
//! channels and guilds.
//!
//! # serde
//!
//! These IDs support deserializing from both integers and strings and serialize
//! into strings.

pub(crate) mod string {
    use serde::{
        de::{Deserializer, Error as DeError, Visitor},
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
            Ok(T::from(NonZeroU64::new(value).expect("not zero")))
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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ApplicationId(#[serde(with = "string")] pub NonZeroU64);

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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AttachmentId(#[serde(with = "string")] pub NonZeroU64);

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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AuditLogEntryId(#[serde(with = "string")] pub NonZeroU64);

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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ChannelId(#[serde(with = "string")] pub NonZeroU64);

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
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct CommandId(#[serde(with = "string")] pub NonZeroU64);

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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct EmojiId(#[serde(with = "string")] pub NonZeroU64);

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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GenericId(#[serde(with = "string")] pub NonZeroU64);

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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct GuildId(#[serde(with = "string")] pub NonZeroU64);

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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct IntegrationId(#[serde(with = "string")] pub NonZeroU64);

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
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct InteractionId(#[serde(with = "string")] pub NonZeroU64);

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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MessageId(#[serde(with = "string")] pub NonZeroU64);

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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RoleId(#[serde(with = "string")] pub NonZeroU64);

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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct StageId(#[serde(with = "string")] pub NonZeroU64);

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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct UserId(#[serde(with = "string")] pub NonZeroU64);

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

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct WebhookId(#[serde(with = "string")] pub NonZeroU64);

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
    use std::num::NonZeroU64;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_id_deser() {
        serde_test::assert_tokens(
            &ApplicationId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct {
                    name: "ApplicationId",
                },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &ApplicationId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct {
                    name: "ApplicationId",
                },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &AttachmentId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct {
                    name: "AttachmentId",
                },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &AttachmentId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct {
                    name: "AttachmentId",
                },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogEntryId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct {
                    name: "AuditLogEntryId",
                },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &AuditLogEntryId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct {
                    name: "AuditLogEntryId",
                },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &ChannelId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &ChannelId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct { name: "ChannelId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &CommandId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct { name: "CommandId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &CommandId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct { name: "CommandId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &EmojiId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct { name: "EmojiId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &EmojiId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct { name: "EmojiId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &GenericId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct { name: "GenericId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &GenericId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct { name: "GenericId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &GuildId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &GuildId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct { name: "GuildId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &IntegrationId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct {
                    name: "IntegrationId",
                },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &IntegrationId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct {
                    name: "IntegrationId",
                },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &InteractionId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct {
                    name: "InteractionId",
                },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &InteractionId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct {
                    name: "InteractionId",
                },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &MessageId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct { name: "MessageId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &MessageId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct { name: "MessageId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &RoleId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &RoleId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct { name: "RoleId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &StageId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct { name: "StageId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &StageId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct { name: "StageId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &UserId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &UserId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct { name: "UserId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &WebhookId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct { name: "WebhookId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &WebhookId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct { name: "WebhookId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
    }
}
