use serde::{Deserialize, Serialize};

/// Information about a guild's vanity URL setting.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VanityUrl {
    /// Code of the vanity URL.
    ///
    /// For example, in an invite of `discord.gg/applejack`, the code is
    /// `applejack`.
    pub code: String,
    /// Number of times the vanity URL has been used.
    pub uses: u64,
}

#[cfg(test)]
mod tests {
    use super::VanityUrl;
    use serde_test::Token;

    #[test]
    fn vanity_url() {
        let url = VanityUrl {
            code: "a".to_owned(),
            uses: 12,
        };
        serde_test::assert_tokens(
            &url,
            &[
                Token::Struct {
                    name: "VanityUrl",
                    len: 2,
                },
                Token::String("code"),
                Token::String("a"),
                Token::String("uses"),
                Token::U64(12),
                Token::StructEnd,
            ],
        );
    }
}
