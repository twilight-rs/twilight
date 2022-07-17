use crate::guild::auto_moderation::AutoModerationRule;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

/// Auto moderation rule has been deleted.
///
/// Requires [`Permissions::MANAGE_GUILD`].
///
/// [`Permissions::MANAGE_GUILD`]: crate::guild::Permissions::MANAGE_GUILD
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct AutoModerationRuleDelete(pub AutoModerationRule);

impl Deref for AutoModerationRuleDelete {
    type Target = AutoModerationRule;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AutoModerationRuleDelete {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
