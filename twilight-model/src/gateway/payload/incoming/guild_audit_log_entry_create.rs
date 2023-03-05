use crate::guild::audit_log::AuditLogEntry;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

/// The inner value of the [`GuildAuditLogEntryCreate`] variant of the
/// [`Event`] enum.
///
/// It is received when a new audit log entry is created in a
/// server. The bot needs the [`VIEW_AUDIT_LOG`] permission to receive
/// the event.
///
/// [`GuildAuditLogEntryCreate`]: crate::gateway::event::Event::GuildAuditLogEntryCreate
/// [`Event`]:crate::gateway::event::Event
/// [`VIEW_AUDIT_LOG`]: crate::guild::Permissions::VIEW_AUDIT_LOG
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct GuildAuditLogEntryCreate(pub AuditLogEntry);

impl Deref for GuildAuditLogEntryCreate {
    type Target = AuditLogEntry;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GuildAuditLogEntryCreate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
