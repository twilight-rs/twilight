//! IDs for OAuth teams.
//!
//! # serde
//!
//! Like all of the IDs in the primary [`id`] crate, these IDs support
//! deserializing from both integers and strings and serialize into strings.
//!
//! [`id`]: ../id

use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    num::NonZeroU64,
};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
// *not* a snowflake
pub struct SkuId(#[serde(with = "crate::id::string")] pub u64);

impl Display for SkuId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct TeamId(#[serde(with = "crate::id::string")] pub NonZeroU64);

impl Display for TeamId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod tests {
    use super::{SkuId, TeamId};
    use serde_test::Token;
    use std::num::NonZeroU64;

    #[test]
    fn test_id_deser() {
        serde_test::assert_tokens(
            &SkuId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "SkuId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &SkuId(114_941_315_417_899_012),
            &[
                Token::NewtypeStruct { name: "SkuId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &TeamId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct { name: "TeamId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &TeamId(NonZeroU64::new(114_941_315_417_899_012).expect("non zero")),
            &[
                Token::NewtypeStruct { name: "TeamId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
    }
}
