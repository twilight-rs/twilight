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
    pub name: String,
    pub owner_user_id: UserId,
}

#[cfg(test)]
mod tests {
    use super::{Team, TeamId, UserId};
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(Team: icon, id, members, name, owner_user_id);

    assert_impl_all!(
        Team: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize
    );

    #[test]
    fn test_team() {
        let value = Team {
            icon: Some("hash".to_owned()),
            id: TeamId::new(1).expect("non zero"),
            members: Vec::new(),
            name: "team name".into(),
            owner_user_id: UserId::new(2).expect("non zero"),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Team",
                    len: 5,
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
                Token::Str("name"),
                Token::Str("team name"),
                Token::Str("owner_user_id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("2"),
                Token::StructEnd,
            ],
        );
    }
}
