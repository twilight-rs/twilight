use crate::gateway::SessionStartLimit;
use serde::{Deserialize, Serialize};

/// Gateway information containing the recommended shard count and session
/// availability.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct BotConnectionInfo {
    /// Current session availability and session connection concurrency limits.
    pub session_start_limit: SessionStartLimit,
    /// Recommended shard count to use.
    pub shards: u64,
    /// URL to the gateway.
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::{BotConnectionInfo, SessionStartLimit};
    use serde_test::Token;

    #[test]
    fn connection_info() {
        let value = BotConnectionInfo {
            session_start_limit: SessionStartLimit {
                max_concurrency: 16,
                remaining: 998,
                reset_after: 84_686_789,
                total: 1000,
            },
            shards: 3,
            url: "wss://gateway.discord.gg".to_owned(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "BotConnectionInfo",
                    len: 3,
                },
                Token::Str("session_start_limit"),
                Token::Struct {
                    name: "SessionStartLimit",
                    len: 4,
                },
                Token::Str("max_concurrency"),
                Token::U64(16),
                Token::Str("remaining"),
                Token::U64(998),
                Token::Str("reset_after"),
                Token::U64(84_686_789),
                Token::Str("total"),
                Token::U64(1000),
                Token::StructEnd,
                Token::Str("shards"),
                Token::U64(3),
                Token::Str("url"),
                Token::Str("wss://gateway.discord.gg"),
                Token::StructEnd,
            ],
        );
    }
}
