use crate::guild::Permissions;
use crate::id::{marker::GuildMarker, Id};
use serde::{Deserialize, Serialize};

/// Information about a guild the current user is in.
///
/// This is a partial guild used for the `Get Current User Guilds` endpoint.
/// Refer to the [Discord documentation] for more information.
///
/// [Discord documentation]: https://discord.com/developers/docs/resources/user#get-current-user-guilds-example-partial-guild
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CurrentUserGuild {
    pub(crate) id: Id<GuildMarker>,
    pub(crate) name: String,
    pub(crate) icon: Option<String>,
    pub(crate) owner: bool,
    pub(crate) permissions: Permissions,
    pub(crate) features: Vec<String>,
}

impl CurrentUserGuild {
    /// Unique ID.
    pub const fn id(&self) -> Id<GuildMarker> {
        self.id
    }

    /// Name of the guild.
    ///
    /// The name must be at least 2 characters long and at most 100 characters
    /// long.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Hash of the icon.
    ///
    /// Refer to the [Discord documentation] for more information.
    ///
    /// [Discord documentation]: https://discord.com/developers/docs/reference#image-formatting
    pub fn icon(&self) -> Option<&str> {
        self.icon.as_deref()
    }

    /// Whether the current user is the owner.
    pub const fn owner(&self) -> bool {
        self.owner
    }

    /// Permissions of the current user in the guild. This excludes channels'
    /// permission overwrites.
    pub const fn permissions(&self) -> Permissions {
        self.permissions
    }

    /// Get a reference to the current user guild's features.
    pub fn features(&self) -> &[String] {
        &self.features
    }
}

#[cfg(test)]
mod tests {
    use super::{CurrentUserGuild, Permissions};
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn test_current_user_guild() {
        // The example partial guild from the discord docs
        let value = CurrentUserGuild {
            id: Id::new(80_351_110_224_678_912).expect("non zero"),
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
                Token::NewtypeStruct { name: "Id" },
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
