use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize_repr,
)]
#[repr(u8)]
pub enum DefaultMessageNotificationLevel {
    All = 0,
    Mentions = 1,
}

#[cfg(test)]
mod tests {
    use super::DefaultMessageNotificationLevel;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&DefaultMessageNotificationLevel::All, &[Token::U8(0)]);
        serde_test::assert_tokens(&DefaultMessageNotificationLevel::Mentions, &[Token::U8(1)]);
    }
}
