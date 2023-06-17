use bitflags::bitflags;

bitflags! {
    /// A set of bitflags which can be used to specify what resource to process
    /// into the cache.
    ///
    /// For example, specifying [`CHANNEL`] but not [`MESSAGE`] will cache
    /// created channels, channel updates, and channel deletes, but not their
    /// messages.
    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    pub struct ResourceType: u64 {
        /// Information relating to channels.
        const CHANNEL = 1;
        /// Information relating to emojis.
        const EMOJI = 1 << 1;
        /// Information relating to guilds.
        const GUILD = 1 << 2;
        /// Information relating to members.
        const MEMBER = 1 << 3;
        /// Information relating to messages.
        const MESSAGE = 1 << 4;
        /// Information relating to presences.
        const PRESENCE = 1 << 5;
        /// Information relating to reactions.
        const REACTION = 1 << 6;
        /// Information relating to roles.
        const ROLE = 1 << 7;
        /// Information relating the current user.
        const USER_CURRENT = 1 << 8;
        /// Information relating to users.
        const USER = 1 << 9;
        /// Information relating to voice states.
        const VOICE_STATE = 1 << 10;
        /// Information relating to stage instances.
        const STAGE_INSTANCE = 1 << 11;
        /// Information relating to guild integrations.
        const INTEGRATION = 1 << 12;
        /// Information relating to guild stickers.
        const STICKER = 1 << 13;
    }
}

/// Configuration for an [`InMemoryCache`].
///
/// [`InMemoryCache`]: crate::InMemoryCache
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub(super) resource_types: ResourceType,
    pub(super) message_cache_size: usize,
}

impl Config {
    /// Create a new default configuration.
    ///
    /// Refer to individual getters for their defaults.
    pub const fn new() -> Self {
        Self {
            resource_types: ResourceType::all(),
            message_cache_size: 100,
        }
    }

    /// Returns an immutable reference to the message cache size.
    ///
    /// Defaults to 100.
    pub const fn message_cache_size(&self) -> usize {
        self.message_cache_size
    }

    /// Returns a mutable reference to the message cache size.
    pub fn message_cache_size_mut(&mut self) -> &mut usize {
        &mut self.message_cache_size
    }
    /// Returns an immutable reference to the resource types enabled.
    ///
    /// Defaults to all resource types.
    pub const fn resource_types(&self) -> ResourceType {
        self.resource_types
    }

    /// Returns a mutable reference to the resource types enabled.
    pub fn resource_types_mut(&mut self) -> &mut ResourceType {
        &mut self.resource_types
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{Config, ResourceType};
    use static_assertions::assert_fields;

    assert_fields!(Config: resource_types, message_cache_size);

    #[test]
    fn defaults() {
        let conf = Config {
            resource_types: ResourceType::all(),
            message_cache_size: 100,
        };
        let default = Config::default();
        assert_eq!(conf.resource_types, default.resource_types);
        assert_eq!(conf.message_cache_size, default.message_cache_size);
    }
}
