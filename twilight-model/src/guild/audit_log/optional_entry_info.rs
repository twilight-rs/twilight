use crate::{
    guild::GuildIntegrationType,
    id::{
        Id,
        marker::{ChannelMarker, GenericMarker, MessageMarker},
    },
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
    /// - [`AuditLogEventType::AutoModerationBlockMessage`]
    /// - [`AuditLogEventType::AutoModerationFlagToChannel`]
    /// - [`AuditLogEventType::AutoModerationUserCommunicationDisabled`]
    ///
    /// [`AuditLogEventType::AutoModerationBlockMessage`]: super::AuditLogEventType::AutoModerationBlockMessage
    /// [`AuditLogEventType::AutoModerationFlagToChannel`]: super::AuditLogEventType::AutoModerationFlagToChannel
    /// [`AuditLogEventType::AutoModerationUserCommunicationDisabled`]: super::AuditLogEventType::AutoModerationUserCommunicationDisabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_moderation_rule_name: Option<String>,
    /// Trigger type of the Auto Moderation rule that was triggered.
    ///
    /// The following events have this option:
    ///
    /// - [`AuditLogEventType::AutoModerationBlockMessage`]
    /// - [`AuditLogEventType::AutoModerationFlagToChannel`]
    /// - [`AuditLogEventType::AutoModerationUserCommunicationDisabled`]
    ///
    /// [`AuditLogEventType::AutoModerationBlockMessage`]: super::AuditLogEventType::AutoModerationBlockMessage
    /// [`AuditLogEventType::AutoModerationFlagToChannel`]: super::AuditLogEventType::AutoModerationFlagToChannel
    /// [`AuditLogEventType::AutoModerationUserCommunicationDisabled`]: super::AuditLogEventType::AutoModerationUserCommunicationDisabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_moderation_rule_trigger_type: Option<String>,
    /// Channel in which the entities were targeted.
    ///
    /// The following events have this option:
    ///
    /// - [`AuditLogEventType::AutoModerationBlockMessage`]
    /// - [`AuditLogEventType::AutoModerationFlagToChannel`]
    /// - [`AuditLogEventType::AutoModerationUserCommunicationDisabled`]
    /// - [`AuditLogEventType::MemberMove`]
    /// - [`AuditLogEventType::MessageDelete`]
    /// - [`AuditLogEventType::MessagePin`]
    /// - [`AuditLogEventType::MessageUnpin`]
    /// - [`AuditLogEventType::StageInstanceCreate`]
    /// - [`AuditLogEventType::StageInstanceDelete`]
    /// - [`AuditLogEventType::StageInstanceUpdate`]
    ///
    /// [`AuditLogEventType::AutoModerationBlockMessage`]: super::AuditLogEventType::AutoModerationBlockMessage
    /// [`AuditLogEventType::AutoModerationFlagToChannel`]: super::AuditLogEventType::AutoModerationFlagToChannel
    /// [`AuditLogEventType::AutoModerationUserCommunicationDisabled`]: super::AuditLogEventType::AutoModerationUserCommunicationDisabled
    /// [`AuditLogEventType::MemberMove`]: super::AuditLogEventType::MemberMove
    /// [`AuditLogEventType::MessageDelete`]: super::AuditLogEventType::MessageDelete
    /// [`AuditLogEventType::MessagePin`]: super::AuditLogEventType::MessagePin
    /// [`AuditLogEventType::MessageUnpin`]: super::AuditLogEventType::MessageUnpin
    /// [`AuditLogEventType::StageInstanceCreate`]: super::AuditLogEventType::StageInstanceCreate
    /// [`AuditLogEventType::StageInstanceDelete`]: super::AuditLogEventType::StageInstanceDelete
    /// [`AuditLogEventType::StageInstanceUpdate`]: super::AuditLogEventType::StageInstanceUpdate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Id<ChannelMarker>>,
    /// Number of entities that were targeted.
    ///
    /// The following events have this option:
    ///
    /// - [`AuditLogEventType::MemberDisconnect`]
    /// - [`AuditLogEventType::MemberMove`]
    /// - [`AuditLogEventType::MessageBulkDelete`]
    /// - [`AuditLogEventType::MessageDelete`]
    ///
    /// [`AuditLogEventType::MemberDisconnect`]: super::AuditLogEventType::MemberDisconnect
    /// [`AuditLogEventType::MemberMove`]: super::AuditLogEventType::MemberMove
    /// [`AuditLogEventType::MessageBulkDelete`]: super::AuditLogEventType::MessageBulkDelete
    /// [`AuditLogEventType::MessageDelete`]: super::AuditLogEventType::MessageDelete
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<String>,
    /// Specified number of days' worth of inactivity members must have in order
    /// to be kicked.
    ///
    /// The following events have this option:
    ///
    /// - [`AuditLogEventType::MemberPrune`]
    ///
    /// [`AuditLogEventType::MemberPrune`]: super::AuditLogEventType::MemberPrune
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_member_days: Option<String>,
    /// ID of overwritten entity.
    ///
    /// The following events have this option:
    ///
    /// - [`AuditLogEventType::ChannelOverwriteCreate`]
    /// - [`AuditLogEventType::ChannelOverwriteDelete`]
    /// - [`AuditLogEventType::ChannelOverwriteUpdate`]
    ///
    /// [`AuditLogEventType::ChannelOverwriteCreate`]: super::AuditLogEventType::ChannelOverwriteCreate
    /// [`AuditLogEventType::ChannelOverwriteDelete`]: super::AuditLogEventType::ChannelOverwriteDelete
    /// [`AuditLogEventType::ChannelOverwriteUpdate`]: super::AuditLogEventType::ChannelOverwriteUpdate
    pub id: Option<Id<GenericMarker>>,
    /// Type of integration which performed the action.
    ///
    /// The following events have this option:
    ///
    /// - [`AuditLogEventType::MemberKick`]
    /// - [`AuditLogEventType::MemberRoleUpdate`]
    ///
    /// [`AuditLogEventType::MemberKick`]: super::AuditLogEventType::MemberKick
    /// [`AuditLogEventType::MemberRoleUpdate`]: super::AuditLogEventType::MemberRoleUpdate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<GuildIntegrationType>,
    /// Type of overwritten entity.
    ///
    /// The following events have this option:
    ///
    /// - [`AuditLogEventType::ChannelOverwriteCreate`]
    /// - [`AuditLogEventType::ChannelOverwriteDelete`]
    /// - [`AuditLogEventType::ChannelOverwriteUpdate`]
    ///
    /// [`AuditLogEventType::ChannelOverwriteCreate`]: super::AuditLogEventType::ChannelOverwriteCreate
    /// [`AuditLogEventType::ChannelOverwriteDelete`]: super::AuditLogEventType::ChannelOverwriteDelete
    /// [`AuditLogEventType::ChannelOverwriteUpdate`]: super::AuditLogEventType::ChannelOverwriteUpdate
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Number of members removed from a change.
    ///
    /// The following events have this option:
    ///
    /// - [`AuditLogEventType::MemberPrune`]
    ///
    /// [`AuditLogEventType::MemberPrune`]: super::AuditLogEventType::MemberPrune
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members_removed: Option<String>,
    /// ID of the affected message.
    ///
    /// The following events have this option:
    ///
    /// - [`AuditLogEventType::MessagePin`]
    /// - [`AuditLogEventType::MessageUnpin`]
    ///
    /// [`AuditLogEventType::MessagePin`]: super::AuditLogEventType::MessagePin
    /// [`AuditLogEventType::MessageUnpin`]: super::AuditLogEventType::MessageUnpin
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Id<MessageMarker>>,
    /// Name of a role.
    ///
    /// The following events have this option:
    ///
    /// - [`AuditLogEventType::ChannelOverwriteCreate`]
    /// - [`AuditLogEventType::ChannelOverwriteDelete`]
    /// - [`AuditLogEventType::ChannelOverwriteUpdate`]
    ///
    /// [`AuditLogEventType::ChannelOverwriteCreate`]: super::AuditLogEventType::ChannelOverwriteCreate
    /// [`AuditLogEventType::ChannelOverwriteDelete`]: super::AuditLogEventType::ChannelOverwriteDelete
    /// [`AuditLogEventType::ChannelOverwriteUpdate`]: super::AuditLogEventType::ChannelOverwriteUpdate
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
