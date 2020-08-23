mod member;
mod membership_state;

pub use self::{member::TeamMember, membership_state::TeamMembershipState};

use crate::{id::UserId, oauth::id::TeamId};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Team {
    pub icon: Option<String>,
    pub id: TeamId,
    pub members: Vec<TeamMember>,
    pub owner_user_id: UserId,
}

#[cfg(test)]
mod tests {
    use super::{Team, TeamId, UserId};
    use serde_test::Token;

    #[test]
    fn test_team() {
        let value = Team {
            icon: Some("hash".to_owned()),
            id: TeamId(1),
            members: Vec::new(),
            owner_user_id: UserId(2),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Team",
                    len: 4,
                },
                Token::Str("icon"),
                Token::Some,
                Token::Str("hash"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "TeamId" },
                Token::Str("1"),
                Token::Str("members"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("owner_user_id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("2"),
                Token::StructEnd,
            ],
        );
    }
}
