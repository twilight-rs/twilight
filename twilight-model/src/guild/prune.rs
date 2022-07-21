use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildPrune {
    pub pruned: u64,
}

#[cfg(test)]
mod tests {
    use super::GuildPrune;
    use serde_test::Token;

    #[test]
    fn guild_prune() {
        let prune = GuildPrune { pruned: 31 };

        serde_test::assert_tokens(
            &prune,
            &[
                Token::Struct {
                    name: "GuildPrune",
                    len: 1,
                },
                Token::Str("pruned"),
                Token::U64(31),
                Token::StructEnd,
            ],
        );
    }
}
