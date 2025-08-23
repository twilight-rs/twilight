use super::{AuditLogChange, AuditLogEventType, AuditLogOptionalEntryInfo};
use crate::id::{
    Id,
    marker::{AuditLogEntryMarker, GenericMarker, GuildMarker, UserMarker},
};
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
    /// ID of the server where the entry was added.
    ///
    /// This is **only** available when receiving the event in
    /// [`GuildAuditLogEntryCreate`].
    ///
    /// [`GuildAuditLogEntryCreate`]: crate::gateway::payload::incoming::GuildAuditLogEntryCreate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// ID of the entire entry.
    pub id: Id<AuditLogEntryMarker>,
    /// Optional information about the entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<AuditLogOptionalEntryInfo>,
    /// Optional application- or user-attached reason for the action that caused
    /// the entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// ID of the target entity.
    pub target_id: Option<Id<GenericMarker>>,
    /// ID of the [user] that performed the action.
    ///
    /// [user]: crate::user::User
    pub user_id: Option<Id<UserMarker>>,
}

#[cfg(test)]
mod tests {
    use super::{
        super::{AuditLogChange, AuditLogEventType},
        AuditLogEntry,
    };
    use crate::{id::Id, test::image_hash};
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
    fn serde() {
        let value = AuditLogEntry {
            action_type: AuditLogEventType::GuildUpdate,
            changes: Vec::from([AuditLogChange::IconHash {
                new: None,
                old: Some(image_hash::ICON),
            }]),
            guild_id: None,
            id: Id::new(3),
            options: None,
            reason: Some("some reason".to_owned()),
            target_id: Some(Id::new(2)),
            user_id: Some(Id::new(1)),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "AuditLogEntry",
                    len: 6,
                },
                Token::Str("action_type"),
                Token::U16(AuditLogEventType::GuildUpdate.into()),
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
                Token::Str(image_hash::ICON_INPUT),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("reason"),
                Token::Some,
                Token::Str("some reason"),
                Token::Str("target_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("user_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::StructEnd,
            ],
        );
    }
}
