use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    time::Duration,
};

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AutoArchiveDuration(u16);

impl AutoArchiveDuration {
    pub const HOUR: Self = Self::new(60);

    pub const DAY: Self = Self::new(Self::HOUR.get() * 24);

    pub const THREE_DAYS: Self = Self::new(Self::DAY.get() * 3);

    pub const WEEK: Self = Self::new(Self::DAY.get() * 7);

    /// Create a new auto archive duration from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants, such as [`HOUR`][`Self::HOUR`] or [`WEEK`][`Self::WEEK`].
    pub const fn new(auto_archive_duration: u16) -> Self {
        Self(auto_archive_duration)
    }

    /// Retrieve the value of the auto archive duration.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::channel::thread::AutoArchiveDuration;
    ///
    /// assert_eq!(60, AutoArchiveDuration::HOUR.get());
    /// ```
    pub const fn get(&self) -> u16 {
        self.0
    }

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::HOUR => "HOUR",
            Self::DAY => "DAY",
            Self::THREE_DAYS => "THREE_DAYS",
            Self::WEEK => "WEEK",
            _ => return None,
        })
    }
}

impl Debug for AutoArchiveDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(name) = self.name() {
            f.debug_struct("AutoArchiveDuration")
                .field("name", &name)
                .field("value", &self.0)
                .finish()
        } else {
            f.debug_tuple("AutoArchiveDuration").field(&self.0).finish()
        }
    }
}

impl From<u16> for AutoArchiveDuration {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl From<AutoArchiveDuration> for u16 {
    fn from(value: AutoArchiveDuration) -> Self {
        value.get()
    }
}

impl From<AutoArchiveDuration> for Duration {
    fn from(value: AutoArchiveDuration) -> Self {
        Self::from_secs(u64::from(value.get()) * 60)
    }
}

#[cfg(test)]
mod tests {
    use super::AutoArchiveDuration;
    use serde_test::Token;
    use std::time::Duration;

    const MAP: &[(AutoArchiveDuration, u16)] = &[
        (AutoArchiveDuration::HOUR, 60),
        (AutoArchiveDuration::DAY, 1440),
        (AutoArchiveDuration::THREE_DAYS, 4320),
        (AutoArchiveDuration::WEEK, 10080),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "AutoArchiveDuration",
                    },
                    Token::U16(*num),
                ],
            );
            assert_eq!(*kind, AutoArchiveDuration::from(*num));
            assert_eq!(*num, kind.get());
        }
    }

    #[test]
    fn std_time_duration() {
        for (kind, _) in MAP {
            let std_duration = Duration::from(*kind);
            assert_eq!(u64::from(kind.get()) * 60, std_duration.as_secs());
        }
    }
}
