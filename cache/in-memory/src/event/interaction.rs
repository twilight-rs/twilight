use crate::{config::ResourceType, InMemoryCache, UpdateCache};
use std::borrow::Cow;
use twilight_model::{
    application::interaction::Interaction, gateway::payload::incoming::InteractionCreate,
};

impl UpdateCache for InteractionCreate {
    fn update(&self, cache: &InMemoryCache) {
        #[allow(clippy::single_match)]
        match &self.0 {
            Interaction::ApplicationCommand(command) => {
                if cache.wants(ResourceType::MEMBER) {
                    if let Some(member) = &command.member {
                        if let Some(user) = &member.user {
                            cache.cache_user(Cow::Borrowed(user), command.guild_id);

                            cache.cache_borrowed_partial_member(
                                command.guild_id.unwrap(),
                                member,
                                user.id,
                            );
                        }
                    }
                }

                if let Some(user) = &command.user {
                    cache.cache_user(Cow::Borrowed(user), None);
                }

                if let Some(resolved) = &command.data.resolved {
                    for u in &resolved.users {
                        cache.cache_user(Cow::Borrowed(u), command.guild_id);

                        if !cache.wants(ResourceType::MEMBER) || command.guild_id.is_none() {
                            continue;
                        }

                        // This should always match, because resolved members
                        // are guaranteed to have a matching resolved user
                        if let Some(member) = &resolved.members.iter().find(|m| m.id == u.id) {
                            if let Some(guild_id) = command.guild_id {
                                cache.cache_borrowed_interaction_member(guild_id, member);
                            }
                        }
                    }

                    if cache.wants(ResourceType::ROLE) {
                        if let Some(guild_id) = command.guild_id {
                            cache.cache_roles(guild_id, resolved.roles.iter().cloned());
                        }
                    }
                }
            }
            _ => {}
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use twilight_model::{
        application::interaction::{
            application_command::{CommandData, CommandInteractionDataResolved, InteractionMember},
            ApplicationCommand, InteractionType,
        },
        channel::{
            message::{
                sticker::{MessageSticker, StickerFormatType, StickerId},
                MessageFlags, MessageType,
            },
            Message,
        },
        datetime::Timestamp,
        guild::{PartialMember, Permissions, Role},
        id::{
            ApplicationId, ChannelId, CommandId, GuildId, InteractionId, MessageId, RoleId, UserId,
        },
        user::User,
    };

    #[test]
    fn test_interaction_create() {
        let timestamp = Timestamp::from_secs(1_632_072_645).expect("non zero");

        let cache = InMemoryCache::new();
        cache.update(&InteractionCreate(Interaction::ApplicationCommand(
            Box::new(ApplicationCommand {
                application_id: ApplicationId::new(1).expect("non zero"),
                channel_id: ChannelId::new(2).expect("non zero"),
                data: CommandData {
                    id: CommandId::new(5).expect("non zero"),
                    name: "command name".into(),
                    options: Vec::new(),
                    resolved: Some(CommandInteractionDataResolved {
                        channels: Vec::new(),
                        members: Vec::from([InteractionMember {
                            hoisted_role: None,
                            id: UserId::new(7).expect("non zero"),
                            joined_at: Some(timestamp),
                            nick: None,
                            premium_since: None,
                            roles: vec![RoleId::new(8).expect("non zero")],
                        }]),
                        messages: Vec::from([Message {
                            activity: None,
                            application: None,
                            application_id: None,
                            attachments: Vec::new(),
                            author: User {
                                accent_color: None,
                                avatar: Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned()),
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
                            channel_id: ChannelId::new(2).expect("non zero"),
                            components: Vec::new(),
                            content: "ping".to_owned(),
                            edited_timestamp: None,
                            embeds: Vec::new(),
                            flags: Some(MessageFlags::empty()),
                            guild_id: Some(GuildId::new(1).expect("non zero")),
                            id: MessageId::new(4).expect("non zero"),
                            interaction: None,
                            kind: MessageType::Regular,
                            member: Some(PartialMember {
                                deaf: false,
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
                            sticker_items: vec![MessageSticker {
                                format_type: StickerFormatType::Png,
                                id: StickerId::new(1).expect("non zero"),
                                name: "sticker name".to_owned(),
                            }],
                            referenced_message: None,
                            thread: None,
                            timestamp,
                            tts: false,
                            webhook_id: None,
                        }]),
                        roles: Vec::from([Role {
                            color: 0u32,
                            hoist: false,
                            icon: None,
                            id: RoleId::new(8).expect("non zero"),
                            managed: false,
                            mentionable: true,
                            name: "role name".into(),
                            permissions: Permissions::empty(),
                            position: 2i64,
                            tags: None,
                            unicode_emoji: None,
                        }]),
                        users: Vec::from([User {
                            accent_color: None,
                            avatar: Some("different avatar".into()),
                            banner: None,
                            bot: false,
                            discriminator: 5678,
                            email: None,
                            flags: None,
                            id: UserId::new(7).expect("non zero"),
                            locale: None,
                            mfa_enabled: None,
                            name: "different name".into(),
                            premium_type: None,
                            public_flags: None,
                            system: None,
                            verified: None,
                        }]),
                    }),
                },
                guild_id: Some(GuildId::new(3).expect("non zero")),
                id: InteractionId::new(4).expect("non zero"),
                kind: InteractionType::ApplicationCommand,
                member: Some(PartialMember {
                    deaf: false,
                    joined_at: Some(timestamp),
                    mute: false,
                    nick: None,
                    permissions: Some(Permissions::empty()),
                    premium_since: None,
                    roles: Vec::new(),
                    user: Some(User {
                        accent_color: None,
                        avatar: Some("avatar string".into()),
                        banner: None,
                        bot: false,
                        discriminator: 1234,
                        email: None,
                        flags: None,
                        id: UserId::new(6).expect("non zero"),
                        locale: None,
                        mfa_enabled: None,
                        name: "username".into(),
                        premium_type: None,
                        public_flags: None,
                        system: None,
                        verified: None,
                    }),
                }),
                token: "token".into(),
                user: None,
            }),
        )));

        {
            let guild_members = cache
                .guild_members(GuildId::new(3).expect("non zero"))
                .unwrap();
            assert_eq!(guild_members.len(), 2);
        }

        {
            let member = cache
                .member(
                    GuildId::new(3).expect("non zero"),
                    UserId::new(6).expect("non zero"),
                )
                .unwrap();
            let user = cache.user(member.user_id).unwrap();
            assert_eq!(user.avatar.as_ref().unwrap(), "avatar string");
        }

        {
            let member = cache
                .member(
                    GuildId::new(3).expect("non zero"),
                    UserId::new(7).expect("non zero"),
                )
                .unwrap();
            let user = cache.user(member.user_id).unwrap();
            assert_eq!(user.avatar.as_ref().unwrap(), "different avatar");
        }

        {
            let guild_roles = cache
                .guild_roles(GuildId::new(3).expect("non zero"))
                .unwrap();
            assert_eq!(guild_roles.len(), 1);
        }
    }
}
