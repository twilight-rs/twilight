use super::{AuditLogChange, AuditLogEventType, AuditLogOptionalEntryInfo};
use crate::id::{AuditLogEntryId, GenericId, UserId};
use serde::{Deserialize, Serialize};

/// Entry in an [`AuditLog`] possibly containing a number of detailed changes.
///
/// [`AuditLog`]: super::AuditLog
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AuditLogEntry {
    /// Type of event to cause the entry.
    pub action_type: AuditLogEventType,
    /// List of changes included in the entry.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub changes: Vec<AuditLogChange>,
    /// ID of the entire entry.
    pub id: AuditLogEntryId,
    /// Optional information about the entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<AuditLogOptionalEntryInfo>,
    /// Optional application- or user-attached reason for the action that caused
    /// the entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// ID of the target entity.
    pub target_id: Option<GenericId>,
    /// ID of the [user] that performed the action.
    ///
    /// [user]: crate::user::User
    pub user_id: Option<UserId>,
}

#[cfg(test)]
mod tests {
    use super::{
        super::{AuditLogChange, AuditLogEventType},
        AuditLogEntry,
    };
    use crate::id::{AuditLogEntryId, GenericId, UserId};
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        AuditLogEntry: action_type,
        changes,
        id,
        options,
        reason,
        target_id,
        user_id
    );
    assert_impl_all!(
        AuditLogEntry: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    /// Test the deserialization and serialization of an audit log entry.
    #[test]
    fn test_serde() {
        const OLD: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

        let value = AuditLogEntry {
            action_type: AuditLogEventType::GuildUpdate,
            changes: Vec::from([AuditLogChange::IconHash {
                new: None,
                old: Some(OLD.to_owned()),
            }]),
            id: AuditLogEntryId::new(3).expect("non zero"),
            options: None,
            reason: Some("some reason".to_owned()),
            target_id: Some(GenericId::new(2).expect("non zero")),
            user_id: Some(UserId::new(1).expect("non zero")),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "AuditLogEntry",
                    len: 6,
                },
                Token::Str("action_type"),
                Token::U8(AuditLogEventType::GuildUpdate as u8),
                Token::Str("changes"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "AuditLogChange",
                    len: 2,
                },
                Token::Str("key"),
                Token::Str("icon_hash"),
                Token::Str("old_value"),
                Token::Some,
                Token::Str(OLD),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("id"),
                Token::NewtypeStruct {
                    name: "AuditLogEntryId",
                },
                Token::Str("3"),
                Token::Str("reason"),
                Token::Some,
                Token::Str("some reason"),
                Token::Str("target_id"),
                Token::Some,
                Token::NewtypeStruct { name: "GenericId" },
                Token::Str("2"),
                Token::Str("user_id"),
                Token::Some,
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("1"),
                Token::StructEnd,
            ],
        );
    }
}
