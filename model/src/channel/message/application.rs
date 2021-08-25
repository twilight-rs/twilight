use crate::id::ApplicationId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageApplication {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<String>,
    pub description: String,
    pub icon: Option<String>,
    pub id: ApplicationId,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::{ApplicationId, MessageApplication};
    use serde_test::Token;

    #[test]
    fn test_message_application() {
        let value = MessageApplication {
            cover_image: Some("cover".to_owned()),
            description: "a description".to_owned(),
            icon: Some("an icon".to_owned()),
            id: ApplicationId::new(1).expect("non zero"),
            name: "application".to_owned(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "MessageApplication",
                    len: 5,
                },
                Token::Str("cover_image"),
                Token::Some,
                Token::Str("cover"),
                Token::Str("description"),
                Token::Str("a description"),
                Token::Str("icon"),
                Token::Some,
                Token::Str("an icon"),
                Token::Str("id"),
                Token::NewtypeStruct {
                    name: "ApplicationId",
                },
                Token::Str("1"),
                Token::Str("name"),
                Token::Str("application"),
                Token::StructEnd,
            ],
        );
    }
}
