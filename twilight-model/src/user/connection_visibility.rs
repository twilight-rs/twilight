use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ConnectionVisibility(u8);

impl ConnectionVisibility {
    /// Connection isn't visible to anyone.
    pub const NONE: Self = Self::new(0);

    /// Connection is visible to everyone.
    pub const EVERYONE: Self = Self::new(1);

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::EVERYONE => "EVERYONE",
            Self::NONE => "NONE",
            _ => return None,
        })
    }
}

impl_typed!(ConnectionVisibility, u8);

#[cfg(test)]
mod tests {
    use super::ConnectionVisibility;
    use serde_test::Token;

    const MAP: &[(ConnectionVisibility, u8)] = &[
        (ConnectionVisibility::NONE, 0),
        (ConnectionVisibility::EVERYONE, 1),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "ConnectionVisibility",
                    },
                    Token::U8(*num),
                ],
            );
            assert_eq!(*kind, ConnectionVisibility::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
