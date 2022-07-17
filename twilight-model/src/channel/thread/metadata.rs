use super::AutoArchiveDuration;
use crate::util::Timestamp;
use serde::{Deserialize, Serialize};

/// The thread metadata object contains a number of thread-specific channel fields
/// that are not needed by other channel types.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadMetadata {
    pub archived: bool,
    /// Duration without messages before the thread automatically archives.
    ///
    /// Automatic archive durations are not locked behind the guild's boost
    /// level.
    pub auto_archive_duration: AutoArchiveDuration,
    pub archive_timestamp: Timestamp,
    /// When the thread was created at.
    ///
    /// Only present if the Thread has been created after 2022-01-09.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<Timestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,
    #[serde(default)]
    pub locked: bool,
}

#[cfg(test)]
mod tests {
    use super::{AutoArchiveDuration, ThreadMetadata};
    use crate::util::datetime::{Timestamp, TimestampParseError};
    use serde_test::Token;
    use std::str::FromStr;

    #[test]
    fn thread_metadata() -> Result<(), TimestampParseError> {
        const DATETIME: &str = "2021-09-19T14:17:32.000000+00:00";

        let timestamp = Timestamp::from_str(DATETIME)?;

        let value = ThreadMetadata {
            archived: true,
            auto_archive_duration: AutoArchiveDuration::Day,
            archive_timestamp: timestamp,
            create_timestamp: Some(timestamp),
            invitable: Some(false),
            locked: false,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ThreadMetadata",
                    len: 6,
                },
                Token::Str("archived"),
                Token::Bool(true),
                Token::Str("auto_archive_duration"),
                Token::U16(1440),
                Token::Str("archive_timestamp"),
                Token::Str(DATETIME),
                Token::Str("create_timestamp"),
                Token::Some,
                Token::Str(DATETIME),
                Token::Str("invitable"),
                Token::Some,
                Token::Bool(false),
                Token::Str("locked"),
                Token::Bool(false),
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
