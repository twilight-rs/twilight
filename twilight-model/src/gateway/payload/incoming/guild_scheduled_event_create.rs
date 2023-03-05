use crate::guild::scheduled_event::GuildScheduledEvent;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

/// Sent when a guild scheduled event is created. The inner payload is a [`GuildScheduledEvent`].
///
/// [`GuildScheduledEvent`]: crate::guild::scheduled_event::GuildScheduledEvent
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct GuildScheduledEventCreate(pub GuildScheduledEvent);

impl Deref for GuildScheduledEventCreate {
    type Target = GuildScheduledEvent;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GuildScheduledEventCreate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
