use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ActivityTimestamps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::ActivityTimestamps;
    use serde_test::Token;

    #[test]
    fn empty() {
        let value = ActivityTimestamps {
            end: None,
            start: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ActivityTimestamps",
                    len: 0,
                },
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn start() {
        let value = ActivityTimestamps {
            end: Some(1),
            start: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ActivityTimestamps",
                    len: 1,
                },
                Token::Str("end"),
                Token::Some,
                Token::U64(1),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn end() {
        let value = ActivityTimestamps {
            end: None,
            start: Some(1),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ActivityTimestamps",
                    len: 1,
                },
                Token::Str("start"),
                Token::Some,
                Token::U64(1),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn present() {
        let value = ActivityTimestamps {
            end: Some(2),
            start: Some(1),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ActivityTimestamps",
                    len: 2,
                },
                Token::Str("end"),
                Token::Some,
                Token::U64(2),
                Token::Str("start"),
                Token::Some,
                Token::U64(1),
                Token::StructEnd,
            ],
        );
    }
}
