mod bot_connection_info;

pub use self::bot_connection_info::BotConnectionInfo;

use serde::{Deserialize, Serialize};

/// Gateway information containing the URL to connect to.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct ConnectionInfo {
    /// URL to the gateway.
    pub url: String,
}

#[cfg(test)]
mod tests {
    use super::ConnectionInfo;
    use serde_test::Token;

    #[test]
    fn connection_info() {
        let value = ConnectionInfo {
            url: "wss://gateway.discord.gg".to_owned(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ConnectionInfo",
                    len: 1,
                },
                Token::Str("url"),
                Token::Str("wss://gateway.discord.gg"),
                Token::StructEnd,
            ],
        );
    }
}
