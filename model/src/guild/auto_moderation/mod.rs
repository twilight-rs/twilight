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
mod rule;
mod trigger_metadata;
mod trigger_type;

pub use self::{
    action::{AutoModerationAction, AutoModerationActionMetadata, AutoModerationActionType},
    event_type::AutoModerationEventType,
    preset_type::AutoModerationKeywordPresetType,
    rule::AutoModerationRule,
    trigger_metadata::AutoModerationTriggerMetadata,
    trigger_type::AutoModerationTriggerType,
};
