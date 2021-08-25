use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    num::NonZeroU64,
};

/// Unique ID denoting a sticker.
///
/// # serde
///
/// Like all of the IDs in the primary [`crate::id`] crate, these
/// IDs support deserializing from both integers and strings and serialize into
/// strings.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct StickerId(#[serde(with = "crate::id::string")] pub NonZeroU64);

impl StickerId {
    /// Create a non-zero application ID without checking the value.
    ///
    /// Equivalent to [`NonZeroU64::new_unchecked`].
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(n: u64) -> Self {
        Self(NonZeroU64::new_unchecked(n))
    }

    /// Create a non-zero application ID if the given value is not zero.
    ///
    /// Equivalent to [`NonZeroU64::new`].
    pub const fn new(n: u64) -> Option<Self> {
        #[allow(clippy::option_if_let_else)]
        if let Some(n) = NonZeroU64::new(n) {
            Some(Self(n))
        } else {
            None
        }
    }

    /// Return the inner primitive value.
    ///
    /// Equivalent to [`NonZeroU64::get`].
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl Display for StickerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

/// Unique ID denoting a sticker pack.
///
/// # serde
///
/// Like all of the IDs in the primary [`crate::id`] crate, these
/// IDs support deserializing from both integers and strings and serialize into
/// strings.
#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct StickerPackId(#[serde(with = "crate::id::string")] pub NonZeroU64);

impl StickerPackId {
    /// Create a non-zero application ID without checking the value.
    ///
    /// Equivalent to [`NonZeroU64::new_unchecked`].
    ///
    /// # Safety
    ///
    /// The value must not be zero.
    #[allow(unsafe_code)]
    pub const unsafe fn new_unchecked(n: u64) -> Self {
        Self(NonZeroU64::new_unchecked(n))
    }

    /// Create a non-zero application ID if the given value is not zero.
    ///
    /// Equivalent to [`NonZeroU64::new`].
    pub const fn new(n: u64) -> Option<Self> {
        #[allow(clippy::option_if_let_else)]
        if let Some(n) = NonZeroU64::new(n) {
            Some(Self(n))
        } else {
            None
        }
    }

    /// Return the inner primitive value.
    ///
    /// Equivalent to [`NonZeroU64::get`].
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl Display for StickerPackId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod tests {
    use super::{StickerId, StickerPackId};
    use serde_test::Token;

    #[test]
    fn test_id_deser() {
        serde_test::assert_tokens(
            &StickerId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "StickerId" },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &StickerId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct { name: "StickerId" },
                Token::U64(114_941_315_417_899_012),
            ],
        );
        serde_test::assert_tokens(
            &StickerPackId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct {
                    name: "StickerPackId",
                },
                Token::Str("114941315417899012"),
            ],
        );
        serde_test::assert_de_tokens(
            &StickerPackId::new(114_941_315_417_899_012).expect("non zero"),
            &[
                Token::NewtypeStruct {
                    name: "StickerPackId",
                },
                Token::U64(114_941_315_417_899_012),
            ],
        );
    }
}
