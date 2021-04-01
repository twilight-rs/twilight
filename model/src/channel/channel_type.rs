use serde::{
    de::{Deserializer, Error as DeError, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};
use std::fmt::{Formatter, Result as FmtResult};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum ChannelType {
    GuildText = 0,
    Private = 1,
    GuildVoice = 2,
    Group = 3,
    GuildCategory = 4,
    GuildNews = 5,
    GuildStore = 6,
    /// Type of channel is not known to Twilight.
    Unknown = 255,
}

impl ChannelType {
    pub fn name(self) -> &'static str {
        match self {
            Self::Group => "Group",
            Self::GuildCategory => "GuildCategory",
            Self::GuildNews => "GuildNews",
            Self::GuildStore => "GuildStore",
            Self::GuildText => "GuildText",
            Self::GuildVoice => "GuildVoice",
            Self::Unknown => "Unknown",
            Self::Private => "Private",
        }
    }
}

struct ChannelTypeVisitor;

impl<'de> Visitor<'de> for ChannelTypeVisitor {
    type Value = ChannelType;

    fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("channel type variant")
    }

    fn visit_u8<E: DeError>(self, value: u8) -> Result<Self::Value, E> {
        self.visit_u64(u64::from(value))
    }

    fn visit_u64<E: DeError>(self, value: u64) -> Result<Self::Value, E> {
        Ok(match value {
            0 => ChannelType::GuildText,
            1 => ChannelType::Private,
            2 => ChannelType::GuildVoice,
            3 => ChannelType::Group,
            4 => ChannelType::GuildCategory,
            5 => ChannelType::GuildNews,
            6 => ChannelType::GuildStore,
            unknown => {
                tracing::debug!(%unknown, "received unknown channel type");

                ChannelType::Unknown
            }
        })
    }
}

impl<'de> Deserialize<'de> for ChannelType {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_u8(ChannelTypeVisitor)
    }
}

impl Serialize for ChannelType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        Serialize::serialize(&(*self as u8), serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::ChannelType;
    use serde_test::Token;

    #[test]
    fn test_variants() {
        serde_test::assert_tokens(&ChannelType::GuildText, &[Token::U8(0)]);
        serde_test::assert_tokens(&ChannelType::Private, &[Token::U8(1)]);
        serde_test::assert_tokens(&ChannelType::GuildVoice, &[Token::U8(2)]);
        serde_test::assert_tokens(&ChannelType::Group, &[Token::U8(3)]);
        serde_test::assert_tokens(&ChannelType::GuildCategory, &[Token::U8(4)]);
        serde_test::assert_tokens(&ChannelType::GuildNews, &[Token::U8(5)]);
        serde_test::assert_tokens(&ChannelType::GuildStore, &[Token::U8(6)]);
        serde_test::assert_tokens(&ChannelType::Unknown, &[Token::U8(255)]);
    }

    #[test]
    fn test_unknown_variant() {
        serde_test::assert_de_tokens(&ChannelType::Unknown, &[Token::U8(100)]);
    }

    #[test]
    fn test_names() {
        assert_eq!("Group", ChannelType::Group.name());
        assert_eq!("GuildCategory", ChannelType::GuildCategory.name());
        assert_eq!("GuildNews", ChannelType::GuildNews.name());
        assert_eq!("GuildStore", ChannelType::GuildStore.name());
        assert_eq!("GuildText", ChannelType::GuildText.name());
        assert_eq!("GuildVoice", ChannelType::GuildVoice.name());
        assert_eq!("Unknown", ChannelType::Unknown.name());
        assert_eq!("Private", ChannelType::Private.name());
    }
}
