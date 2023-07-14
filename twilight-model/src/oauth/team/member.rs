use crate::{
    id::{marker::OauthTeamMarker, Id},
    oauth::team::TeamMembershipState,
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct TeamMember {
    pub membership_state: TeamMembershipState,
    pub permissions: Vec<String>,
    pub team_id: Id<OauthTeamMarker>,
    pub user: User,
}

#[cfg(test)]
mod tests {
    use super::{TeamMember, TeamMembershipState, User};
    use crate::id::Id;
    use serde_test::Token;

    #[test]
    fn team_member() {
        let value = TeamMember {
            membership_state: TeamMembershipState::Accepted,
            permissions: vec!["*".to_owned()],
            team_id: Id::new(1),
            user: User {
                accent_color: None,
                avatar: None,
                avatar_decoration: None,
                banner: None,
                bot: false,
                discriminator: 1,
                email: None,
                flags: None,
                global_name: Some("test".to_owned()),
                id: Id::new(2),
                locale: None,
                mfa_enabled: None,
                name: "test".to_owned(),
                premium_type: None,
                public_flags: None,
                system: None,
                verified: None,
            },
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "TeamMember",
                    len: 4,
                },
                Token::Str("membership_state"),
                Token::U8(2),
                Token::Str("permissions"),
                Token::Seq { len: Some(1) },
                Token::Str("*"),
                Token::SeqEnd,
                Token::Str("team_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("user"),
                Token::Struct {
                    name: "User",
                    len: 8,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::None,
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("global_name"),
                Token::Some,
                Token::Str("test"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::StructEnd,
            ],
        );
    }
}
