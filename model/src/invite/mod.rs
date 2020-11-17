mod channel;
mod guild;
mod metadata;
mod target_user_type;

pub use self::{
    channel::InviteChannel, guild::InviteGuild, metadata::InviteMetadata,
    target_user_type::TargetUserType,
};

use super::user::User;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Invite {
    pub approximate_member_count: Option<u64>,
    pub approximate_presence_count: Option<u64>,
    pub channel: InviteChannel,
    pub code: String,
    pub guild: Option<InviteGuild>,
    pub inviter: Option<User>,
    pub target_user_type: Option<TargetUserType>,
    pub target_user: Option<User>,
}

#[cfg(test)]
mod tests {
    use super::{Invite, InviteChannel, TargetUserType};
    use crate::{channel::ChannelType, id::ChannelId};
    use serde_test::Token;

    #[test]
    fn test_invite() {
        let value = Invite {
            approximate_member_count: Some(31),
            approximate_presence_count: Some(7),
            channel: InviteChannel {
                id: ChannelId(2),
                kind: ChannelType::Group,
                name: None,
            },
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
                    name: "InviteChannel",
                    len: 3,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("2"),
                Token::Str("name"),
                Token::None,
                Token::Str("type"),
                Token::U8(3),
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
