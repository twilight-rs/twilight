use crate::{
    guild::auto_moderation::{AutoModerationAction, AutoModerationTriggerType},
    id::{
        marker::{AutoModerationRuleMarker, ChannelMarker, GuildMarker, MessageMarker, UserMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};

/// Message has been blocked by AutoMod according to a rule.
///
/// Requires [`Permissions::MANAGE_GUILD`].
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_system_message_id: Option<Id<MessageMarker>>,
    /// ID of the channel in which user content was posted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<Id<ChannelMarker>>,
    /// User generated text content.
    ///
    /// Requires [`Intents::MESSAGE_CONTENT`].
    ///
    /// [`Intents::MESSAGE_CONTENT`]: crate::gateway::Intents::MESSAGE_CONTENT
    pub content: String,
    /// ID of the guild in which action was executed.
    pub guild_id: Id<GuildMarker>,
    /// Substring in content that triggered the rule.
    ///
    /// Requires [`Intents::MESSAGE_CONTENT`].
    ///
    /// [`Intents::MESSAGE_CONTENT`]: crate::gateway::Intents::MESSAGE_CONTENT
    pub matched_content: Option<String>,
    /// Word or phrase configured in the rule that triggered the rule.
    pub matched_keyword: Option<String>,
    /// ID of any user message which content belongs to.
    ///
    /// Will not exist if message was blocked by AutoMod or content was not part
    /// of any message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<Id<MessageMarker>>,
    /// ID of the rule which action belongs to.
    pub rule_id: Id<AutoModerationRuleMarker>,
    /// Type of rule which was triggered.
    pub rule_trigger_type: AutoModerationTriggerType,
    /// ID of the user which generated the content which triggered the rule.
    pub user_id: Id<UserMarker>,
}

#[cfg(test)]
mod tests {
    use super::AutoModerationActionExecution;
    use crate::{
        guild::auto_moderation::{
            AutoModerationAction, AutoModerationActionType, AutoModerationTriggerType,
        },
        id::{
            marker::{
                AutoModerationRuleMarker, ChannelMarker, GuildMarker, MessageMarker, UserMarker,
            },
            Id,
        },
    };
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        AutoModerationActionExecution: action,
        alert_system_message_id,
        channel_id,
        content,
        guild_id,
        matched_content,
        matched_keyword,
        message_id,
        rule_id,
        rule_trigger_type,
        user_id
    );
    assert_impl_all!(
        AutoModerationActionExecution: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize,
        Send,
        Sync
    );

    #[test]
    fn action_execution() {
        const ALERT_SYSTEM_MESSAGE_ID: Id<MessageMarker> = Id::new(1);
        const CHANNEL_ID: Id<ChannelMarker> = Id::new(2);
        const GUILD_ID: Id<GuildMarker> = Id::new(3);
        const MESSAGE_ID: Id<MessageMarker> = Id::new(4);
        const RULE_ID: Id<AutoModerationRuleMarker> = Id::new(5);
        const USER_ID: Id<UserMarker> = Id::new(6);

        let value = AutoModerationActionExecution {
            action: AutoModerationAction {
                kind: AutoModerationActionType::BlockMessage,
                metadata: None,
            },
            alert_system_message_id: Some(ALERT_SYSTEM_MESSAGE_ID),
            channel_id: Some(CHANNEL_ID),
            content: "darn".into(),
            guild_id: GUILD_ID,
            matched_content: Some("darn".into()),
            matched_keyword: Some("darn".into()),
            message_id: Some(MESSAGE_ID),
            rule_id: RULE_ID,
            rule_trigger_type: AutoModerationTriggerType::Keyword,
            user_id: USER_ID,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "AutoModerationActionExecution",
                    len: 11,
                },
                Token::Str("action"),
                Token::Struct {
                    name: "AutoModerationAction",
                    len: 1,
                },
                Token::Str("type"),
                Token::U8(u8::from(AutoModerationActionType::BlockMessage)),
                Token::StructEnd,
                Token::Str("alert_system_message_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("content"),
                Token::Str("darn"),
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("matched_content"),
                Token::Some,
                Token::Str("darn"),
                Token::Str("matched_keyword"),
                Token::Some,
                Token::Str("darn"),
                Token::Str("message_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::Str("rule_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("5"),
                Token::Str("rule_trigger_type"),
                Token::U8(u8::from(AutoModerationTriggerType::Keyword)),
                Token::Str("user_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("6"),
                Token::StructEnd,
            ],
        );
    }
}
