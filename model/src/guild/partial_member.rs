use crate::{
    datetime::Timestamp,
    guild::Permissions,
    id::{marker::RoleMarker, Id},
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PartialMember {
    pub deaf: bool,
    pub joined_at: Option<Timestamp>,
    pub mute: bool,
    pub nick: Option<String>,
    /// Permission data for the member.
    ///
    /// Sent in an [`Interaction`].
    ///
    /// [`Interaction`]: crate::application::interaction::Interaction
    pub permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<Timestamp>,
    pub roles: Vec<Id<RoleMarker>>,
    pub user: Option<User>,
}

#[cfg(test)]
mod tests {
    use super::PartialMember;
    use crate::{
        datetime::{Timestamp, TimestampParseError},
        id::Id,
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[test]
    fn test_partial_member() -> Result<(), TimestampParseError> {
        let joined_at = Timestamp::from_str("2015-04-26T06:26:56.936000+00:00")?;

        let value = PartialMember {
            deaf: false,
            joined_at: Some(joined_at),
            mute: true,
            nick: Some("a nickname".to_owned()),
            permissions: None,
            premium_since: None,
            roles: vec![Id::new(1).expect("non zero")],
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
                Token::Str("2015-04-26T06:26:56.936000+00:00"),
                Token::Str("mute"),
                Token::Bool(true),
                Token::Str("nick"),
                Token::Some,
                Token::Str("a nickname"),
                Token::Str("permissions"),
                Token::None,
                Token::Str("roles"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::SeqEnd,
                Token::Str("user"),
                Token::None,
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
