use crate::{
    guild::Permissions,
    id::{marker::GuildMarker, Id},
    util::image_hash::ImageHash,
};
use serde::{Deserialize, Serialize};

/// Information about a guild the current user is in.
///
/// This is a partial guild used for the `Get Current User Guilds` endpoint.
/// See [Discord Docs/Get Current User Guilds].
///
/// [Discord Docs/Get Current User Guilds]: https://discord.com/developers/docs/resources/user#get-current-user-guilds-example-partial-guild
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CurrentUserGuild {
    /// Unique ID.
    pub id: Id<GuildMarker>,
    /// Name of the guild.
    ///
    /// The name must be at least 2 characters long and at most 100 characters
    /// long.
    pub name: String,
    /// Hash of the icon.
    ///
    /// See [Discord Docs/Image Formatting].
    ///
    /// [Discord Docs/Image Formatting]: https://discord.com/developers/docs/reference#image-formatting
    pub icon: Option<ImageHash>,
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
    use super::{CurrentUserGuild, Permissions};
    use crate::{id::Id, test::image_hash};
    use serde_test::Token;

    #[test]
    fn current_user_guild() {
        // The example partial guild from the Discord Docs
        let value = CurrentUserGuild {
            id: Id::new(80_351_110_224_678_912),
            name: "abcd".to_owned(),
            icon: Some(image_hash::ICON),
            owner: true,
            permissions: Permissions::from_bits_truncate(36_953_089),
            features: Vec::from(["a feature".to_owned()]),
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
                Token::Str(image_hash::ICON_INPUT),
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
