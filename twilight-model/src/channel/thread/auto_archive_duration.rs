use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AutoArchiveDuration(u16);

impl AutoArchiveDuration {
    pub const HOUR: Self = Self::new(60);

    pub const DAY: Self = Self::new(Self::HOUR.get() * 24);

    pub const THREE_DAYS: Self = Self::new(Self::DAY.get() * 3);

    pub const WEEK: Self = Self::new(Self::DAY.get() * 7);

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

impl From<AutoArchiveDuration> for Duration {
    fn from(value: AutoArchiveDuration) -> Self {
        Self::from_secs(u64::from(value.get()) * 60)
    }
}

impl_typed!(AutoArchiveDuration, u16);

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
