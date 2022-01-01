use crate::{
    channel::ChannelType,
    datetime::Timestamp,
    id::{
        marker::{ApplicationMarker, ChannelMarker, MessageMarker, UserMarker},
        Id,
    },
    user::User,
    util::image_hash::ImageHash,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Group {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Id<ApplicationMarker>>,
    pub icon: Option<ImageHash>,
    pub id: Id<ChannelMarker>,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_id: Option<Id<MessageMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub owner_id: Id<UserMarker>,
    pub recipients: Vec<User>,
}

#[cfg(test)]
mod tests {
    use super::{ChannelType, Group};
    use crate::{
        datetime::{Timestamp, TimestampParseError},
        id::Id,
        test::image_hash,
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[test]
    fn test_group() {
        let value = Group {
            application_id: Some(Id::new(1)),
            icon: Some(image_hash::ICON),
            id: Id::new(2),
            kind: ChannelType::Group,
            last_message_id: Some(Id::new(3)),
            last_pin_timestamp: None,
            name: Some("a group".to_owned()),
            owner_id: Id::new(4),
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
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("icon"),
                Token::Some,
                Token::Str(image_hash::ICON_INPUT),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("type"),
                Token::U8(3),
                Token::Str("last_message_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("name"),
                Token::Some,
                Token::Str("a group"),
                Token::Str("owner_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::Str("recipients"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_group_complete() -> Result<(), TimestampParseError> {
        let timestamp = Timestamp::from_str("2021-08-10T12:21:10+00:00")?;

        let value = Group {
            application_id: Some(Id::new(1)),
            icon: Some(image_hash::ICON),
            id: Id::new(2),
            kind: ChannelType::Group,
            last_message_id: Some(Id::new(3)),
            last_pin_timestamp: Some(timestamp),
            name: Some("a group".to_owned()),
            owner_id: Id::new(4),
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
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("icon"),
                Token::Some,
                Token::Str(image_hash::ICON_INPUT),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("type"),
                Token::U8(3),
                Token::Str("last_message_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("last_pin_timestamp"),
                Token::Some,
                Token::Str("2021-08-10T12:21:10.000000+00:00"),
                Token::Str("name"),
                Token::Some,
                Token::Str("a group"),
                Token::Str("owner_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::Str("recipients"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
