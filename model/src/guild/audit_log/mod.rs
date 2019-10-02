mod change;
mod change_key;
mod entry;
mod event;
mod optional_entry_info;

pub use self::{
    change::AuditLogChange,
    change_key::AuditLogChangeKey,
    entry::AuditLogEntry,
    event::AuditLogEvent,
    optional_entry_info::AuditLogOptionalEntryInfo,
};

use crate::{channel::Webhook, user::User};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug)]
pub struct AuditLog {
    audit_log_entries: Vec<AuditLogEntry>,
    users: Vec<User>,
    webhooks: Vec<Webhook>,
}
