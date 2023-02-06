use crate::id::{
    marker::{ChannelMarker, GenericMarker, MessageMarker},
    Id,
};
use serde::{Deserialize, Serialize};

/// Additional information for certain [`AuditLogEventType`]s.
///
/// [`AuditLogEventType`]: super::AuditLogEventType
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AuditLogOptionalEntryInfo {
    /// Name of the Auto Moderation rule that was triggered.
    ///
    /// The following events have this option:
    ///
    /// - [`AuditLogEventType::AUTO_MODERATION_BLOCK_MESSAGE`]
    /// - [`AuditLogEventType::AUTO_MODERATION_FLAG_TO_CHANNEL`]
    /// - [`AuditLogEventType::AUTO_MODERATION_USER_COMMUNICATION_DISABLED`]
    ///
    /// [`AuditLogEventType::AUTO_MODERATION_BLOCK_MESSAGE`]: super::AuditLogEventType::AUTO_MODERATION_BLOCK_MESSAGE
    /// [`AuditLogEventType::AUTO_MODERATION_FLAG_TO_CHANNEL`]: super::AuditLogEventType::AUTO_MODERATION_FLAG_TO_CHANNEL
    /// [`AuditLogEventType::AUTO_MODERATION_USER_COMMUNICATION_DISABLED`]: super::AuditLogEventType::AUTO_MODERATION_USER_COMMUNICATION_DISABLED
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_moderation_rule_name: Option<String>,
    /// Trigger type of the Auto Moderation rule that was triggered.
    ///
    /// The following events have this option:
    ///
    /// - [`AuditLogEventType::AUTO_MODERATION_BLOCK_MESSAGE`]
    /// - [`AuditLogEventType::AUTO_MODERATION_FLAG_TO_CHANNEL`]
    /// - [`AuditLogEventType::AUTO_MODERATION_USER_COMMUNICATION_DISABLED`]
    ///
    /// [`AuditLogEventType::AUTO_MODERATION_BLOCK_MESSAGE`]: super::AuditLogEventType::AUTO_MODERATION_BLOCK_MESSAGE
    /// [`AuditLogEventType::AUTO_MODERATION_FLAG_TO_CHANNEL`]: super::AuditLogEventType::AUTO_MODERATION_FLAG_TO_CHANNEL
    /// [`AuditLogEventType::AUTO_MODERATION_USER_COMMUNICATION_DISABLED`]: super::AuditLogEventType::AUTO_MODERATION_USER_COMMUNICATION_DISABLED
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_moderation_rule_trigger_type: Option<String>,
    /// Channel in which the entities were targeted.
    ///
    /// The following events have this option:
    ///
    /// - [`AuditLogEventType::AUTO_MODERATION_BLOCK_MESSAGE`]
    /// - [`AuditLogEventType::AUTO_MODERATION_FLAG_TO_CHANNEL`]
    /// - [`AuditLogEventType::AUTO_MODERATION_USER_COMMUNICATION_DISABLED`]
    /// - [`AuditLogEventType::MEMBER_MOVE`]
    /// - [`AuditLogEventType::MESSAGE_DELETE`]
    /// - [`AuditLogEventType::MESSAGE_PIN`]
    /// - [`AuditLogEventType::MESSAGE_UNPIN`]
    /// - [`AuditLogEventType::STAGE_INSTANCE_CREATE`]
    /// - [`AuditLogEventType::STAGE_INSTANCE_DELETE`]
    /// - [`AuditLogEventType::STAGE_INSTANCE_UPDATE`]
    ///
    /// [`AuditLogEventType::AUTO_MODERATION_BLOCK_MESSAGE`]: super::AuditLogEventType::AUTO_MODERATION_BLOCK_MESSAGE
    /// [`AuditLogEventType::AUTO_MODERATION_FLAG_TO_CHANNEL`]: super::AuditLogEventType::AUTO_MODERATION_FLAG_TO_CHANNEL
    /// [`AuditLogEventType::AUTO_MODERATION_USER_COMMUNICATION_DISABLED`]: super::AuditLogEventType::AUTO_MODERATION_USER_COMMUNICATION_DISABLED
    /// [`AuditLogEventType::MEMBER_MOVE`]: super::AuditLogEventType::MEMBER_MOVE
    /// [`AuditLogEventType::MESSAGE_DELETE`]: super::AuditLogEventType::MESSAGE_DELETE
    /// [`AuditLogEventType::MESSAGE_PIN`]: super::AuditLogEventType::MESSAGE_PIN
    /// [`AuditLogEventType::MESSAGE_UNPIN`]: super::AuditLogEventType::MESSAGE_UNPIN
    /// [`AuditLogEventType::STAGE_INSTANCE_CREATE`]: super::AuditLogEventType::STAGE_INSTANCE_CREATE
    /// [`AuditLogEventType::STAGE_INSTANCE_DELETE`]: super::AuditLogEventType::STAGE_INSTANCE_DELETE
    /// [`AuditLogEventType::STAGE_INSTANCE_UPDATE`]: super::AuditLogEventType::STAGE_INSTANCE_UPDATE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Id<ChannelMarker>>,
    /// Number of entities that were targeted.
    ///
    /// The following events have this option:
    ///
    /// - [`AuditLogEventType::MEMBER_DISCONNECT`]
    /// - [`AuditLogEventType::MEMBER_MOVE`]
    /// - [`AuditLogEventType::MESSAGE_BULK_DELETE`]
    /// - [`AuditLogEventType::MESSAGE_DELETE`]
    ///
    /// [`AuditLogEventType::MEMBER_DISCONNECT`]: super::AuditLogEventType::MEMBER_DISCONNECT
    /// [`AuditLogEventType::MEMBER_MOVE`]: super::AuditLogEventType::MEMBER_MOVE
    /// [`AuditLogEventType::MESSAGE_BULK_DELETE`]: super::AuditLogEventType::MESSAGE_BULK_DELETE
    /// [`AuditLogEventType::MESSAGE_DELETE`]: super::AuditLogEventType::MESSAGE_DELETE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
    /// Specified number of days' worth of inactivity members must have in order
    /// to be kicked.
    ///
    /// The following events have this option:
    ///
    /// - [`AuditLogEventType::MEMBER_PRUNE`]
    ///
    /// [`AuditLogEventType::MEMBER_PRUNE`]: super::AuditLogEventType::MEMBER_PRUNE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_member_days: Option<String>,
    /// ID of overwritten entity.
    ///
    /// The following events have this option:
    ///
    /// - [`AuditLogEventType::CHANNEL_OVERWRITE_CREATE`]
    /// - [`AuditLogEventType::CHANNEL_OVERWRITE_DELETE`]
    /// - [`AuditLogEventType::CHANNEL_OVERWRITE_UPDATE`]
    ///
    /// [`AuditLogEventType::CHANNEL_OVERWRITE_CREATE`]: super::AuditLogEventType::CHANNEL_OVERWRITE_CREATE
    /// [`AuditLogEventType::CHANNEL_OVERWRITE_DELETE`]: super::AuditLogEventType::CHANNEL_OVERWRITE_DELETE
    /// [`AuditLogEventType::CHANNEL_OVERWRITE_UPDATE`]: super::AuditLogEventType::CHANNEL_OVERWRITE_UPDATE
    pub id: Option<Id<GenericMarker>>,
    /// Type of overwritten entity.
    ///
    /// The following events have this option:
    ///
    /// - [`AuditLogEventType::CHANNEL_OVERWRITE_CREATE`]
    /// - [`AuditLogEventType::CHANNEL_OVERWRITE_DELETE`]
    /// - [`AuditLogEventType::CHANNEL_OVERWRITE_UPDATE`]
    ///
    /// [`AuditLogEventType::CHANNEL_OVERWRITE_CREATE`]: super::AuditLogEventType::CHANNEL_OVERWRITE_CREATE
    /// [`AuditLogEventType::CHANNEL_OVERWRITE_DELETE`]: super::AuditLogEventType::CHANNEL_OVERWRITE_DELETE
    /// [`AuditLogEventType::CHANNEL_OVERWRITE_UPDATE`]: super::AuditLogEventType::CHANNEL_OVERWRITE_UPDATE
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Number of members removed from a change.
    ///
    /// The following events have this option:
    ///
    /// - [`AuditLogEventType::MEMBER_PRUNE`]
    ///
    /// [`AuditLogEventType::MEMBER_PRUNE`]: super::AuditLogEventType::MEMBER_PRUNE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_removed: Option<String>,
    /// ID of the affected message.
    ///
    /// The following events have this option:
    ///
    /// - [`AuditLogEventType::MESSAGE_PIN`]
    /// - [`AuditLogEventType::MESSAGE_UNPIN`]
    ///
    /// [`AuditLogEventType::MESSAGE_PIN`]: super::AuditLogEventType::MESSAGE_PIN
    /// [`AuditLogEventType::MESSAGE_UNPIN`]: super::AuditLogEventType::MESSAGE_UNPIN
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Id<MessageMarker>>,
    /// Name of a role.
    ///
    /// The following events have this option:
    ///
    /// - [`AuditLogEventType::CHANNEL_OVERWRITE_CREATE`]
    /// - [`AuditLogEventType::CHANNEL_OVERWRITE_DELETE`]
    /// - [`AuditLogEventType::CHANNEL_OVERWRITE_UPDATE`]
    ///
    /// [`AuditLogEventType::CHANNEL_OVERWRITE_CREATE`]: super::AuditLogEventType::CHANNEL_OVERWRITE_CREATE
    /// [`AuditLogEventType::CHANNEL_OVERWRITE_DELETE`]: super::AuditLogEventType::CHANNEL_OVERWRITE_DELETE
    /// [`AuditLogEventType::CHANNEL_OVERWRITE_UPDATE`]: super::AuditLogEventType::CHANNEL_OVERWRITE_UPDATE
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::AuditLogOptionalEntryInfo;
    use serde::{Deserialize, Serialize};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        AuditLogOptionalEntryInfo: channel_id,
        count,
        delete_member_days,
        id,
        kind,
        members_removed,
        message_id,
        role_name
    );
    assert_impl_all!(
        AuditLogOptionalEntryInfo: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );
}
