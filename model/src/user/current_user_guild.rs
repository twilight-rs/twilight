use crate::guild::Permissions;
use crate::id::GuildId;
use serde::{Deserialize, Serialize};

/// Information about a guild the current user is in.
///
/// This is a partial guild used for the `Get Current User Guilds` endpoint.
/// Refer to the [Discord documentation] for more information.
///
/// [Discord documentation]: https://discord.com/developers/docs/resources/user#get-current-user-guilds-example-partial-guild
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CurrentUserGuild {
    /// Unique ID.
    pub id: GuildId,
    /// Name of the guild.
    ///
    /// The name must be at least 2 characters long and at most 100 characters
    /// long.
    pub name: String,
    /// Hash of the icon.
    ///
    /// Refer to the [Discord documentation] for more information.
    ///
    /// [Discord documentation]: https://discord.com/developers/docs/reference#image-formatting
    pub icon: Option<String>,
    /// Whether the current user is the owner.
    pub owner: bool,
    /// Permissions of the current user in the guild. This excludes channels'
    /// permission overwrites.
    pub permissions: Permissions,
    /// List of enabled guild features.
    pub features: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::{CurrentUserGuild, GuildId, Permissions};
    use serde_test::Token;

    #[test]
    fn test_current_user_guild() {
        // The example partial guild from the discord docs
        let value = CurrentUserGuild {
            id: GuildId::new(80_351_110_224_678_912).expect("non zero"),
            name: "abcd".to_owned(),
            icon: Some("8342729096ea3675442027381ff50dfe".to_owned()),
            owner: true,
            permissions: Permissions::from_bits_truncate(36_953_089),
            features: vec!["a feature".to_owned()],
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "CurrentUserGuild",
                    len: 6,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("80351110224678912"),
                Token::Str("name"),
                Token::Str("abcd"),
                Token::Str("icon"),
                Token::Some,
                Token::Str("8342729096ea3675442027381ff50dfe"),
                Token::Str("owner"),
                Token::Bool(true),
                Token::Str("permissions"),
                Token::Str("36953089"),
                Token::Str("features"),
                Token::Seq { len: Some(1) },
                Token::Str("a feature"),
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}
