use crate::{config::ResourceType, model::CachedVoiceState, InMemoryCache, UpdateCache};
use twilight_model::{gateway::payload::incoming::VoiceStateUpdate, voice::VoiceState};

impl InMemoryCache {
    pub(crate) fn cache_voice_states(&self, voice_states: impl IntoIterator<Item = VoiceState>) {
        for voice_state in voice_states {
            self.cache_voice_state(voice_state);
        }
    }

    fn cache_voice_state(&self, voice_state: VoiceState) {
        // This should always exist, but let's check just in case.
        let guild_id = if let Some(id) = voice_state.guild_id {
            id
        } else {
            return;
        };

        let user_id = voice_state.user_id;

        // Check if the user is switching channels in the same guild (ie. they already have a voice state entry)
        if let Some(voice_state) = self.voice_states.get(&(guild_id, user_id)) {
            let remove_channel_mapping = self
                .voice_state_channels
                .get_mut(&voice_state.channel_id())
                .map(|mut channel_voice_states| {
                    channel_voice_states.remove(&(guild_id, user_id));

                    channel_voice_states.is_empty()
                })
                .unwrap_or_default();

            if remove_channel_mapping {
                self.voice_state_channels.remove(&voice_state.channel_id());
            }
        }

        if let Some(channel_id) = voice_state.channel_id {
            let cached_voice_state =
                CachedVoiceState::from_model(channel_id, guild_id, voice_state);

            self.voice_states
                .insert((guild_id, user_id), cached_voice_state);

            self.voice_state_guilds
                .entry(guild_id)
                .or_default()
                .insert(user_id);

            self.voice_state_channels
                .entry(channel_id)
                .or_default()
                .insert((guild_id, user_id));
        } else {
            // voice channel_id does not exist, signifying that the user has left
            {
                let remove_guild = self
                    .voice_state_guilds
                    .get_mut(&guild_id)
                    .map(|mut guild_users| {
                        guild_users.remove(&user_id);

                        guild_users.is_empty()
                    })
                    .unwrap_or_default();

                if remove_guild {
                    self.voice_state_guilds.remove(&guild_id);
                }
            }

            self.voice_states.remove(&(guild_id, user_id));
        }
    }
}

impl UpdateCache for VoiceStateUpdate {
    fn update(&self, cache: &InMemoryCache) {
        if !cache.wants(ResourceType::VOICE_STATE) {
            return;
        }

        cache.cache_voice_state(self.0.clone());

        if let (Some(guild_id), Some(member)) = (self.0.guild_id, &self.0.member) {
            cache.cache_member(guild_id, member.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{model::CachedVoiceState, test, InMemoryCache, ResourceType};
    use std::str::FromStr;
    use twilight_model::{
        gateway::payload::incoming::VoiceStateUpdate,
        guild::{Member, MemberFlags},
        id::{
            marker::{ChannelMarker, GuildMarker, UserMarker},
            Id,
        },
        user::User,
        util::{image_hash::ImageHashParseError, ImageHash, Timestamp},
        voice::VoiceState,
    };

    #[test]
    fn voice_state_inserts_and_removes() {
        let cache = InMemoryCache::new();

        // Note: Channel ids are `<guildid><idx>` where idx is the index of the channel id
        // This is done to prevent channel id collisions between guilds
        // The other 2 ids are not special since they can't overlap

        // User 1 joins guild 1's channel 11 (1 channel, 1 guild)
        {
            // Ids for this insert
            let (guild_id, channel_id, user_id) = (Id::new(1), Id::new(11), Id::new(1));
            cache.cache_voice_state(test::voice_state(guild_id, Some(channel_id), user_id));

            // The new user should show up in the global voice states
            assert!(cache.voice_states.contains_key(&(guild_id, user_id)));
            // There should only be the one new voice state in there
            assert_eq!(1, cache.voice_states.len());

            // The new channel should show up in the voice states by channel lookup
            assert!(cache.voice_state_channels.contains_key(&channel_id));
            assert_eq!(1, cache.voice_state_channels.len());

            // The new guild should also show up in the voice states by guild lookup
            assert!(cache.voice_state_guilds.contains_key(&guild_id));
            assert_eq!(1, cache.voice_state_guilds.len());
        }

        // User 2 joins guild 2's channel 21 (2 channels, 2 guilds)
        {
            // Ids for this insert
            let (guild_id, channel_id, user_id) = (Id::new(2), Id::new(21), Id::new(2));
            cache.cache_voice_state(test::voice_state(guild_id, Some(channel_id), user_id));

            // The new voice state should show up in the global voice states
            assert!(cache.voice_states.contains_key(&(guild_id, user_id)));
            // There should be two voice states now that we have inserted another
            assert_eq!(2, cache.voice_states.len());

            // The new channel should also show up in the voice states by channel lookup
            assert!(cache.voice_state_channels.contains_key(&channel_id));
            assert_eq!(2, cache.voice_state_channels.len());

            // The new guild should also show up in the voice states by guild lookup
            assert!(cache.voice_state_guilds.contains_key(&guild_id));
            assert_eq!(2, cache.voice_state_guilds.len());
        }

        // User 3 joins guild 1's channel 12  (3 channels, 2 guilds)
        {
            // Ids for this insert
            let (guild_id, channel_id, user_id) = (Id::new(1), Id::new(12), Id::new(3));
            cache.cache_voice_state(test::voice_state(guild_id, Some(channel_id), user_id));

            // The new voice state should show up in the global voice states
            assert!(cache.voice_states.contains_key(&(guild_id, user_id)));
            assert_eq!(3, cache.voice_states.len());

            // The new channel should also show up in the voice states by channel lookup
            assert!(cache.voice_state_channels.contains_key(&channel_id));
            assert_eq!(3, cache.voice_state_channels.len());

            // The guild should still show up in the voice states by guild lookup
            assert!(cache.voice_state_guilds.contains_key(&guild_id));
            // Since we have used a guild that has been inserted into the cache already, there
            // should not be a new guild in the map
            assert_eq!(2, cache.voice_state_guilds.len());
        }

        // User 3 moves to guild 1's channel 11 (2 channels, 2 guilds)
        {
            // Ids for this insert
            let (guild_id, channel_id, user_id) = (Id::new(1), Id::new(11), Id::new(3));
            cache.cache_voice_state(test::voice_state(guild_id, Some(channel_id), user_id));

            // The new voice state should show up in the global voice states
            assert!(cache.voice_states.contains_key(&(guild_id, user_id)));
            // The amount of global voice states should not change since it was a move, not a join
            assert_eq!(3, cache.voice_states.len());

            // The new channel should show up in the voice states by channel lookup
            assert!(cache.voice_state_channels.contains_key(&channel_id));
            // The old channel should be removed from the lookup table
            assert_eq!(2, cache.voice_state_channels.len());

            // The guild should still show up in the voice states by guild lookup
            assert!(cache.voice_state_guilds.contains_key(&guild_id));
            assert_eq!(2, cache.voice_state_guilds.len());
        }

        // User 3 dcs (2 channels, 2 guilds)
        {
            let (guild_id, channel_id, user_id) = (Id::new(1), Id::new(11), Id::new(3));
            cache.cache_voice_state(test::voice_state(guild_id, None, user_id));

            // Now that the user left, they should not show up in the voice states
            assert!(!cache.voice_states.contains_key(&(guild_id, user_id)));
            assert_eq!(2, cache.voice_states.len());

            // Since they were not alone in their channel, the channel and guild mappings should not disappear
            assert!(cache.voice_state_channels.contains_key(&channel_id));
            // assert_eq!(2, cache.voice_state_channels.len());
            assert!(cache.voice_state_guilds.contains_key(&guild_id));
            assert_eq!(2, cache.voice_state_guilds.len());
        }

        // User 2 dcs (1 channel, 1 guild)
        {
            let (guild_id, channel_id, user_id) = (Id::new(2), Id::new(21), Id::new(2));
            cache.cache_voice_state(test::voice_state(guild_id, None, user_id));

            // Now that the user left, they should not show up in the voice states
            assert!(!cache.voice_states.contains_key(&(guild_id, user_id)));
            assert_eq!(1, cache.voice_states.len());

            // Since they were the last in their channel, the mapping should disappear
            assert!(!cache.voice_state_channels.contains_key(&channel_id));
            assert_eq!(1, cache.voice_state_channels.len());

            // Since they were the last in their guild, the mapping should disappear
            assert!(!cache.voice_state_guilds.contains_key(&guild_id));
            assert_eq!(1, cache.voice_state_guilds.len());
        }

        // User 1 dcs (0 channels, 0 guilds)
        {
            let (guild_id, _channel_id, user_id) =
                (Id::new(1), Id::<ChannelMarker>::new(11), Id::new(1));
            cache.cache_voice_state(test::voice_state(guild_id, None, user_id));

            // Since the last person has disconnected, the global voice states, guilds, and channels should all be gone
            assert!(cache.voice_states.is_empty());
            assert!(cache.voice_state_channels.is_empty());
            assert!(cache.voice_state_guilds.is_empty());
        }
    }

    #[test]
    fn voice_states() {
        let cache = InMemoryCache::new();
        cache.cache_voice_state(test::voice_state(Id::new(1), Some(Id::new(2)), Id::new(3)));
        cache.cache_voice_state(test::voice_state(Id::new(1), Some(Id::new(2)), Id::new(4)));

        // Returns both voice states for the channel that exists.
        assert_eq!(2, cache.voice_channel_states(Id::new(2)).unwrap().count());

        // Returns None if the channel does not exist.
        assert!(cache.voice_channel_states(Id::new(1)).is_none());
    }

    #[test]
    fn voice_states_with_no_cached_guilds() {
        let cache = InMemoryCache::builder()
            .resource_types(ResourceType::VOICE_STATE)
            .build();

        cache.update(&VoiceStateUpdate(VoiceState {
            channel_id: None,
            deaf: false,
            guild_id: Some(Id::new(1)),
            member: None,
            mute: false,
            self_deaf: false,
            self_mute: false,
            self_stream: false,
            self_video: false,
            session_id: "38fj3jfkh3pfho3prh2".to_string(),
            suppress: false,
            user_id: Id::new(1),
            request_to_speak_timestamp: Some(
                Timestamp::from_str("2021-04-21T22:16:50+00:00").expect("proper datetime"),
            ),
        }));
    }

    #[test]
    fn voice_states_members() -> Result<(), ImageHashParseError> {
        let joined_at = Timestamp::from_secs(1_632_072_645).expect("non zero");

        let cache = InMemoryCache::new();

        let avatar = ImageHash::parse(b"169280485ba78d541a9090e7ea35a14e")?;
        let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;

        let mutation = VoiceStateUpdate(VoiceState {
            channel_id: Some(Id::new(4)),
            deaf: false,
            guild_id: Some(Id::new(2)),
            member: Some(Member {
                avatar: None,
                communication_disabled_until: None,
                deaf: false,
                flags,
                guild_id: Id::new(2),
                joined_at,
                mute: false,
                nick: None,
                pending: false,
                premium_since: None,
                roles: Vec::new(),
                user: User {
                    accent_color: None,
                    avatar: Some(avatar),
                    banner: None,
                    bot: false,
                    discriminator: 1,
                    email: None,
                    flags: None,
                    id: Id::new(3),
                    locale: None,
                    mfa_enabled: None,
                    name: "test".to_owned(),
                    premium_type: None,
                    public_flags: None,
                    system: None,
                    verified: None,
                },
            }),
            mute: false,
            self_deaf: false,
            self_mute: false,
            self_stream: false,
            self_video: false,
            session_id: String::new(),
            suppress: false,
            user_id: Id::new(3),
            request_to_speak_timestamp: Some(
                Timestamp::from_str("2021-04-21T22:16:50+00:00").expect("proper datetime"),
            ),
        });

        cache.update(&mutation);

        assert_eq!(cache.members.len(), 1);
        {
            let entry = cache.user_guilds.get(&Id::new(3)).unwrap();
            assert_eq!(entry.value().len(), 1);
        }
        assert_eq!(
            cache.member(Id::new(2), Id::new(3)).unwrap().user_id,
            Id::new(3),
        );

        Ok(())
    }

    /// Assert that the a cached variant of the voice state is correctly
    /// inserted.
    #[test]
    fn uses_cached_variant() {
        const CHANNEL_ID: Id<ChannelMarker> = Id::new(2);
        const GUILD_ID: Id<GuildMarker> = Id::new(1);
        const USER_ID: Id<UserMarker> = Id::new(3);

        let cache = InMemoryCache::new();
        let voice_state = test::voice_state(GUILD_ID, Some(CHANNEL_ID), USER_ID);
        cache.update(&VoiceStateUpdate(voice_state.clone()));

        let cached = CachedVoiceState::from_model(CHANNEL_ID, GUILD_ID, voice_state);
        let in_cache = cache.voice_state(USER_ID, GUILD_ID).unwrap();
        assert_eq!(in_cache.value(), &cached);
    }
}
