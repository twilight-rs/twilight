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
    };

    struct IdVisitor<T: From<u64>>(PhantomData<T>);

    impl<'de, T: From<u64>> Visitor<'de> for IdVisitor<T> {
        type Value = T;

        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("string or integer snowflake")
        }

        fn visit_u64<E: DeError>(self, value: u64) -> Result<Self::Value, E> {
            Ok(T::from(value))
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

    pub fn deserialize<'de, T: From<u64>, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<T, D::Error> {
        deserializer.deserialize_any(IdVisitor(PhantomData))
    }
}

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct ApplicationId(#[serde(with = "string")] pub u64);

impl Display for ApplicationId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<u64> for ApplicationId {
    fn from(id: u64) -> Self {
        ApplicationId(id)
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct AttachmentId(#[serde(with = "string")] pub u64);

impl Display for AttachmentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<u64> for AttachmentId {
    fn from(id: u64) -> Self {
        AttachmentId(id)
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct AuditLogEntryId(#[serde(with = "string")] pub u64);

impl Display for AuditLogEntryId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<u64> for AuditLogEntryId {
    fn from(id: u64) -> Self {
        AuditLogEntryId(id)
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct ChannelId(#[serde(with = "string")] pub u64);

impl Display for ChannelId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<u64> for ChannelId {
    fn from(id: u64) -> Self {
        ChannelId(id)
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct CommandId(#[serde(with = "string")] pub u64);

impl Display for CommandId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<u64> for CommandId {
    fn from(id: u64) -> Self {
        CommandId(id)
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct EmojiId(#[serde(with = "string")] pub u64);

impl Display for EmojiId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<u64> for EmojiId {
    fn from(id: u64) -> Self {
        EmojiId(id)
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct GenericId(#[serde(with = "string")] pub u64);

impl Display for GenericId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<u64> for GenericId {
    fn from(id: u64) -> Self {
        GenericId(id)
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct GuildId(#[serde(with = "string")] pub u64);

impl Display for GuildId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<u64> for GuildId {
    fn from(id: u64) -> Self {
        GuildId(id)
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct IntegrationId(#[serde(with = "string")] pub u64);

impl Display for IntegrationId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<u64> for IntegrationId {
    fn from(id: u64) -> Self {
        IntegrationId(id)
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct InteractionId(#[serde(with = "string")] pub u64);

impl Display for InteractionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<u64> for InteractionId {
    fn from(id: u64) -> Self {
        InteractionId(id)
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct MessageId(#[serde(with = "string")] pub u64);

impl Display for MessageId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<u64> for MessageId {
    fn from(id: u64) -> Self {
        MessageId(id)
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct RoleId(#[serde(with = "string")] pub u64);

impl Display for RoleId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<u64> for RoleId {
    fn from(id: u64) -> Self {
        RoleId(id)
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct UserId(#[serde(with = "string")] pub u64);

impl Display for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<u64> for UserId {
    fn from(id: u64) -> Self {
        UserId(id)
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct WebhookId(#[serde(with = "string")] pub u64);

impl Display for WebhookId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl From<u64> for WebhookId {
    fn from(id: u64) -> Self {
        WebhookId(id)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ApplicationId, AttachmentId, AuditLogEntryId, ChannelId, EmojiId, GenericId, GuildId,
        IntegrationId, MessageId, RoleId, UserId, WebhookId,
    };
    use serde_test::Token;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_id_deser() {
        serde_test::assert_tokens(
            &ApplicationId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct {
                    name: "ApplicationId",
                },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &ApplicationId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct {
                    name: "ApplicationId",
                },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &AttachmentId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct {
                    name: "AttachmentId",
                },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &AttachmentId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct {
                    name: "AttachmentId",
                },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &AuditLogEntryId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct {
                    name: "AuditLogEntryId",
                },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &AuditLogEntryId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct {
                    name: "AuditLogEntryId",
                },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &ChannelId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &ChannelId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "ChannelId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &EmojiId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "EmojiId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &EmojiId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "EmojiId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &GenericId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "GenericId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &GenericId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "GenericId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &GuildId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &GuildId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "GuildId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &IntegrationId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct {
                    name: "IntegrationId",
                },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &IntegrationId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct {
                    name: "IntegrationId",
                },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &MessageId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "MessageId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &MessageId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "MessageId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &RoleId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &RoleId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "RoleId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &UserId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &UserId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "UserId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &WebhookId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "WebhookId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &WebhookId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "WebhookId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
    }
}
