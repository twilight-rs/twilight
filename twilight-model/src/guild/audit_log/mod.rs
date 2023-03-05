//! Audit Logs, created whenever an administrative action is performed within a
//! guild.
//!
//! For additional information refer to [Discord Docs/Audit Logs].
//!
//! [Discord Docs/Audit Logs]: https://discord.com/developers/docs/resources/audit-log

mod change;
mod change_key;
mod entry;
mod event_type;
mod integration;
mod optional_entry_info;

pub use self::{
    change::{AffectedRole, AuditLogChange, AuditLogChangeTypeValue},
    change_key::AuditLogChangeKey,
    entry::AuditLogEntry,
    event_type::AuditLogEventType,
    integration::AuditLogGuildIntegration,
    optional_entry_info::AuditLogOptionalEntryInfo,
};

use super::auto_moderation::AutoModerationRule;
use crate::{
    application::command::Command,
    channel::{Channel, Webhook},
    guild::scheduled_event::GuildScheduledEvent,
    user::User,
};
use serde::{Deserialize, Serialize};

/// Paginated audit log entries with additional information.
///
/// For additional information refer to [Discord Docs/Audit Logs][1].
///
/// [1]: https://discord.com/developers/docs/resources/audit-log#audit-log-object
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct AuditLog {
    /// List of referenced application commands.
    pub application_commands: Vec<Command>,
    /// List of referenced auto moderation rules.
    pub auto_moderation_rules: Vec<AutoModerationRule>,
    /// Paginated entries in a guild's audit log.
    #[serde(rename = "audit_log_entries")]
    pub entries: Vec<AuditLogEntry>,
    /// Information about mentioned scheduled events.
    pub guild_scheduled_events: Vec<GuildScheduledEvent>,
    /// Information about mentioned integrations.
    pub integrations: Vec<AuditLogGuildIntegration>,
    /// Information about mentioned threads.
    pub threads: Vec<Channel>,
    /// Information about mentioned users.
    ///
    /// For example, [users that performed the action][`AuditLogEntry::user_id`]
    /// to create an entry are in this list.
    pub users: Vec<User>,
    /// Information about mentioned webhooks.
    pub webhooks: Vec<Webhook>,
}

#[cfg(test)]
mod tests {
    use super::AuditLog;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;

    assert_fields!(
        AuditLog: entries,
        guild_scheduled_events,
        integrations,
        users,
        webhooks
    );
    assert_impl_all!(
        AuditLog: Clone,
        Debug,
        Deserialize<'static>,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    /// Test the (de)serialization of an audit log.
    ///
    /// We don't need to test with values since they're individually tested, so
    /// we just need to test that fields are present in deserialization and
    /// serialization as expected.
    #[test]
    fn serde() {
        let value = AuditLog {
            application_commands: Vec::new(),
            auto_moderation_rules: Vec::new(),
            entries: Vec::new(),
            guild_scheduled_events: Vec::new(),
            integrations: Vec::new(),
            threads: Vec::new(),
            users: Vec::new(),
            webhooks: Vec::new(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "AuditLog",
                    len: 8,
                },
                Token::Str("application_commands"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("auto_moderation_rules"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("audit_log_entries"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("guild_scheduled_events"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("integrations"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("threads"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("users"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("webhooks"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
            ],
        )
    }
}
