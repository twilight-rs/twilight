mod guild;
mod metadata;
mod target_user_type;

pub use self::{guild::InviteGuild, metadata::InviteMetadata, target_user_type::TargetUserType};

use super::{channel::Channel, user::User};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Invite {
    pub approximate_member_count: Option<u64>,
    pub approximate_presence_count: Option<u64>,
    pub channel: Channel,
    pub code: String,
    pub guild: Option<InviteGuild>,
    pub inviter: Option<User>,
    pub target_user_type: Option<TargetUserType>,
    pub target_user: Option<User>,
}

#[cfg(test)]
mod tests {
    use super::{Channel, Invite, TargetUserType};
    use crate::{
        channel::{ChannelType, Group},
        id::{ChannelId, UserId},
    };
    use serde_test::Token;

    #[test]
    fn test_invite() {
        let value = Invite {
            approximate_member_count: Some(31),
            approximate_presence_count: Some(7),
            channel: Channel::Group(Group {
                application_id: None,
                icon: None,
                id: ChannelId(2),
                kind: ChannelType::Group,
                last_message_id: None,
                last_pin_timestamp: None,
                name: None,
                owner_id: UserId(3),
                recipients: Vec::new(),
            }),
            code: "uniquecode".to_owned(),
            guild: None,
            inviter: None,
            target_user_type: Some(TargetUserType::Stream),
            target_user: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Invite",
                    len: 8,
                },
                Token::Str("approximate_member_count"),
                Token::Some,
                Token::U64(31),
                Token::Str("approximate_presence_count"),
                Token::Some,
                Token::U64(7),
                Token::Str("channel"),
                Token::Struct {
                    name: "Group",
                    len: 9,
                },
                Token::Str("application_id"),
                Token::None,
                Token::Str("icon"),
                Token::None,
                Token::Str("id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("2"),
                Token::Str("type"),
                Token::U8(3),
                Token::Str("last_message_id"),
                Token::None,
                Token::Str("last_pin_timestamp"),
                Token::None,
                Token::Str("name"),
                Token::None,
                Token::Str("owner_id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("3"),
                Token::Str("recipients"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
                Token::Str("code"),
                Token::Str("uniquecode"),
                Token::Str("guild"),
                Token::None,
                Token::Str("inviter"),
                Token::None,
                Token::Str("target_user_type"),
                Token::Some,
                Token::U8(1),
                Token::Str("target_user"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }
}
