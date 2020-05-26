#[cfg(feature = "serde-support")]
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

use std::fmt::{Display, Formatter, Result as FmtResult};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ApplicationId(#[cfg_attr(feature = "serde-support", serde(with = "string"))] pub u64);

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

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct AttachmentId(#[cfg_attr(feature = "serde-support", serde(with = "string"))] pub u64);

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

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct AuditLogEntryId(#[cfg_attr(feature = "serde-support", serde(with = "string"))] pub u64);

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

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ChannelId(#[cfg_attr(feature = "serde-support", serde(with = "string"))] pub u64);

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

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct EmojiId(#[cfg_attr(feature = "serde-support", serde(with = "string"))] pub u64);

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

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct GenericId(#[cfg_attr(feature = "serde-support", serde(with = "string"))] pub u64);

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

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct GuildId(#[cfg_attr(feature = "serde-support", serde(with = "string"))] pub u64);

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

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct IntegrationId(#[cfg_attr(feature = "serde-support", serde(with = "string"))] pub u64);

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

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct MessageId(#[cfg_attr(feature = "serde-support", serde(with = "string"))] pub u64);

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

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct RoleId(#[cfg_attr(feature = "serde-support", serde(with = "string"))] pub u64);

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

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct UserId(#[cfg_attr(feature = "serde-support", serde(with = "string"))] pub u64);

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

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct WebhookId(#[cfg_attr(feature = "serde-support", serde(with = "string"))] pub u64);

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

#[cfg(all(feature = "serde-support", test))]
mod tests {
    use super::GenericId;
    use serde_test::Token;

    #[test]
    fn test_id_deser() -> Result<(), Box<dyn std::error::Error>> {
        serde_test::assert_tokens(
            &GenericId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "GenericId" },
                Token::Str("114941315417899012"),
            ],
        );

        Ok(())
    }
}
