use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AfkTimeout(u16);

impl AfkTimeout {
    pub const ONE_MINUTE: Self = Self(60);

    pub const FIVE_MINUTES: Self = Self(300);

    pub const FIFTEEN_MINUTES: Self = Self(900);

    pub const THIRTY_MINUTES: Self = Self(1800);

    pub const ONE_HOUR: Self = Self(3600);

    /// Retrieve the duration of the AFK timeout, in seconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::guild::AfkTimeout;
    ///
    /// assert_eq!(60, AfkTimeout::ONE_MINUTE.get());
    /// ```
    pub const fn get(self) -> u16 {
        self.0
    }
}

impl From<u16> for AfkTimeout {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl From<AfkTimeout> for Duration {
    fn from(value: AfkTimeout) -> Self {
        Self::from_secs(u64::from(value.get()))
    }
}

#[cfg(test)]
mod tests {
    use super::AfkTimeout;
    use serde_test::Token;
    use std::time::Duration;

    const MAP: &[(AfkTimeout, u16)] = &[
        (AfkTimeout::ONE_MINUTE, 60),
        (AfkTimeout::FIVE_MINUTES, 300),
        (AfkTimeout::FIFTEEN_MINUTES, 900),
        (AfkTimeout::THIRTY_MINUTES, 1800),
        (AfkTimeout::ONE_HOUR, 3600),
    ];

    #[test]
    fn consts() {
        for (value, seconds) in MAP {
            serde_test::assert_tokens(
                value,
                &[
                    Token::NewtypeStruct { name: "AfkTimeout" },
                    Token::U16(*seconds),
                ],
            );
            assert_eq!(*value, AfkTimeout::from(*seconds));
            assert_eq!(*seconds, value.get());
        }
    }

    #[test]
    fn std_time_duration() {
        for (kind, _) in MAP {
            let std_duration = Duration::from(*kind);
            assert_eq!(u64::from(kind.get()), std_duration.as_secs());
        }
    }
}
