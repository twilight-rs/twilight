use crate::{
    channel::{permission_overwrite::PermissionOverwrite, ChannelType},
    id::{ChannelId, GuildId},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CategoryChannel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
    pub id: ChannelId,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub name: String,
    pub permission_overwrites: Vec<PermissionOverwrite>,
    pub position: i64,
}

#[cfg(test)]
mod tests {
    use super::{CategoryChannel, ChannelId, ChannelType, GuildId};
    use serde_test::Token;

    #[test]
    fn test_category_channel() {
        let value = CategoryChannel {
            guild_id: Some(GuildId::new(1).expect("non zero")),
            id: ChannelId::new(2).expect("non zero"),
            kind: ChannelType::GuildCategory,
            name: "category".to_owned(),
            permission_overwrites: Vec::new(),
            position: 2,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "CategoryChannel",
                    len: 6,
                },
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("2"),
                Token::Str("type"),
                Token::U8(4),
                Token::Str("name"),
                Token::Str("category"),
                Token::Str("permission_overwrites"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("position"),
                Token::I64(2),
                Token::StructEnd,
            ],
        );
    }
}
