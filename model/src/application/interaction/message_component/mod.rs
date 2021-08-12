mod data;

pub use self::data::MessageComponentInteractionData;

use super::InteractionType;
use crate::{
    channel::Message,
    guild::PartialMember,
    id::{ApplicationId, ChannelId, GuildId, InteractionId, UserId},
    user::User,
};
use serde::Serialize;

/// Information present in an [`Interaction::MessageComponent`].
///
/// [`Interaction::MessageComponent`]: super::Interaction::MessageComponent
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(rename(serialize = "Interaction"))]
pub struct MessageComponentInteraction {
    /// ID of the associated application.
    pub application_id: ApplicationId,
    /// ID of the channel the interaction was triggered from.
    pub channel_id: ChannelId,
    /// Data from the invoked command.
    pub data: MessageComponentInteractionData,
    /// ID of the guild the interaction was triggered from.
    pub guild_id: Option<GuildId>,
    /// ID of the interaction.
    pub id: InteractionId,
    /// Type of the interaction.
    #[serde(rename = "type")]
    pub kind: InteractionType,
    /// Member that triggered the interaction.
    ///
    /// Present when the command is used in a guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<PartialMember>,
    /// Message object for the message this button belongs to.
    ///
    /// This is currently *not* validated by the discord API and may be spoofed
    /// by malicious users.
    pub message: Message,
    /// Token of the interaction.
    pub token: String,
    /// User that triggered the interaction.
    ///
    /// Present when the command is used in a direct message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

impl MessageComponentInteraction {
    /// ID of the user that invoked the interaction.
    ///
    /// This will first check for the [`member`]'s
    /// [`user`][`PartialMember::user`]'s ID and, if not present, then check the
    /// [`user`]'s ID.
    ///
    /// [`member`]: Self::member
    /// [`user`]: Self::user
    pub const fn author_id(&self) -> Option<UserId> {
        if let Some(member) = &self.member {
            if let Some(user) = &member.user {
                return Some(user.id);
            }
        }

        if let Some(user) = &self.user {
            return Some(user.id);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{MessageComponentInteraction, MessageComponentInteractionData};
    use crate::{
        application::{component::ComponentType, interaction::InteractionType},
        channel::message::{Message, MessageType},
        guild::PartialMember,
        id::{ApplicationId, ChannelId, GuildId, InteractionId, MessageId, UserId},
        user::User,
    };
    use serde::Serialize;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        MessageComponentInteraction: application_id,
        channel_id,
        data,
        guild_id,
        id,
        kind,
        member,
        message,
        token,
        user
    );
    assert_impl_all!(
        MessageComponentInteraction: Clone,
        Debug,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    fn user(id: UserId) -> User {
        User {
            accent_color: None,
            avatar: None,
            banner: None,
            bot: false,
            discriminator: 4444,
            email: None,
            flags: None,
            id,
            locale: None,
            mfa_enabled: None,
            name: "twilight".to_owned(),
            premium_type: None,
            public_flags: None,
            system: None,
            verified: None,
        }
    }

    #[test]
    fn test_author_id() {
        const USER_ID: UserId = UserId(7);

        let in_guild = MessageComponentInteraction {
            application_id: ApplicationId(1),
            channel_id: ChannelId(2),
            data: MessageComponentInteractionData {
                custom_id: "foo".to_owned(),
                component_type: ComponentType::Button,
                values: Vec::from(["bar".to_owned()]),
            },
            guild_id: Some(GuildId(3)),
            id: InteractionId(4),
            kind: InteractionType::MessageComponent,
            member: Some(PartialMember {
                deaf: false,
                joined_at: None,
                mute: false,
                nick: None,
                permissions: None,
                premium_since: None,
                roles: Vec::new(),
                user: Some(user(USER_ID)),
            }),
            message: Message {
                activity: None,
                application: None,
                application_id: None,
                attachments: Vec::new(),
                author: user(USER_ID),
                channel_id: ChannelId(5),
                components: Vec::new(),
                content: String::new(),
                edited_timestamp: None,
                embeds: Vec::new(),
                flags: None,
                guild_id: Some(GuildId(3)),
                id: MessageId(6),
                interaction: None,
                kind: MessageType::Regular,
                member: None,
                mention_channels: Vec::new(),
                mention_everyone: false,
                mention_roles: Vec::new(),
                mentions: Vec::new(),
                pinned: false,
                reactions: Vec::new(),
                reference: None,
                referenced_message: None,
                sticker_items: Vec::new(),
                timestamp: String::new(),
                thread: None,
                tts: false,
                webhook_id: None,
            },
            token: String::new(),
            user: None,
        };

        assert_eq!(Some(USER_ID), in_guild.author_id());

        let in_dm = MessageComponentInteraction {
            member: None,
            message: Message {
                guild_id: None,
                ..in_guild.message
            },
            user: Some(user(USER_ID)),
            ..in_guild
        };
        assert_eq!(Some(USER_ID), in_dm.author_id());
    }
}
