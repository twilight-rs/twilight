use bitflags::bitflags;

bitflags! {
    pub struct EventType: u64 {
        const BAN_ADD = 1;
        const BAN_REMOVE = 1 << 1;
        const CHANNEL_CREATE = 1 << 2;
        const CHANNEL_DELETE = 1 << 3;
        const CHANNEL_PINS_UPDATE = 1 << 4;
        const CHANNEL_UPDATE = 1 << 5;
        const GUILD_CREATE = 1 << 6;
        const GUILD_DELETE = 1 << 7;
        const GUILD_EMOJIS_UPDATE = 1 << 8;
        const GUILD_INTEGRATIONS_UPDATE = 1 << 9;
        const GUILD_UPDATE = 1 << 10;
        const MEMBER_ADD = 1 << 11;
        const MEMBER_CHUNK = 1 << 12;
        const MEMBER_REMOVE = 1 << 13;
        const MEMBER_UPDATE = 1 << 14;
        const MESSAGE_CREATE = 1 << 15;
        const MESSAGE_DELETE = 1 << 16;
        const MESSAGE_DELETE_BULK = 1 << 17;
        const MESSAGE_UPDATE = 1 << 18;
        const PRESENCE_UPDATE = 1 << 19;
        const REACTION_ADD = 1 << 20;
        const REACTION_REMOVE = 1 << 21;
        const REACTION_REMOVE_ALL = 1 << 22;
        const READY = 1 << 23;
        const ROLE_CREATE = 1 << 24;
        const ROLE_DELETE = 1 << 25;
        const ROLE_UPDATE = 1 << 26;
        const TYPING_START = 1 << 27;
        const UNAVAILABLE_GUILD = 1 << 28;
        const USER_UPDATE = 1 << 29;
        const VOICE_SERVER_UPDATE = 1 << 30;
        const VOICE_STATE_UPDATE = 1 << 31;
        const WEBHOOKS_UPDATE = 1 << 32;
    }
}

#[derive(Clone, Debug)]
pub struct InMemoryConfig {
    event_types: EventType,
    message_cache_size: usize,
}

impl InMemoryConfig {
    /// Creates a new builder to make a configuration.
    pub fn builder() -> InMemoryConfigBuilder {
        InMemoryConfigBuilder::default()
    }

    /// Returns an immutable reference to the event types enabled.
    pub fn event_types(&self) -> EventType {
        self.event_types
    }

    /// Returns a mutable reference to the event types enabled.
    pub fn event_types_mut(&mut self) -> &mut EventType {
        &mut self.event_types
    }

    /// Returns an immutable reference to the message cache size.
    pub fn message_cache_size(&self) -> usize {
        self.message_cache_size
    }

    /// Returns a mutable reference to the message cache size.
    pub fn message_cache_size_mut(&mut self) -> &mut usize {
        &mut self.message_cache_size
    }
}

impl Default for InMemoryConfig {
    fn default() -> Self {
        Self {
            event_types: EventType::all(),
            message_cache_size: 100,
        }
    }
}

impl From<InMemoryConfigBuilder> for InMemoryConfig {
    fn from(builder: InMemoryConfigBuilder) -> Self {
        builder.build()
    }
}

#[derive(Clone, Debug, Default)]
pub struct InMemoryConfigBuilder(InMemoryConfig);

impl InMemoryConfigBuilder {
    /// Creates a new, default builder for a [`InMemoryConfig`].
    ///
    /// [`InMemoryConfig`]: struct.InMemoryConfig.html
    pub fn new() -> Self {
        Self::default()
    }

    /// Consumes the builder, returning the built configuration.
    pub fn build(self) -> InMemoryConfig {
        self.0
    }

    /// Sets the list of event types for the cache to handle.
    ///
    /// Defaults to all types.
    pub fn event_types(mut self, event_types: EventType) -> Self {
        self.0.event_types = event_types;

        self
    }

    /// Sets the number of messages to cache per channel.
    ///
    /// Defaults to 100.
    pub fn message_cache_size(mut self, message_cache_size: usize) -> Self {
        self.0.message_cache_size = message_cache_size;

        self
    }
}

#[cfg(test)]
mod tests {
    use super::{EventType, InMemoryConfig, InMemoryConfigBuilder};

    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn test_event_type_const_values() {
        assert_eq!(1, EventType::BAN_ADD.bits());
        assert_eq!(1 << 1, EventType::BAN_REMOVE.bits());
        assert_eq!(1 << 2, EventType::CHANNEL_CREATE.bits());
        assert_eq!(1 << 3, EventType::CHANNEL_DELETE.bits());
        assert_eq!(1 << 4, EventType::CHANNEL_PINS_UPDATE.bits());
        assert_eq!(1 << 5, EventType::CHANNEL_UPDATE.bits());
        assert_eq!(1 << 6, EventType::GUILD_CREATE.bits());
        assert_eq!(1 << 7, EventType::GUILD_DELETE.bits());
        assert_eq!(1 << 8, EventType::GUILD_EMOJIS_UPDATE.bits());
        assert_eq!(1 << 9, EventType::GUILD_INTEGRATIONS_UPDATE.bits());
        assert_eq!(1 << 10, EventType::GUILD_UPDATE.bits());
        assert_eq!(1 << 11, EventType::MEMBER_ADD.bits());
        assert_eq!(1 << 12, EventType::MEMBER_CHUNK.bits());
        assert_eq!(1 << 13, EventType::MEMBER_REMOVE.bits());
        assert_eq!(1 << 14, EventType::MEMBER_UPDATE.bits());
        assert_eq!(1 << 15, EventType::MESSAGE_CREATE.bits());
        assert_eq!(1 << 16, EventType::MESSAGE_DELETE.bits());
        assert_eq!(1 << 17, EventType::MESSAGE_DELETE_BULK.bits());
        assert_eq!(1 << 18, EventType::MESSAGE_UPDATE.bits());
        assert_eq!(1 << 19, EventType::PRESENCE_UPDATE.bits());
        assert_eq!(1 << 20, EventType::REACTION_ADD.bits());
        assert_eq!(1 << 21, EventType::REACTION_REMOVE.bits());
        assert_eq!(1 << 22, EventType::REACTION_REMOVE_ALL.bits());
        assert_eq!(1 << 23, EventType::READY.bits());
        assert_eq!(1 << 24, EventType::ROLE_CREATE.bits());
        assert_eq!(1 << 25, EventType::ROLE_DELETE.bits());
        assert_eq!(1 << 26, EventType::ROLE_UPDATE.bits());
        assert_eq!(1 << 27, EventType::TYPING_START.bits());
        assert_eq!(1 << 28, EventType::UNAVAILABLE_GUILD.bits());
        assert_eq!(1 << 29, EventType::USER_UPDATE.bits());
        assert_eq!(1 << 30, EventType::VOICE_SERVER_UPDATE.bits());
        assert_eq!(1 << 31, EventType::VOICE_STATE_UPDATE.bits());
        assert_eq!(1 << 32, EventType::WEBHOOKS_UPDATE.bits());
    }

    #[test]
    fn test_defaults() {
        let conf = InMemoryConfig {
            event_types: EventType::all(),
            message_cache_size: 100,
        };
        let default = InMemoryConfig::default();
        assert_eq!(conf.event_types, default.event_types);
        assert_eq!(conf.message_cache_size, default.message_cache_size);
        let default = InMemoryConfigBuilder::default();
        assert_eq!(conf.event_types, default.0.event_types);
        assert_eq!(conf.message_cache_size, default.0.message_cache_size);
    }

    #[test]
    fn test_config_fields() {
        static_assertions::assert_fields!(InMemoryConfig: event_types, message_cache_size);
    }
}
