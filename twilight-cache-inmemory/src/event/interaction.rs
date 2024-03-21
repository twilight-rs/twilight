use crate::{config::ResourceType, CacheableModels, InMemoryCache, UpdateCache};
use std::borrow::Cow;
use twilight_model::{
    application::interaction::InteractionData, gateway::payload::incoming::InteractionCreate,
};

impl<CacheModels: CacheableModels> UpdateCache<CacheModels> for InteractionCreate {
    fn update(&self, cache: &InMemoryCache<CacheModels>) {
        // Cache interaction member
        if cache.wants(ResourceType::MEMBER) {
            if let (Some(member), Some(guild_id)) = (&self.member, self.guild_id) {
                if let Some(user) = &member.user {
                    cache.cache_user(Cow::Borrowed(user), self.guild_id);

                    cache.cache_borrowed_partial_member(guild_id, member, user.id);
                }
            }
        }

        // Cache interaction user
        if cache.wants(ResourceType::USER) {
            if let Some(user) = &self.user {
                cache.cache_user(Cow::Borrowed(user), None);
            }
        }

        // Cache resolved interaction data
        if let Some(InteractionData::ApplicationCommand(data)) = &self.data {
            if let Some(resolved) = &data.resolved {
                // Cache resolved users and members
                for u in resolved.users.values() {
                    if cache.wants(ResourceType::USER) {
                        cache.cache_user(Cow::Borrowed(u), self.guild_id);
                    }

                    if !cache.wants(ResourceType::MEMBER) || self.guild_id.is_none() {
                        continue;
                    }

                    // This should always match, because resolved members
                    // are guaranteed to have a matching resolved user
                    if let Some(member) = &resolved.members.get(&u.id) {
                        if let Some(guild_id) = self.guild_id {
                            cache.cache_borrowed_interaction_member(guild_id, member, u.id);
                        }
                    }
                }

                // Cache resolved roles
                if cache.wants(ResourceType::ROLE) {
                    if let Some(guild_id) = self.guild_id {
                        cache.cache_roles(guild_id, resolved.roles.values().cloned());
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DefaultInMemoryCache;
    use std::collections::HashMap;
    use twilight_model::{
        application::{
            command::CommandType,
            interaction::{
                application_command::CommandData, Interaction, InteractionData,
                InteractionDataResolved, InteractionMember, InteractionType,
            },
        },
        channel::{
            message::{
                sticker::{MessageSticker, StickerFormatType},
                MessageFlags, MessageType,
            },
            Channel, ChannelType, Message,
        },
        gateway::payload::incoming::InteractionCreate,
        guild::{MemberFlags, PartialMember, Permissions, Role, RoleFlags},
        id::Id,
        oauth::ApplicationIntegrationMap,
        user::User,
        util::{image_hash::ImageHashParseError, ImageHash, Timestamp},
    };

    #[allow(clippy::too_many_lines, deprecated)]
    #[test]
    fn interaction_create() -> Result<(), ImageHashParseError> {
        let timestamp = Timestamp::from_secs(1_632_072_645).expect("non zero");
        // let avatar1 = ImageHash::parse(b"1ef6bca4fddaa303a9cd32dd70fb395d")?;
        let avatar2 = ImageHash::parse(b"3a43231a99f4dfcf0fd94d1d8defd301")?;
        let avatar3 = ImageHash::parse(b"5e23c298295ad37936cfe24ad314774f")?;
        let flags = MemberFlags::BYPASSES_VERIFICATION | MemberFlags::DID_REJOIN;

        let cache = DefaultInMemoryCache::new();

        cache.update(&InteractionCreate(Interaction {
            app_permissions: Permissions::SEND_MESSAGES,
            application_id: Id::new(1),
            authorizing_integration_owners: ApplicationIntegrationMap {
                guild: None,
                user: None,
            },
            channel: Some(Channel {
                bitrate: None,
                guild_id: None,
                id: Id::new(400),
                kind: ChannelType::GuildText,
                last_message_id: None,
                last_pin_timestamp: None,
                name: None,
                nsfw: None,
                owner_id: None,
                parent_id: None,
                permission_overwrites: None,
                position: None,
                rate_limit_per_user: None,
                recipients: None,
                rtc_region: None,
                topic: None,
                user_limit: None,
                application_id: None,
                applied_tags: None,
                available_tags: None,
                default_auto_archive_duration: None,
                default_forum_layout: None,
                default_reaction_emoji: None,
                default_sort_order: None,
                default_thread_rate_limit_per_user: None,
                flags: None,
                icon: None,
                invitable: None,
                managed: None,
                member: None,
                member_count: None,
                message_count: None,
                newly_created: None,
                thread_metadata: None,
                video_quality_mode: None,
            }),
            channel_id: Some(Id::new(2)),
            data: Some(InteractionData::ApplicationCommand(Box::new(CommandData {
                guild_id: None,
                id: Id::new(5),
                name: "command name".into(),
                kind: CommandType::ChatInput, // This isn't actually a valid command, so just mark it as a slash command.
                options: Vec::new(),
                resolved: Some(InteractionDataResolved {
                    attachments: HashMap::new(),
                    channels: HashMap::new(),
                    members: HashMap::from([(
                        Id::new(7),
                        InteractionMember {
                            avatar: None,
                            communication_disabled_until: None,
                            flags,
                            joined_at: Some(timestamp),
                            nick: None,
                            pending: false,
                            permissions: Permissions::empty(),
                            premium_since: None,
                            roles: vec![Id::new(8)],
                        },
                    )]),
                    messages: HashMap::from([(
                        Id::new(4),
                        Message {
                            activity: None,
                            application: None,
                            application_id: None,
                            attachments: Vec::new(),
                            author: User {
                                accent_color: None,
                                avatar: Some(avatar3),
                                avatar_decoration: None,
                                banner: None,
                                bot: false,
                                discriminator: 1,
                                email: None,
                                flags: None,
                                global_name: Some("test".to_owned()),
                                id: Id::new(3),
                                locale: None,
                                mfa_enabled: None,
                                name: "test".to_owned(),
                                premium_type: None,
                                public_flags: None,
                                system: None,
                                verified: None,
                            },
                            channel_id: Id::new(2),
                            components: Vec::new(),
                            content: "ping".to_owned(),
                            edited_timestamp: None,
                            embeds: Vec::new(),
                            flags: Some(MessageFlags::empty()),
                            guild_id: Some(Id::new(1)),
                            id: Id::new(4),
                            interaction: None,
                            interaction_metadata: None,
                            kind: MessageType::Regular,
                            member: Some(PartialMember {
                                avatar: None,
                                communication_disabled_until: None,
                                deaf: false,
                                flags,
                                joined_at: Some(timestamp),
                                mute: false,
                                nick: Some("member nick".to_owned()),
                                permissions: None,
                                premium_since: None,
                                roles: Vec::new(),
                                user: None,
                            }),
                            mention_channels: Vec::new(),
                            mention_everyone: false,
                            mention_roles: Vec::new(),
                            mentions: Vec::new(),
                            pinned: false,
                            reactions: Vec::new(),
                            reference: None,
                            referenced_message: None,
                            role_subscription_data: None,
                            sticker_items: vec![MessageSticker {
                                format_type: StickerFormatType::Png,
                                id: Id::new(1),
                                name: "sticker name".to_owned(),
                            }],
                            timestamp,
                            thread: None,
                            tts: false,
                            webhook_id: None,
                        },
                    )]),
                    roles: HashMap::from([(
                        Id::new(8),
                        Role {
                            color: 0u32,
                            hoist: false,
                            icon: None,
                            id: Id::new(8),
                            managed: false,
                            mentionable: true,
                            name: "role name".into(),
                            permissions: Permissions::empty(),
                            position: 2i64,
                            flags: RoleFlags::empty(),
                            tags: None,
                            unicode_emoji: None,
                        },
                    )]),
                    users: HashMap::from([(
                        Id::new(7),
                        User {
                            accent_color: None,
                            avatar: Some(avatar2),
                            avatar_decoration: None,
                            banner: None,
                            bot: false,
                            discriminator: 5678,
                            email: None,
                            flags: None,
                            global_name: Some("different name".to_owned()),
                            id: Id::new(7),
                            locale: None,
                            mfa_enabled: None,
                            name: "different name".into(),
                            premium_type: None,
                            public_flags: None,
                            system: None,
                            verified: None,
                        },
                    )]),
                }),
                target_id: None,
            }))),
            guild_id: Some(Id::new(3)),
            guild_locale: None,
            id: Id::new(4),
            kind: InteractionType::ApplicationCommand,
            locale: Some("en-GB".to_owned()),
            member: Some(PartialMember {
                avatar: None,
                communication_disabled_until: None,
                deaf: false,
                flags,
                joined_at: Some(timestamp),
                mute: false,
                nick: None,
                permissions: Some(Permissions::empty()),
                premium_since: None,
                roles: Vec::new(),
                user: Some(User {
                    accent_color: None,
                    avatar: Some(avatar3),
                    avatar_decoration: None,
                    banner: None,
                    bot: false,
                    discriminator: 1234,
                    email: None,
                    flags: None,
                    global_name: Some("test".to_owned()),
                    id: Id::new(6),
                    locale: None,
                    mfa_enabled: None,
                    name: "username".into(),
                    premium_type: None,
                    public_flags: None,
                    system: None,
                    verified: None,
                }),
            }),
            message: None,
            token: "token".into(),
            user: None,
        }));

        {
            let guild_members = cache.guild_members(Id::new(3)).unwrap();
            assert_eq!(guild_members.len(), 2);
        }

        {
            let member = cache.member(Id::new(3), Id::new(6)).unwrap();
            let user = cache.user(member.user_id).unwrap();
            assert_eq!(user.avatar.as_ref().unwrap(), &avatar3);
        }

        {
            let member = cache.member(Id::new(3), Id::new(7)).unwrap();
            let user = cache.user(member.user_id).unwrap();
            assert_eq!(user.avatar.as_ref().unwrap(), &avatar2);
        }

        {
            let guild_roles = cache.guild_roles(Id::new(3)).unwrap();
            assert_eq!(guild_roles.len(), 1);
        }

        Ok(())
    }
}
