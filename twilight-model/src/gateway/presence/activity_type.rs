use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ActivityType(u8);

impl ActivityType {
    pub const PLAYING: Self = Self::new(0);
    pub const STREAMING: Self = Self::new(1);
    pub const LISTENING: Self = Self::new(2);
    pub const WATCHING: Self = Self::new(3);
    pub const CUSTOM: Self = Self::new(4);
    pub const COMPETING: Self = Self::new(5);

    /// Create a new activity type from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`WATCHING`][`Self::WATCHING`].
    pub const fn new(activity_type: u8) -> Self {
        Self(activity_type)
    }

    /// Retrieve the value of the activity type.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::gateway::presence::ActivityType;
    ///
    /// assert_eq!(2, ActivityType::LISTENING.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }
}

impl Default for ActivityType {
    fn default() -> Self {
        Self::PLAYING
    }
}

impl From<u8> for ActivityType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<ActivityType> for u8 {
    fn from(value: ActivityType) -> Self {
        value.get()
    }
}

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
