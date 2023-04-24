//! Types for Auto Moderation.
//!
//! Auto Moderation is a feature which allows each guild to set up rules that
//! trigger based on some criteria. For example, a rule can trigger whenever a
//! message contains a specific keyword.
//!
//! Rules can be configured to automatically execute actions whenever they
//! trigger. For example, if a user tries to send a message which contains a
//! certain keyword, a rule can trigger and block the message before it is sent.

#![warn(missing_docs)]

mod action;
mod event_type;
mod preset_type;
mod trigger_metadata;
mod trigger_type;

pub use self::{
    action::{AutoModerationAction, AutoModerationActionMetadata, AutoModerationActionType},
    event_type::AutoModerationEventType,
    preset_type::AutoModerationKeywordPresetType,
    trigger_metadata::AutoModerationTriggerMetadata,
    trigger_type::AutoModerationTriggerType,
};

use crate::id::{
    marker::{AutoModerationRuleMarker, ChannelMarker, GuildMarker, RoleMarker, UserMarker},
    Id,
};
use serde::{Deserialize, Serialize};

/// Configured auto moderation rule.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AutoModerationRule {
    /// Actions which will execute when the rule is triggered.
    pub actions: Vec<AutoModerationAction>,
    /// User which created the rule.
    pub creator_id: Id<UserMarker>,
    /// Whether the rule is enabled.
    pub enabled: bool,
    /// Rule event type.
    pub event_type: AutoModerationEventType,
    /// Channels that should not be affected by the rule.
    ///
    /// Maximum of 50.
    pub exempt_channels: Vec<Id<ChannelMarker>>,
    /// Roles that should not be affected by the rule.
    ///
    /// Maximum of 20.
    pub exempt_roles: Vec<Id<RoleMarker>>,
    /// ID of the guild the rule belongs to.
    pub guild_id: Id<GuildMarker>,
    /// ID of the rule.
    pub id: Id<AutoModerationRuleMarker>,
    /// Name of the rule.
    pub name: String,
    /// Rule trigger metadata.
    pub trigger_metadata: AutoModerationTriggerMetadata,
    /// Rule trigger type.
    pub trigger_type: AutoModerationTriggerType,
}

#[cfg(test)]
mod tests {
    use super::{
        AutoModerationAction, AutoModerationActionMetadata, AutoModerationActionType,
        AutoModerationEventType, AutoModerationRule, AutoModerationTriggerMetadata,
        AutoModerationTriggerType,
    };
    use crate::id::{
        marker::{AutoModerationRuleMarker, ChannelMarker, GuildMarker, RoleMarker, UserMarker},
        Id,
    };
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        AutoModerationRule: actions,
        creator_id,
        enabled,
        event_type,
        exempt_channels,
        exempt_roles,
        guild_id,
        id,
        name,
        trigger_metadata,
        trigger_type
    );
    assert_impl_all!(
        AutoModerationRule: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize,
        Send,
        Sync,
    );

    #[allow(clippy::too_many_lines)]
    #[test]
    fn rule() {
        const ACTION_CHANNEL_ID: Id<ChannelMarker> = Id::new(1);
        const AUTO_MODERATION_RULE_ID: Id<AutoModerationRuleMarker> = Id::new(2);
        const CREATOR_ID: Id<UserMarker> = Id::new(3);
        const EXEMPT_CHANNEL_ID: Id<ChannelMarker> = Id::new(4);
        const EXEMPT_ROLE_ID: Id<RoleMarker> = Id::new(5);
        const GUILD_ID: Id<GuildMarker> = Id::new(6);

        let value = AutoModerationRule {
            actions: Vec::from([
                AutoModerationAction {
                    kind: AutoModerationActionType::BlockMessage,
                    metadata: None,
                },
                AutoModerationAction {
                    kind: AutoModerationActionType::SendAlertMessage,
                    metadata: Some(AutoModerationActionMetadata {
                        channel_id: Some(ACTION_CHANNEL_ID),
                        custom_message: None,
                        duration_seconds: None,
                    }),
                },
                AutoModerationAction {
                    kind: AutoModerationActionType::Timeout,
                    metadata: Some(AutoModerationActionMetadata {
                        channel_id: None,
                        custom_message: None,
                        duration_seconds: Some(120),
                    }),
                },
            ]),
            creator_id: CREATOR_ID,
            enabled: true,
            event_type: AutoModerationEventType::MessageSend,
            exempt_channels: Vec::from([EXEMPT_CHANNEL_ID]),
            exempt_roles: Vec::from([EXEMPT_ROLE_ID]),
            guild_id: GUILD_ID,
            id: AUTO_MODERATION_RULE_ID,
            name: "rule".into(),
            trigger_metadata: AutoModerationTriggerMetadata {
                allow_list: None,
                keyword_filter: Some(Vec::from(["shoot".into(), "darn".into()])),
                presets: None,
                regex_patterns: Some(Vec::from(["[a-z]+".into(), "[0-9]+".into()])),
                mention_total_limit: None,
            },
            trigger_type: AutoModerationTriggerType::Keyword,
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "AutoModerationRule",
                    len: 11,
                },
                Token::Str("actions"),
                Token::Seq { len: Some(3) },
                Token::Struct {
                    name: "AutoModerationAction",
                    len: 1,
                },
                Token::Str("type"),
                Token::U8(u8::from(AutoModerationActionType::BlockMessage)),
                Token::StructEnd,
                Token::Struct {
                    name: "AutoModerationAction",
                    len: 2,
                },
                Token::Str("type"),
                Token::U8(u8::from(AutoModerationActionType::SendAlertMessage)),
                Token::Str("metadata"),
                Token::Some,
                Token::Struct {
                    name: "AutoModerationActionMetadata",
                    len: 1,
                },
                Token::Str("channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::StructEnd,
                Token::StructEnd,
                Token::Struct {
                    name: "AutoModerationAction",
                    len: 2,
                },
                Token::Str("type"),
                Token::U8(u8::from(AutoModerationActionType::Timeout)),
                Token::Str("metadata"),
                Token::Some,
                Token::Struct {
                    name: "AutoModerationActionMetadata",
                    len: 1,
                },
                Token::Str("duration_seconds"),
                Token::Some,
                Token::U32(120),
                Token::StructEnd,
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("creator_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("enabled"),
                Token::Bool(true),
                Token::Str("event_type"),
                Token::U8(u8::from(AutoModerationEventType::MessageSend)),
                Token::Str("exempt_channels"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("4"),
                Token::SeqEnd,
                Token::Str("exempt_roles"),
                Token::Seq { len: Some(1) },
                Token::NewtypeStruct { name: "Id" },
                Token::Str("5"),
                Token::SeqEnd,
                Token::Str("guild_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("6"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("name"),
                Token::Str("rule"),
                Token::Str("trigger_metadata"),
                Token::Struct {
                    name: "AutoModerationTriggerMetadata",
                    len: 2,
                },
                Token::Str("keyword_filter"),
                Token::Some,
                Token::Seq { len: Some(2) },
                Token::Str("shoot"),
                Token::Str("darn"),
                Token::SeqEnd,
                Token::Str("regex_patterns"),
                Token::Some,
                Token::Seq { len: Some(2) },
                Token::Str("[a-z]+"),
                Token::Str("[0-9]+"),
                Token::SeqEnd,
                Token::StructEnd,
                Token::Str("trigger_type"),
                Token::U8(u8::from(AutoModerationTriggerType::Keyword)),
                Token::StructEnd,
            ],
        );
    }
}
