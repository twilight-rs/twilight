use serde::{Deserialize, Serialize};

use crate::{channel::AutoArchiveDuration, id::UserId};

/// The thread metadata object contains a number of thread-specific channel fields
/// that are not needed by other channel types.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ThreadMetadata {
    pub archived: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archiver_id: Option<UserId>,
    pub auto_archive_duration: AutoArchiveDuration,
    pub archive_timestamp: String,
    #[serde(default)]
    pub locked: bool,
}

#[cfg(test)]
mod tests {
    use super::ThreadMetadata;
    use crate::{channel::AutoArchiveDuration, id::UserId};
    use serde_test::Token;

    #[test]
    fn test_thread_metadata() {
        let value = ThreadMetadata {
            archived: true,
            archiver_id: Some(UserId(1)),
            auto_archive_duration: AutoArchiveDuration::Day,
            archive_timestamp: "123".to_owned(),
            locked: false,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "ThreadMetadata",
                    len: 5,
                },
                Token::Str("archived"),
                Token::Bool(true),
                Token::Str("archiver_id"),
                Token::Some,
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("1"),
                Token::Str("auto_archive_duration"),
                Token::U16(1440),
                Token::Str("archive_timestamp"),
                Token::Str("123"),
                Token::Str("locked"),
                Token::Bool(false),
                Token::StructEnd,
            ],
        );
    }
}
