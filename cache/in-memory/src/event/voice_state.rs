use crate::{config::ResourceType, InMemoryCache, UpdateCache};
use twilight_model::{gateway::payload::incoming::VoiceStateUpdate, voice::VoiceState};

impl InMemoryCache {
    pub(crate) fn cache_voice_states(&self, voice_states: impl IntoIterator<Item = VoiceState>) {
        for voice_state in voice_states {
            self.cache_voice_state(voice_state);
        }
    }

    fn cache_voice_state(&self, voice_state: VoiceState) {
        // This should always exist, but just in case use a match
        let guild_id = match voice_state.guild_id {
            Some(id) => id,
            None => return,
        };

        let user_id = voice_state.user_id;

        // Check if the user is switching channels in the same guild (ie. they already have a voice state entry)
        if let Some(voice_state) = self.voice_states.get(&(guild_id, user_id)) {
            if let Some(channel_id) = voice_state.channel_id {
                let remove_channel_mapping = self
                    .voice_state_channels
                    .get_mut(&channel_id)
                    .map(|mut channel_voice_states| {
                        channel_voice_states.remove(&(guild_id, user_id));

                        channel_voice_states.is_empty()
                    })
                    .unwrap_or_default();

                if remove_channel_mapping {
                    self.voice_state_channels.remove(&channel_id);
                }
            }
        }

        // Check if the voice channel_id does not exist, signifying that the user has left
        if voice_state.channel_id.is_none() {
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

            return;
        }

        let maybe_channel_id = voice_state.channel_id;
        self.voice_states.insert((guild_id, user_id), voice_state);

        self.voice_state_guilds
            .entry(guild_id)
            .or_default()
            .insert(user_id);

        if let Some(channel_id) = maybe_channel_id {
            self.voice_state_channels
                .entry(channel_id)
                .or_default()
                .insert((guild_id, user_id));
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
    use std::str::FromStr;

    use super::*;
    use crate::test;
    use twilight_model::{
        datetime::Timestamp,
        id::{ChannelId, GuildId, UserId},
    };

    #[test]
    fn test_voice_state_inserts_and_removes() {
        let cache = InMemoryCache::new();

        // Note: Channel ids are `<guildid><idx>` where idx is the index of the channel id
        // This is done to prevent channel id collisions between guilds
        // The other 2 ids are not special since they can't overlap

        // User 1 joins guild 1's channel 11 (1 channel, 1 guild)
        {
            // Ids for this insert
            let (guild_id, channel_id, user_id) = (
                GuildId::new(1).expect("non zero"),
                ChannelId::new(11).expect("non zero"),
                UserId::new(1).expect("non zero"),
            );
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
            let (guild_id, channel_id, user_id) = (
                GuildId::new(2).expect("non zero"),
                ChannelId::new(21).expect("non zero"),
                UserId::new(2).expect("non zero"),
            );
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
            let (guild_id, channel_id, user_id) = (
                GuildId::new(1).expect("non zero"),
                ChannelId::new(12).expect("non zero"),
                UserId::new(3).expect("non zero"),
            );
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
            let (guild_id, channel_id, user_id) = (
                GuildId::new(1).expect("non zero"),
                ChannelId::new(11).expect("non zero"),
                UserId::new(3).expect("non zero"),
            );
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
            let (guild_id, channel_id, user_id) = (
                GuildId::new(1).expect("non zero"),
                ChannelId::new(11).expect("non zero"),
                UserId::new(3).expect("non zero"),
            );
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
            let (guild_id, channel_id, user_id) = (
                GuildId::new(2).expect("non zero"),
                ChannelId::new(21).expect("non zero"),
                UserId::new(2).expect("non zero"),
            );
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
            let (guild_id, _channel_id, user_id) = (
                GuildId::new(1).expect("non zero"),
                ChannelId::new(11).expect("non zero"),
                UserId::new(1).expect("non zero"),
            );
            cache.cache_voice_state(test::voice_state(guild_id, None, user_id));

            // Since the last person has disconnected, the global voice states, guilds, and channels should all be gone
            assert!(cache.voice_states.is_empty());
            assert!(cache.voice_state_channels.is_empty());
            assert!(cache.voice_state_guilds.is_empty());
        }
    }

    #[test]
    fn test_voice_states() {
        let cache = InMemoryCache::new();
        cache.cache_voice_state(test::voice_state(
            GuildId::new(1).expect("non zero"),
            Some(ChannelId::new(2).expect("non zero")),
            UserId::new(3).expect("non zero"),
        ));
        cache.cache_voice_state(test::voice_state(
            GuildId::new(1).expect("non zero"),
            Some(ChannelId::new(2).expect("non zero")),
            UserId::new(4).expect("non zero"),
        ));

        // Returns both voice states for the channel that exists.
        assert_eq!(
            2,
            cache
                .voice_channel_states(ChannelId::new(2).expect("non zero"))
                .unwrap()
                .count()
        );

        // Returns None if the channel does not exist.
        assert!(cache
            .voice_channel_states(ChannelId::new(1).expect("non zero"))
            .is_none());
    }

    #[test]
    fn test_voice_states_with_no_cached_guilds() {
        let cache = InMemoryCache::builder()
            .resource_types(ResourceType::VOICE_STATE)
            .build();

        cache.update(&VoiceStateUpdate(VoiceState {
            channel_id: None,
            deaf: false,
            guild_id: Some(GuildId::new(1).expect("non zero")),
            member: None,
            mute: false,
            self_deaf: false,
            self_mute: false,
            self_stream: false,
            session_id: "38fj3jfkh3pfho3prh2".to_string(),
            suppress: false,
            token: None,
            user_id: UserId::new(1).expect("non zero"),
            request_to_speak_timestamp: Some(
                Timestamp::from_str("2021-04-21T22:16:50+00:00").expect("proper datetime"),
            ),
        }));
    }

    #[test]
    fn test_voice_states_members() {
        use twilight_model::{guild::member::Member, user::User};

        let cache = InMemoryCache::new();

        let mutation = VoiceStateUpdate(VoiceState {
            channel_id: Some(ChannelId::new(4).expect("non zero")),
            deaf: false,
            guild_id: Some(GuildId::new(2).expect("non zero")),
            member: Some(Member {
                deaf: false,
                guild_id: GuildId::new(2).expect("non zero"),
                joined_at: None,
                mute: false,
                nick: None,
                pending: false,
                premium_since: None,
                roles: Vec::new(),
                user: User {
                    accent_color: None,
                    avatar: Some("".to_owned()),
                    banner: None,
                    bot: false,
                    discriminator: 1,
                    email: None,
                    flags: None,
                    id: UserId::new(3).expect("non zero"),
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
            session_id: "".to_owned(),
            suppress: false,
            token: None,
            user_id: UserId::new(3).expect("non zero"),
            request_to_speak_timestamp: Some(
                Timestamp::from_str("2021-04-21T22:16:50+00:00").expect("proper datetime"),
            ),
        });

        cache.update(&mutation);

        assert_eq!(cache.members.len(), 1);
        {
            let entry = cache
                .user_guilds
                .get(&UserId::new(3).expect("non zero"))
                .unwrap();
            assert_eq!(entry.value().len(), 1);
        }
        assert_eq!(
            cache
                .member(
                    GuildId::new(2).expect("non zero"),
                    UserId::new(3).expect("non zero")
                )
                .unwrap()
                .user_id,
            UserId::new(3).expect("non zero"),
        );
    }
}
