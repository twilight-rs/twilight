use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum InviteType {
    Guild,
    GroupDM,
    Friend,
    Unknown(u8),
}

impl From<u8> for InviteType {
    fn from(value: u8) -> Self {
        match value {
            0 => InviteType::Guild,
            1 => InviteType::GroupDM,
            2 => InviteType::Friend,
            unknown => InviteType::Unknown(unknown),
        }
    }
}

impl From<InviteType> for u8 {
    fn from(value: InviteType) -> Self {
        match value {
            InviteType::Guild => 0,
            InviteType::GroupDM => 1,
            InviteType::Friend => 2,
            InviteType::Unknown(unknown) => unknown,
        }
    }
}

impl InviteType {
    pub const fn name(&self) -> &str {
        match self {
            Self::Guild => "Guild",
            Self::GroupDM => "Group DM",
            Self::Friend => "Friend",
            Self::Unknown(_) => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InviteType;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&InviteType::Guild, &[Token::U8(0)]);
        serde_test::assert_tokens(&InviteType::GroupDM, &[Token::U8(1)]);
        serde_test::assert_tokens(&InviteType::Friend, &[Token::U8(2)]);
        serde_test::assert_tokens(&InviteType::Unknown(99), &[Token::U8(99)]);
    }

    #[test]
    fn names() {
        assert_eq!(InviteType::Guild.name(), "Guild");
        assert_eq!(InviteType::GroupDM.name(), "Group DM");
        assert_eq!(InviteType::Friend.name(), "Friend");
        assert_eq!(InviteType::Unknown(99).name(), "Unknown");
    }
}
