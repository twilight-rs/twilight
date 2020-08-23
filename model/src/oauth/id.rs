use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SkuId(#[serde(with = "crate::id::string")] pub u64);

impl Display for SkuId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TeamId(#[serde(with = "crate::id::string")] pub u64);

impl Display for TeamId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod tests {
    use super::{SkuId, TeamId};
    use serde_test::Token;

    #[test]
    fn test_id_deser() {
        serde_test::assert_tokens(
            &SkuId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "SkuId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_tokens(
            &TeamId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "TeamId" },
                Token::Str("114941315417899012"),
            ],
        );
    }
}
