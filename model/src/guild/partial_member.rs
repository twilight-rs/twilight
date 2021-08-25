use crate::{guild::Permissions, id::RoleId, user::User};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PartialMember {
    pub deaf: bool,
    pub joined_at: Option<String>,
    pub mute: bool,
    pub nick: Option<String>,
    /// Permission data for the member.
    ///
    /// Sent in an [`Interaction`].
    ///
    /// [`Interaction`]: crate::application::interaction::Interaction
    pub permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<String>,
    pub roles: Vec<RoleId>,
    pub user: Option<User>,
}

#[cfg(test)]
mod tests {
    use super::{PartialMember, RoleId};
    use serde_test::Token;

    #[test]
    fn test_partial_member() {
        let value = PartialMember {
            deaf: false,
            joined_at: Some("timestamp".to_owned()),
            mute: true,
            nick: Some("a nickname".to_owned()),
            permissions: None,
            premium_since: None,
            roles: vec![RoleId::new(1).expect("non zero")],
            user: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "PartialMember",
                    len: 7,
                },
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("joined_at"),
                Token::Some,
                Token::Str("timestamp"),
                Token::Str("mute"),
                Token::Bool(true),
                Token::Str("nick"),
                Token::Some,
                Token::Str("a nickname"),
                Token::Str("permissions"),
                Token::None,
                Token::Str("roles"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("1"),
                Token::SeqEnd,
                Token::Str("user"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }
}
