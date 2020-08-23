use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ActivityTimestamps {
    pub end: Option<u64>,
    pub start: Option<u64>,
}

#[cfg(test)]
mod tests {
    use super::ActivityTimestamps;
    use serde_test::Token;

    #[test]
    fn test_empty() {
        let value = ActivityTimestamps {
            end: None,
            start: None,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ActivityTimestamps",
                    len: 2,
                },
                Token::Str("end"),
                Token::None,
                Token::Str("start"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_start() {
        let value = ActivityTimestamps {
            end: Some(1),
            start: None,
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
                Token::U64(1),
                Token::Str("start"),
                Token::None,
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_end() {
        let value = ActivityTimestamps {
            end: None,
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
                Token::None,
                Token::Str("start"),
                Token::Some,
                Token::U64(1),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_present() {
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
