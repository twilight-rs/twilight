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
                    for u in resolved.users.values() {
                        cache.cache_user(Cow::Borrowed(u), command.guild_id);

                        if !cache.wants(ResourceType::MEMBER) || command.guild_id.is_none() {
                            continue;
                        }

                        // This should always match, because resolved members
                        // are guaranteed to have a matching resolved user
                        if let Some((&id, member)) =
                            &resolved.members.iter().find(|(&id, _)| id == u.id)
                        {
                            if let Some(guild_id) = command.guild_id {
                                cache.cache_borrowed_interaction_member(guild_id, member, id);
                            }
                        }
                    }

                    if cache.wants(ResourceType::ROLE) {
                        if let Some(guild_id) = command.guild_id {
                            cache.cache_roles(
                                guild_id,
                                resolved.roles.iter().map(|(_, v)| v).cloned(),
                            );
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
    use std::collections::HashMap;
    use twilight_model::{
        application::{
            command::CommandType,
            interaction::{
                application_command::{
                    CommandData, CommandInteractionDataResolved, InteractionMember,
                },
                ApplicationCommand, InteractionType,
            },
        },
        channel::{
            message::{
                sticker::{MessageSticker, StickerFormatType},
                MessageFlags, MessageType,
            },
            Message,
        },
        datetime::Timestamp,
        guild::{PartialMember, Permissions, Role},
        id::Id,
        user::User,
        util::{image_hash::ImageHashParseError, ImageHash},
    };

    #[test]
    fn test_interaction_create() -> Result<(), ImageHashParseError> {
        let timestamp = Timestamp::from_secs(1_632_072_645).expect("non zero");
        // let avatar1 = ImageHash::parse(b"1ef6bca4fddaa303a9cd32dd70fb395d")?;
        let avatar2 = ImageHash::parse(b"3a43231a99f4dfcf0fd94d1d8defd301")?;
        let avatar3 = ImageHash::parse(b"5e23c298295ad37936cfe24ad314774f")?;

        let cache = InMemoryCache::new();

        cache.update(&InteractionCreate(Interaction::ApplicationCommand(
            Box::new(ApplicationCommand {
                application_id: Id::new(1),
                channel_id: Id::new(2),
                data: CommandData {
                    id: Id::new(5),
                    name: "command name".into(),
                    kind: CommandType::ChatInput, // This isn't actually a valid command, so just mark it as a slash command.
                    options: Vec::new(),
                    resolved: Some(CommandInteractionDataResolved {
                        attachments: HashMap::new(),
                        channels: HashMap::new(),
                        members: IntoIterator::into_iter([(
                            Id::new(7),
                            InteractionMember {
                                avatar: None,
                                communication_disabled_until: None,
                                joined_at: timestamp,
                                nick: None,
                                pending: false,
                                permissions: Permissions::empty(),
                                premium_since: None,
                                roles: vec![Id::new(8)],
                            },
                        )])
                        .collect(),
                        messages: IntoIterator::into_iter([(
                            Id::new(4),
                            Message {
                                activity: None,
                                application: None,
                                application_id: None,
                                attachments: Vec::new(),
                                author: User {
                                    accent_color: None,
                                    avatar: Some(avatar3),
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
                                channel_id: Id::new(2),
                                components: Vec::new(),
                                content: "ping".to_owned(),
                                edited_timestamp: None,
                                embeds: Vec::new(),
                                flags: Some(MessageFlags::empty()),
                                guild_id: Some(Id::new(1)),
                                id: Id::new(4),
                                interaction: None,
                                kind: MessageType::Regular,
                                member: Some(PartialMember {
                                    avatar: None,
                                    communication_disabled_until: None,
                                    deaf: false,
                                    joined_at: timestamp,
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
                                    id: Id::new(1),
                                    name: "sticker name".to_owned(),
                                }],
                                referenced_message: None,
                                thread: None,
                                timestamp,
                                tts: false,
                                webhook_id: None,
                            },
                        )])
                        .collect(),
                        roles: IntoIterator::into_iter([(
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
                                tags: None,
                                unicode_emoji: None,
                            },
                        )])
                        .collect(),
                        users: IntoIterator::into_iter([(
                            Id::new(7),
                            User {
                                accent_color: None,
                                avatar: Some(avatar2),
                                banner: None,
                                bot: false,
                                discriminator: 5678,
                                email: None,
                                flags: None,
                                id: Id::new(7),
                                locale: None,
                                mfa_enabled: None,
                                name: "different name".into(),
                                premium_type: None,
                                public_flags: None,
                                system: None,
                                verified: None,
                            },
                        )])
                        .collect(),
                    }),
                    target_id: None,
                },
                guild_id: Some(Id::new(3)),
                guild_locale: None,
                id: Id::new(4),
                kind: InteractionType::ApplicationCommand,
                locale: "en-GB".to_owned(),
                member: Some(PartialMember {
                    avatar: None,
                    communication_disabled_until: None,
                    deaf: false,
                    joined_at: timestamp,
                    mute: false,
                    nick: None,
                    permissions: Some(Permissions::empty()),
                    premium_since: None,
                    roles: Vec::new(),
                    user: Some(User {
                        accent_color: None,
                        avatar: Some(avatar3),
                        banner: None,
                        bot: false,
                        discriminator: 1234,
                        email: None,
                        flags: None,
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
                token: "token".into(),
                user: None,
            }),
        )));

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
