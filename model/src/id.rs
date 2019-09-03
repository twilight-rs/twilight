use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct ApplicationId(pub String);

impl Display for ApplicationId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct AttachmentId(pub String);

impl Display for AttachmentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct AuditLogEntryId(pub String);

impl Display for AuditLogEntryId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct ChannelId(pub String);

impl Display for ChannelId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct EmojiId(pub String);

impl Display for EmojiId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct GenericId(pub String);

impl Display for GenericId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct GuildId(pub String);

impl Display for GuildId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct IntegrationId(pub String);

impl Display for IntegrationId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct MessageId(pub String);

impl Display for MessageId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct RoleId(pub String);

impl Display for RoleId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct UserId(pub String);

impl Display for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Hash, Serialize)]
pub struct WebhookId(pub String);

impl Display for WebhookId {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}
