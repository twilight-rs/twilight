mod change;
mod change_key;
mod entry;
mod event;
mod optional_entry_info;
mod partial_integration;

pub use self::{
    change::AuditLogChange, change_key::AuditLogChangeKey, entry::AuditLogEntry,
    event::AuditLogEvent, optional_entry_info::AuditLogOptionalEntryInfo,
    partial_integration::PartialGuildIntegration,
};

use crate::{channel::Webhook, user::User};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AuditLog {
    pub audit_log_entries: Vec<AuditLogEntry>,
    pub integrations: Vec<PartialGuildIntegration>,
    pub users: Vec<User>,
    pub webhooks: Vec<Webhook>,
}
