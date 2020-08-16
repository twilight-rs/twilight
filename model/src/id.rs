pub(crate) mod string {
    use serde::{
        de::{Deserialize, Deserializer, Error as DeError},
        ser::Serializer,
    };
    use std::{fmt::Display, str::FromStr};

    pub fn serialize<T: Display, S: Serializer>(
        value: &T,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(DeError::custom)
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
        serde_test::assert_tokens(
            &AttachmentId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct {
                    name: "AttachmentId",
                },
                Token::Str("114941315417899012"),
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
        serde_test::assert_tokens(
            &ChannelId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_tokens(
            &EmojiId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "EmojiId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_tokens(
            &GenericId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "GenericId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_tokens(
            &GuildId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("114941315417899012"),
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
        serde_test::assert_tokens(
            &MessageId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "MessageId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_tokens(
            &RoleId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_tokens(
            &UserId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_tokens(
            &WebhookId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "WebhookId" },
                Token::Str("114941315417899012"),
            ],
        );
    }
}
