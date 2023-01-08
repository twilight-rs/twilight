use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct DefaultMessageNotificationLevel(u8);

impl DefaultMessageNotificationLevel {
    pub const ALL: Self = Self::new(0);

    pub const MENTIONS: Self = Self::new(1);

    /// Create a new command type from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`ALL`][`Self::ALL`].
    pub const fn new(default_message_notification_level: u8) -> Self {
        Self(default_message_notification_level)
    }

    /// Retrieve the value of the command type.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::guild::DefaultMessageNotificationLevel;
    ///
    /// assert_eq!(1, DefaultMessageNotificationLevel::MENTIONS.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }
}

impl From<u8> for DefaultMessageNotificationLevel {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<DefaultMessageNotificationLevel> for u8 {
    fn from(value: DefaultMessageNotificationLevel) -> Self {
        value.get()
    }
}

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
