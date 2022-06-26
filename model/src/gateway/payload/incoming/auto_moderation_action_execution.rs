use crate::{
    guild::auto_moderation::{AutoModerationAction, AutoModerationTriggerType},
    id::{
        marker::{AutoModerationRuleMarker, ChannelMarker, GuildMarker, MessageMarker, UserMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};

/// Message was blocked by AutoMod according to a rule.
///
/// Sent to bot users with [`Permissions::MANAGE_GUILD`].
///
/// [`Permissions::MANAGE_GUILD`]: crate::guild::Permissions::MANAGE_GUILD
#[allow(clippy::doc_markdown)]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AutoModerationActionExecution {
    /// Action which was executed.
    pub action: AutoModerationAction,
    /// ID of any system auto moderation messages posted as a result of this
    /// action.
    ///
    /// Will not exist if this event does not correspond to an action with type
    /// [`SendAlertMessage`].
    ///
    /// [`SendAlertMessage`]: crate::guild::auto_moderation::AutoModerationActionType::SendAlertMessage
    pub alert_system_message_id: Option<Id<MessageMarker>>,
    /// ID of the channel in which user content was posted.
    pub channel_id: Option<Id<ChannelMarker>>,
    /// User generated text content.
    pub content: String,
    /// ID of the guild in which action was executed.
    pub guild_id: Id<GuildMarker>,
    /// Substring in content that triggered the rule.
    pub matched_content: Option<String>,
    /// Word or phrase configured in the rule that triggered the rule.
    pub matched_keyword: Option<String>,
    /// ID of any user message which content belongs to.
    ///
    /// Will not exist if message was blocked by AutoMod or content was not part
    /// of any message.
    pub message_id: Option<Id<MessageMarker>>,
    /// ID of the rule which action belongs to.
    pub rule_id: Id<AutoModerationRuleMarker>,
    /// Type of rule which was triggered.
    pub rule_trigger_type: AutoModerationTriggerType,
    /// ID of the user which generated the content which triggered the rule.
    pub user_id: Id<UserMarker>,
}
