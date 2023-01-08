use crate::gateway::presence::Status;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ClientStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desktop: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<Status>,
}

#[cfg(test)]
mod tests {
    use super::{ClientStatus, Status};
    use serde_test::Token;

    #[test]
    fn mobile_online() {
        let value = ClientStatus {
            desktop: Some(Status::IDLE),
            mobile: Some(Status::ONLINE),
            web: Some(Status::DO_NOT_DISTURB),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ClientStatus",
                    len: 3,
                },
                Token::Str("desktop"),
                Token::Some,
                Token::NewtypeStruct { name: "Status" },
                Token::Str("idle"),
                Token::Str("mobile"),
                Token::Some,
                Token::NewtypeStruct { name: "Status" },
                Token::Str("online"),
                Token::Str("web"),
                Token::Some,
                Token::NewtypeStruct { name: "Status" },
                Token::Str("dnd"),
                Token::StructEnd,
            ],
        );
    }
}
