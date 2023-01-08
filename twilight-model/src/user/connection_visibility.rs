use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ConnectionVisibility(u8);

impl ConnectionVisibility {
    /// Connection isn't visible to anyone.
    pub const NONE: Self = Self::new(0);

    /// Connection is visible to everyone.
    pub const EVERYONE: Self = Self::new(1);

    /// Create a new connection visibility from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`EVERYONE`][`Self::EVERYONE`].
    pub const fn new(connection_visibility: u8) -> Self {
        Self(connection_visibility)
    }

    /// Retrieve the value of the connection visibility.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::user::ConnectionVisibility;
    ///
    /// assert_eq!(1, ConnectionVisibility::EVERYONE.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }
}

impl From<u8> for ConnectionVisibility {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<ConnectionVisibility> for u8 {
    fn from(value: ConnectionVisibility) -> Self {
        value.get()
    }
}

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
