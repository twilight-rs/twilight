use serde::{Deserialize, Serialize};

/// Current gateway session utilization status.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct SessionStartLimit {
    /// Maximum number of session that may be started concurrently.
    pub max_concurrency: u8,
    /// Number of remaining sessions for a given time period.
    ///
    /// Max 1999.
    pub remaining: u16,
    /// Milliseconds until `remaining` resets back to `total`.
    pub reset_after: u64,
    /// Total number of sessions that can be started within the given time
    /// period.
    ///
    /// Max 2000.
    pub total: u16,
}

#[cfg(test)]
mod tests {
    use super::SessionStartLimit;
    use serde_test::Token;

    #[test]
    fn connection_info() {
        let value = SessionStartLimit {
            max_concurrency: 16,
            remaining: 998,
            reset_after: 84_686_789,
            total: 1_000,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "SessionStartLimit",
                    len: 4,
                },
                Token::Str("max_concurrency"),
                Token::U8(16),
                Token::Str("remaining"),
                Token::U16(998),
                Token::Str("reset_after"),
                Token::U64(84_686_789),
                Token::Str("total"),
                Token::U16(1_000),
                Token::StructEnd,
            ],
        );
    }
}
