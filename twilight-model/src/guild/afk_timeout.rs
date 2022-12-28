use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Duration of a user being AFK before being timed out from a voice channel.
///
/// This value is configured [for guilds][`Guild::afk_timeout`].
///
/// # Examples
///
/// ```
/// use twilight_model::guild::AfkTimeout;
///
/// assert_eq!(300, AfkTimeout::FIVE_MINUTES);
/// ```
///
/// [`Guild::afk_timeout`]: super::Guild::afk_timeout
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AfkTimeout(u16);

impl AfkTimeout {
    /// AFK timeout of one minute.
    pub const ONE_MINUTE: Self = Self(60);

    /// AFK timeout of five minutes.
    pub const FIVE_MINUTES: Self = Self(300);

    /// AFK timeout of fifteen minutes.
    pub const FIFTEEN_MINUTES: Self = Self(900);

    /// AFK timeout of thirty minutes.
    pub const THIRTY_MINUTES: Self = Self(1800);

    /// AFK timeout of one hour.
    pub const ONE_HOUR: Self = Self(3600);

    /// Retrieve the duration of the AFK timeout in seconds.
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

impl PartialEq<u16> for AfkTimeout {
    fn eq(&self, other: &u16) -> bool {
        self.get() == *other
    }
}

impl PartialEq<AfkTimeout> for u16 {
    fn eq(&self, other: &AfkTimeout) -> bool {
        *self == other.get()
    }
}

#[cfg(test)]
mod tests {
    use super::AfkTimeout;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash, time::Duration};

    assert_impl_all!(
        AfkTimeout: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    const MAP: &[(AfkTimeout, u16)] = &[
        (AfkTimeout::ONE_MINUTE, 60),
        (AfkTimeout::FIVE_MINUTES, 300),
        (AfkTimeout::FIFTEEN_MINUTES, 900),
        (AfkTimeout::THIRTY_MINUTES, 1800),
        (AfkTimeout::ONE_HOUR, 3600),
    ];

    #[test]
    fn serde() {
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

    /// Test two-way equality implementation.
    #[test]
    fn eq() {
        assert_eq!(300, AfkTimeout::FIVE_MINUTES);
        assert_eq!(AfkTimeout::FIVE_MINUTES, 300);
    }

    /// Test conversion to [`std::time::Duration`].
    #[test]
    fn std_time_duration() {
        for (kind, _) in MAP {
            let std_duration = Duration::from(*kind);
            assert_eq!(u64::from(kind.get()), std_duration.as_secs());
        }
    }
}
