use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct DefaultMessageNotificationLevel(u8);

impl DefaultMessageNotificationLevel {
    pub const ALL: Self = Self::new(0);

    pub const MENTIONS: Self = Self::new(1);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::ALL => "ALL",
            Self::MENTIONS => "MENTIONS",
            _ => return None,
        })
    }
}

impl_typed!(DefaultMessageNotificationLevel, u8);

#[cfg(test)]
mod tests {
    use super::DefaultMessageNotificationLevel;
    use serde_test::Token;

    const MAP: &[(DefaultMessageNotificationLevel, u8)] = &[
        (DefaultMessageNotificationLevel::ALL, 0),
        (DefaultMessageNotificationLevel::MENTIONS, 1),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "DefaultMessageNotificationLevel",
                    },
                    Token::U8(*num),
                ],
            );
            assert_eq!(*kind, DefaultMessageNotificationLevel::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
