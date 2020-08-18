use crate::{guild::VerificationLevel, id::GuildId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InviteGuild {
    /// Hash of the banner image.
    pub banner: Option<String>,
    /// Description used for guild discovery.
    pub description: Option<String>,
    /// List of features that the guild has had enabled.
    pub features: Vec<String>,
    /// Hash of the icon image.
    pub icon: Option<String>,
    /// ID of the guild.
    pub id: GuildId,
    /// Name of the guild.
    pub name: String,
    /// Hash of the splash image.
    pub splash: Option<String>,
    /// Vanity code unique to the guild for invites.
    pub vanity_url_code: Option<String>,
    /// Account verification level required to participate.
    pub verification_level: VerificationLevel,
}

#[cfg(test)]
mod tests {
    use super::{GuildId, InviteGuild, VerificationLevel};
    use serde_test::Token;

    #[test]
    fn test_invite_guild() {
        let value = InviteGuild {
            banner: Some("banner hash".to_owned()),
            description: Some("a description".to_owned()),
            features: vec!["a feature".to_owned()],
            icon: Some("icon hash".to_owned()),
            id: GuildId(1),
            name: "guild name".to_owned(),
            splash: Some("splash hash".to_owned()),
            vanity_url_code: Some("twilight".to_owned()),
            verification_level: VerificationLevel::Medium,
        };

        serde_test::assert_tokens(&value, &[
            Token::Struct { name: "InviteGuild", len: 9 },
            Token::Str("banner"),
            Token::Some,
            Token::Str("banner hash"),
            Token::Str("description"),
            Token::Some,
            Token::Str("a description"),
            Token::Str("features"),
            Token::Seq { len: Some(1) },
            Token::Str("a feature"),
            Token::SeqEnd,
            Token::Str("icon"),
            Token::Some,
            Token::Str("icon hash"),
            Token::Str("id"),
            Token::NewtypeStruct { name: "GuildId" },
            Token::Str("1"),
            Token::Str("name"),
            Token::Str("guild name"),
            Token::Str("splash"),
            Token::Some,
            Token::Str("splash hash"),
            Token::Str("vanity_url_code"),
            Token::Some,
            Token::Str("twilight"),
            Token::Str("verification_level"),
            Token::U8(2),
            Token::StructEnd,
        ])
    }
}
