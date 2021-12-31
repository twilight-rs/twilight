use crate::{guild::GuildIntegration, user::ConnectionVisibility};
use serde::{Deserialize, Serialize};

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Connection {
    pub (crate) friend_sync: bool,
    pub (crate) id: String,
    #[serde(default)]
    pub (crate) integrations: Vec<GuildIntegration>,
    #[serde(rename = "type")]
    pub (crate) kind: String,
    pub (crate) name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub (crate) revoked: Option<bool>,
    pub (crate) show_activity: bool,
    pub (crate) verified: bool,
    pub (crate) visibility: ConnectionVisibility,
}

impl Connection {
    pub const fn friend_sync(&self) -> bool {
        self.friend_sync
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn integrations(&self) -> &[GuildIntegration] {
        &self.integrations
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn revoked(&self) -> Option<bool> {
        self.revoked
    }

    pub const fn show_activity(&self) -> bool {
        self.show_activity
    }

    pub const fn verified(&self) -> bool {
        self.verified
    }

    pub const fn visibility(&self) -> ConnectionVisibility {
        self.visibility
    }
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
