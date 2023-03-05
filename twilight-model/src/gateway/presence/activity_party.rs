use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
)]
pub struct ActivityParty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<[u64; 2]>,
}

#[cfg(test)]
mod tests {
    use super::ActivityParty;
    use serde_test::Token;

    #[test]
    fn activity_secrets() {
        let value = ActivityParty {
            id: Some("party id".to_owned()),
            size: Some([2, 6]),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ActivityParty",
                    len: 2,
                },
                Token::Str("id"),
                Token::Some,
                Token::Str("party id"),
                Token::Str("size"),
                Token::Some,
                Token::Tuple { len: 2 },
                Token::U64(2),
                Token::U64(6),
                Token::TupleEnd,
                Token::StructEnd,
            ],
        );
    }
}
