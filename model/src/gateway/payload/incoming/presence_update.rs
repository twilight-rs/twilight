use crate::gateway::presence::Presence;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

/// User's presence was updated.
///
/// This may be received when a user's activity, status, or user
/// information - such as avatar or username - is updated.
///
/// Requires the [`Intents::GUILD_PRESENCES`] intent to receive this event.
///
/// Refer to [Discord Docs/Presence Update] for additional information.
///
/// [`Intents::GUILD_PRESENCES`]: crate::gateway::Intents
/// [Discord Docs/Presence Update]: https://discord.com/developers/docs/topics/gateway#presence-update
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PresenceUpdate(pub Presence);

impl Deref for PresenceUpdate {
    type Target = Presence;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PresenceUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::PresenceUpdate;
    use serde::{Deserialize, Serialize};
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash, ops::{Deref, DerefMut}};

    assert_impl_all!(PresenceUpdate: Clone, Debug, Deref, DerefMut, Deserialize<'static>, Eq, Hash, PartialEq, Serialize, Send, Sync);
}
