mod member;
mod membership_state;

pub use self::{member::TeamMember, membership_state::TeamMembershipState};

use crate::{
    id::{
        Id,
        marker::{OauthTeamMarker, UserMarker},
    },
    util::image_hash::ImageHash,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Team {
    pub icon: Option<ImageHash>,
    pub id: Id<OauthTeamMarker>,
    pub members: Vec<TeamMember>,
    pub name: String,
    pub owner_user_id: Id<UserMarker>,
}

#[cfg(test)]
mod tests {
    use super::Team;
    use crate::{id::Id, test::image_hash};
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
    fn team() {
        let value = Team {
            icon: Some(image_hash::ICON),
            id: Id::new(1),
            members: Vec::new(),
            name: "team name".into(),
            owner_user_id: Id::new(2),
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
                Token::Str(image_hash::ICON_INPUT),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("members"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("name"),
                Token::Str("team name"),
                Token::Str("owner_user_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::StructEnd,
            ],
        );
    }
}
