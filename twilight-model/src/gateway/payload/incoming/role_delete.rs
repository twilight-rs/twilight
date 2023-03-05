use crate::id::{
    marker::{GuildMarker, RoleMarker},
    Id,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub struct RoleDelete {
    pub guild_id: Id<GuildMarker>,
    pub role_id: Id<RoleMarker>,
}

#[cfg(test)]
mod tests {
    use super::RoleDelete;
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn webhooks_update() {
        let value = RoleDelete {
            guild_id: Id::new(1),
            role_id: Id::new(2),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "RoleDelete",
                    len: 2,
                },
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("role_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::StructEnd,
            ],
        );
    }
}
