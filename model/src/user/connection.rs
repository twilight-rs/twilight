use crate::{guild::GuildIntegration, user::ConnectionVisibility};
use serde::{Deserialize, Serialize};

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Connection {
    pub friend_sync: bool,
    pub id: String,
    #[serde(default)]
    pub integrations: Vec<GuildIntegration>,
    #[serde(rename = "type")]
    pub kind: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked: Option<bool>,
    pub show_activity: bool,
    pub verified: bool,
    pub visibility: ConnectionVisibility,
}

#[cfg(test)]
mod tests {
    use super::{Connection, ConnectionVisibility};
    use serde_test::Token;

    #[test]
    fn test_connection() {
        let value = Connection {
            friend_sync: true,
            id: "connection id".to_owned(),
            integrations: Vec::new(),
            kind: "integration type".to_owned(),
            name: "integration name".to_owned(),
            revoked: Some(false),
            show_activity: true,
            verified: true,
            visibility: ConnectionVisibility::Everyone,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Connection",
                    len: 9,
                },
                Token::Str("friend_sync"),
                Token::Bool(true),
                Token::Str("id"),
                Token::Str("connection id"),
                Token::Str("integrations"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("type"),
                Token::Str("integration type"),
                Token::Str("name"),
                Token::Str("integration name"),
                Token::Str("revoked"),
                Token::Some,
                Token::Bool(false),
                Token::Str("show_activity"),
                Token::Bool(true),
                Token::Str("verified"),
                Token::Bool(true),
                Token::Str("visibility"),
                Token::U8(1),
                Token::StructEnd,
            ],
        );
    }
}
