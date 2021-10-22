use crate::EventTypeFlags;
use std::sync::Arc;
use twilight_gateway_queue::Queue;
use twilight_http::Client;
use twilight_model::gateway::{
    payload::outgoing::{identify::IdentifyProperties, update_presence::UpdatePresencePayload},
    Intents,
};

/// The configuration used by the shard to identify with the gateway and
/// operate.
///
/// Use [`Shard::builder`] to start creating a configured shard.
///
/// [`Shard::builder`]: super::Shard::builder
#[derive(Clone, Debug)]
pub struct Config {
    pub(crate) event_types: EventTypeFlags,
    pub(crate) gateway_url: Option<Box<str>>,
    pub(crate) http_client: Arc<Client>,
    pub(super) identify_properties: Option<IdentifyProperties>,
    pub(super) intents: Intents,
    pub(super) large_threshold: u64,
    pub(super) presence: Option<UpdatePresencePayload>,
    pub(super) queue: Arc<dyn Queue>,
    pub(crate) shard: [u64; 2],
    pub(super) token: Box<str>,
    pub(crate) session_id: Option<Box<str>>,
    pub(crate) sequence: Option<u64>,
}

impl Config {
    /// Copy of the event type flags.
    pub const fn event_types(&self) -> EventTypeFlags {
        self.event_types
    }

    /// Return an immutable reference to the url used to connect to the gateway.
    pub fn gateway_url(&self) -> Option<&str> {
        self.gateway_url.as_deref()
    }

    /// Return an immutable reference to the `twilight_http` client to be used
    /// by the shard.
    pub fn http_client(&self) -> &Client {
        &self.http_client
    }

    /// Return an immutable reference to the identification properties the shard
    /// will use.
    pub const fn identify_properties(&self) -> Option<&IdentifyProperties> {
        self.identify_properties.as_ref()
    }

    /// Return a copy of the intents that the gateway is using.
    pub const fn intents(&self) -> Intents {
        self.intents
    }

    /// Return the maximum threshold at which point the gateway will stop
    /// sending a guild's member list in Guild Create events.
    pub const fn large_threshold(&self) -> u64 {
        self.large_threshold
    }

    /// Return an immutable reference to the presence to set when identifying
    /// with the gateway.
    ///
    /// This will be the bot's presence. For example, setting the online status
    /// to Do Not Disturb will show the status in the bot's presence.
    pub const fn presence(&self) -> Option<&UpdatePresencePayload> {
        self.presence.as_ref()
    }

    /// The shard's ID and the total number of shards used by the bot.
    pub const fn shard(&self) -> [u64; 2] {
        self.shard
    }

    /// Return an immutable reference to the token used to authenticate with
    /// when identifying with the gateway.
    pub const fn token(&self) -> &str {
        &self.token
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(Config: Clone, Debug, Send, Sync);
}
