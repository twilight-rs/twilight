use crate::{
    guild::Permissions,
    id::{marker::GuildMarker, Id},
    util::image_hash::ImageHash,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildInfo {
    pub icon: Option<ImageHash>,
    pub id: Id<GuildMarker>,
    pub name: String,
    pub owner: bool,
    pub permissions: Permissions,
}

#[cfg(test)]
mod tests {
    use super::{GuildInfo, Permissions};
    use crate::{id::Id, test::image_hash};
    use serde_test::Token;

    #[test]
    fn test_guild_info() {
        let value = GuildInfo {
            icon: Some(image_hash::ICON),
            id: Id::new(1),
            name: "guild name".to_owned(),
            owner: false,
            permissions: Permissions::MANAGE_CHANNELS,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "GuildInfo",
                    len: 5,
                },
                Token::Str("icon"),
                Token::Some,
                Token::Str(image_hash::ICON_INPUT),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("name"),
                Token::Str("guild name"),
                Token::Str("owner"),
                Token::Bool(false),
                Token::Str("permissions"),
                Token::Str("16"),
                Token::StructEnd,
            ],
        );
    }
}
