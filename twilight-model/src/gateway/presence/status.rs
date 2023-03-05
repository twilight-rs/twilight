use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize),
    archive(as = "Self")
)]
pub enum Status {
    #[serde(rename = "dnd")]
    DoNotDisturb,
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "invisible")]
    Invisible,
    #[serde(rename = "offline")]
    Offline,
    #[serde(rename = "online")]
    Online,
}

#[cfg(test)]
mod tests {
    use super::Status;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(
            &Status::DoNotDisturb,
            &[Token::UnitVariant {
                name: "Status",
                variant: "dnd",
            }],
        );
        serde_test::assert_tokens(
            &Status::Idle,
            &[Token::UnitVariant {
                name: "Status",
                variant: "idle",
            }],
        );
        serde_test::assert_tokens(
            &Status::Invisible,
            &[Token::UnitVariant {
                name: "Status",
                variant: "invisible",
            }],
        );
        serde_test::assert_tokens(
            &Status::Offline,
            &[Token::UnitVariant {
                name: "Status",
                variant: "offline",
            }],
        );
        serde_test::assert_tokens(
            &Status::Online,
            &[Token::UnitVariant {
                name: "Status",
                variant: "online",
            }],
        );
    }
}
