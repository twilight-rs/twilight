use crate::channel::Channel;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

/// A channel has been updated.
///
/// # Channel obfuscation
///
/// A guild channel which a user does not have access to may be obfuscated. Once
/// the current user has access to a guild channel the previously obfuscated
/// fields will have their true values revealed. Refer to the documentation for
/// [`Channel`] for more information on channel obfuscation.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ChannelUpdate(pub Channel);

impl Deref for ChannelUpdate {
    type Target = Channel;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ChannelUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
