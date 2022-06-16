use crate::guild::auto_moderation::AutoModerationRule;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

/// Sent when an auto moderation rule is updated.
///
/// Sent to bot users with [`Permissions::MANAGE_GUILD`].
///
/// [`Permissions::MANAGE_GUILD`]: crate::guild::Permissions::MANAGE_GUILD
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AutoModerationRuleUpdate(pub AutoModerationRule);

impl Deref for AutoModerationRuleUpdate {
    type Target = AutoModerationRule;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AutoModerationRuleUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
