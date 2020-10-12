use crate::{guild::Permissions, id::GuildId};
use serde::{Deserialize, Serialize};

/// A representation of a guild received when querying the http api for the current user guilds
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CurrentUserGuild {
    /// The ID of the guild
    pub id: GuildId,
    /// The name of the guild (2-100 characters, excluding trailing and leading whitespace)
    pub name: String,
    /// The optional [icon hash](https://discord.com/developers/docs/reference#image-formatting)
    pub icon: Option<String>,
    /// True if the user is the owner of the guild
    pub owner: bool,
    /// Total permissions for the user in the guild (excludes overrides)
    #[serde(rename = "permissions_new")]
    pub permissions: Permissions,
}

#[cfg(test)]
mod tests {
    use super::{CurrentUserGuild, GuildId};
    use crate::guild::Permissions;
    use serde_test::Token;

    #[test]
    fn test_unavailable_guild() {
        // The example partial guild from the discord docs
        let value = CurrentUserGuild {
            id: GuildId(80351110224678912),
            name: "1337 Krew".to_owned(),
            icon: Some("8342729096ea3675442027381ff50dfe".to_owned()),
            owner: true,
            permissions: Permissions::from_bits_truncate(36953089),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "CurrentUserGuild",
                    len: 5,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("80351110224678912"),
                Token::Str("name"),
                Token::Str("1337 Krew"),
                Token::Str("icon"),
                Token::Some,
                Token::Str("8342729096ea3675442027381ff50dfe"),
                Token::Str("owner"),
                Token::Bool(true),
                Token::Str("permissions_new"),
                Token::Str("36953089"),
                Token::StructEnd,
            ],
        );
    }
}
