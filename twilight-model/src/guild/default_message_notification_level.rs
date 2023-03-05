use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub enum DefaultMessageNotificationLevel {
    All,
    Mentions,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for DefaultMessageNotificationLevel {
    fn from(value: u8) -> Self {
        match value {
            0 => DefaultMessageNotificationLevel::All,
            1 => DefaultMessageNotificationLevel::Mentions,
            unknown => DefaultMessageNotificationLevel::Unknown(unknown),
        }
    }
}

impl From<DefaultMessageNotificationLevel> for u8 {
    fn from(value: DefaultMessageNotificationLevel) -> Self {
        match value {
            DefaultMessageNotificationLevel::All => 0,
            DefaultMessageNotificationLevel::Mentions => 1,
            DefaultMessageNotificationLevel::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DefaultMessageNotificationLevel;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&DefaultMessageNotificationLevel::All, &[Token::U8(0)]);
        serde_test::assert_tokens(&DefaultMessageNotificationLevel::Mentions, &[Token::U8(1)]);
        serde_test::assert_tokens(
            &DefaultMessageNotificationLevel::Unknown(99),
            &[Token::U8(99)],
        );
    }
}
