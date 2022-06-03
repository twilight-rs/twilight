use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct IntegrationAccount {
    pub id: String,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::IntegrationAccount;
    use serde_test::Token;

    #[test]
    fn integration_account() {
        let value = IntegrationAccount {
            id: "account-id".to_owned(),
            name: "account name".to_owned(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "IntegrationAccount",
                    len: 2,
                },
                Token::Str("id"),
                Token::Str("account-id"),
                Token::Str("name"),
                Token::Str("account name"),
                Token::StructEnd,
            ],
        );
    }
}
