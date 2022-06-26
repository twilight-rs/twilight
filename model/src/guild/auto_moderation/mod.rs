//! Auto Moderation is a feature which allows each guild to set up rules that
//! trigger based on some criteria. For example, a rule can trigger whenever a
//! message contains a specific keyword.
//!
//! Rules can be configured to automatically execute actions whenever they
//! trigger. For example, if a user tries to send a message which contains a
//! certain keyword, a rule can trigger and block the message before it is sent.

#![deny(missing_docs)]

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
