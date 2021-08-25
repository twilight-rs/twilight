use crate::{
    channel::ChannelType,
    id::{ApplicationId, ChannelId, MessageId, UserId},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Group {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,
    pub icon: Option<String>,
    pub id: ChannelId,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub owner_id: UserId,
    pub recipients: Vec<User>,
}

#[cfg(test)]
mod tests {
    use super::{ApplicationId, ChannelId, ChannelType, Group, MessageId, UserId};
    use serde_test::Token;

    #[test]
    fn test_group() {
        let value = Group {
            application_id: Some(ApplicationId::new(1).expect("non zero")),
            icon: Some("icon hash".to_owned()),
            id: ChannelId::new(2).expect("non zero"),
            kind: ChannelType::Group,
            last_message_id: Some(MessageId::new(3).expect("non zero")),
            last_pin_timestamp: None,
            name: Some("a group".to_owned()),
            owner_id: UserId::new(4).expect("non zero"),
            recipients: Vec::new(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Group",
                    len: 8,
                },
                Token::Str("application_id"),
                Token::Some,
                Token::NewtypeStruct {
                    name: "ApplicationId",
                },
                Token::Str("1"),
                Token::Str("icon"),
                Token::Some,
                Token::Str("icon hash"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("2"),
                Token::Str("type"),
                Token::U8(3),
                Token::Str("last_message_id"),
                Token::Some,
                Token::NewtypeStruct { name: "MessageId" },
                Token::Str("3"),
                Token::Str("name"),
                Token::Some,
                Token::Str("a group"),
                Token::Str("owner_id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("4"),
                Token::Str("recipients"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_group_complete() {
        let value = Group {
            application_id: Some(ApplicationId::new(1).expect("non zero")),
            icon: Some("icon hash".to_owned()),
            id: ChannelId::new(2).expect("non zero"),
            kind: ChannelType::Group,
            last_message_id: Some(MessageId::new(3).expect("non zero")),
            last_pin_timestamp: Some("123".to_owned()),
            name: Some("a group".to_owned()),
            owner_id: UserId::new(4).expect("non zero"),
            recipients: Vec::new(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Group",
                    len: 9,
                },
                Token::Str("application_id"),
                Token::Some,
                Token::NewtypeStruct {
                    name: "ApplicationId",
                },
                Token::Str("1"),
                Token::Str("icon"),
                Token::Some,
                Token::Str("icon hash"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("2"),
                Token::Str("type"),
                Token::U8(3),
                Token::Str("last_message_id"),
                Token::Some,
                Token::NewtypeStruct { name: "MessageId" },
                Token::Str("3"),
                Token::Str("last_pin_timestamp"),
                Token::Some,
                Token::Str("123"),
                Token::Str("name"),
                Token::Some,
                Token::Str("a group"),
                Token::Str("owner_id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("4"),
                Token::Str("recipients"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }
}
