use crate::gateway::presence::Status;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ClientStatus {
    pub desktop: Option<Status>,
    pub mobile: Option<Status>,
    pub web: Option<Status>,
}

#[cfg(test)]
mod tests {
    use super::{ClientStatus, Status};
    use serde_test::Token;

    #[test]
    fn test_mobile_online() {
        let status = ClientStatus {
            desktop: None,
            mobile: Some(Status::Online),
            web: None,
        };

        serde_test::assert_tokens(
            &status,
            &[
                Token::Struct {
                    name: "ClientStatus",
                    len: 3,
                },
                Token::Str("desktop"),
                Token::None,
                Token::Str("mobile"),
                Token::Some,
                Token::Enum { name: "Status" },
                Token::Str("online"),
                Token::Unit,
                Token::Str("web"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }
}
