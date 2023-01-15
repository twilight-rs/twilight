use crate::guild::audit_log::AuditLogEntry;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
