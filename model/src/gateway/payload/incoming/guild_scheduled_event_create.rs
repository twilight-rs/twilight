use crate::scheduled_event::GuildScheduledEvent;
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
