use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ActivityType(u8);

impl ActivityType {
    pub const PLAYING: Self = Self::new(0);
    pub const STREAMING: Self = Self::new(1);
    pub const LISTENING: Self = Self::new(2);
    pub const WATCHING: Self = Self::new(3);
    pub const CUSTOM: Self = Self::new(4);
    pub const COMPETING: Self = Self::new(5);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::PLAYING => "PLAYING",
            Self::STREAMING => "STREAMING",
            Self::LISTENING => "LISTENING",
            Self::WATCHING => "WATCHING",
            Self::CUSTOM => "CUSTOM",
            Self::COMPETING => "COMPETING",
            _ => return None,
        })
    }
}

impl Default for ActivityType {
    fn default() -> Self {
        Self::PLAYING
    }
}

impl_typed!(ActivityType, u8);

#[cfg(test)]
mod tests {
    use super::ActivityType;
    use serde_test::Token;

    const MAP: &[(ActivityType, u8)] = &[
        (ActivityType::PLAYING, 0),
        (ActivityType::STREAMING, 1),
        (ActivityType::LISTENING, 2),
        (ActivityType::WATCHING, 3),
        (ActivityType::CUSTOM, 4),
        (ActivityType::COMPETING, 5),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "ActivityType",
                    },
                    Token::U8(*num),
                ],
            );
            assert_eq!(*kind, ActivityType::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
