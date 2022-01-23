use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Ban {
    pub reason: Option<String>,
    pub user: User,
}

#[cfg(test)]
mod tests {
    use super::{Ban, User};
    use crate::{id::Id, test::image_hash};
    use serde_test::Token;

    #[test]
    fn test_ban() {
        let ban = Ban {
            reason: Some("foo".to_owned()),
            user: User {
                accent_color: None,
                avatar: Some(image_hash::AVATAR),
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                id: Id::new(100_000_000_000_000_000),
                locale: None,
                mfa_enabled: None,
                name: "test".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            },
        };

        serde_test::assert_de_tokens(
            &ban,
            &[
                Token::Struct {
                    name: "Ban",
                    len: 2,
                },
                Token::Str("reason"),
                Token::Some,
                Token::Str("foo"),
                Token::Str("user"),
                Token::Struct {
                    name: "User",
                    len: 7,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str(image_hash::AVATAR_INPUT),
                Token::Str("banner"),
                Token::None,
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("100000000000000000"),
                Token::Str("public_flags"),
                Token::None,
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
