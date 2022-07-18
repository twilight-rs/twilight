use crate::scheduled_event::GuildScheduledEvent;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

/// Sent when a guild scheduled event is updated. The inner payload is a [`GuildScheduledEvent`].
///
/// [`GuildScheduledEvent`]: crate::scheduled_event::GuildScheduledEvent
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct GuildScheduledEventUpdate(pub GuildScheduledEvent);

impl Deref for GuildScheduledEventUpdate {
    type Target = GuildScheduledEvent;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GuildScheduledEventUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
