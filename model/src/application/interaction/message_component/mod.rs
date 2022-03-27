mod data;

pub use self::data::MessageComponentInteractionData;

use super::InteractionType;
use crate::{
    channel::Message,
    guild::PartialMember,
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, InteractionMarker, UserMarker},
        Id,
    },
    user::User,
};
use serde::{ser::SerializeStruct, Serialize};

/// Information present in an [`Interaction::MessageComponent`].
///
/// [`Interaction::MessageComponent`]: super::Interaction::MessageComponent
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MessageComponentInteraction {
    /// ID of the associated application.
    pub application_id: Id<ApplicationMarker>,
    /// ID of the channel the interaction was triggered from.
    pub channel_id: Id<ChannelMarker>,
    /// Data from the invoked command.
    pub data: MessageComponentInteractionData,
    /// ID of the guild the interaction was triggered from.
    pub guild_id: Option<Id<GuildMarker>>,
    /// Guild's preferred locale.
    ///
    /// Present when the command is used in a guild.
    ///
    /// Defaults to `en-US`.
    pub guild_locale: Option<String>,
    /// ID of the interaction.
    pub id: Id<InteractionMarker>,
    /// Selected language of the user who triggered the interaction.
    pub locale: String,
    /// Member that triggered the interaction.
    ///
    /// Present when the command is used in a guild.
    pub member: Option<PartialMember>,
    /// Message object for the message this button belongs to.
    ///
    /// This is currently *not* validated by the Discord API and may be spoofed
    /// by malicious users.
    pub message: Message,
    /// Token of the interaction.
    pub token: String,
    /// User that triggered the interaction.
    ///
    /// Present when the command is used in a direct message.
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
    pub const fn author_id(&self) -> Option<Id<UserMarker>> {
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

impl Serialize for MessageComponentInteraction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let len = 8
            + usize::from(self.guild_id.is_some())
            + usize::from(self.guild_locale.is_some())
            + usize::from(self.member.is_some())
            + usize::from(self.user.is_some());

        let mut state = serializer.serialize_struct("Interaction", len)?;
        state.serialize_field("application_id", &self.application_id)?;
        state.serialize_field("channel_id", &self.channel_id)?;
        state.serialize_field("data", &self.data)?;
        if let Some(guild_id) = self.guild_id {
            state.serialize_field("guild_id", &guild_id)?;
        }
        if let Some(guild_locale) = &self.guild_locale {
            state.serialize_field("guild_locale", &guild_locale)?;
        }
        state.serialize_field("id", &self.id)?;
        state.serialize_field("type", &InteractionType::MessageComponent)?;
        state.serialize_field("locale", &self.locale)?;
        if let Some(member) = &self.member {
            state.serialize_field("member", &member)?;
        }
        state.serialize_field("message", &self.message)?;
        state.serialize_field("token", &self.token)?;
        if let Some(user) = &self.user {
            state.serialize_field("user", &user)?;
        }
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::{MessageComponentInteraction, MessageComponentInteractionData};
    use crate::{
        application::component::ComponentType,
        channel::message::{Message, MessageType},
        datetime::{Timestamp, TimestampParseError},
        guild::PartialMember,
        id::{marker::UserMarker, Id},
        user::User,
    };
    use serde::Serialize;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash, str::FromStr};

    assert_fields!(
        MessageComponentInteraction: application_id,
        channel_id,
        data,
        guild_id,
        id,
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

    fn user(id: Id<UserMarker>) -> User {
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
    fn test_author_id() -> Result<(), TimestampParseError> {
        const USER_ID: Id<UserMarker> = Id::new(7);

        let timestamp = Timestamp::from_str("2020-02-02T02:02:02.020000+00:00")?;

        let in_guild = MessageComponentInteraction {
            application_id: Id::new(1),
            channel_id: Id::new(2),
            data: MessageComponentInteractionData {
                custom_id: "foo".to_owned(),
                component_type: ComponentType::Button,
                values: Vec::from(["bar".to_owned()]),
            },
            guild_id: Some(Id::new(3)),
            guild_locale: None,
            id: Id::new(4),
            locale: "en-GB".to_owned(),
            member: Some(PartialMember {
                avatar: None,
                communication_disabled_until: None,
                deaf: false,
                joined_at: timestamp,
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
                channel_id: Id::new(5),
                components: Vec::new(),
                content: String::new(),
                edited_timestamp: None,
                embeds: Vec::new(),
                flags: None,
                guild_id: Some(Id::new(3)),
                id: Id::new(6),
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
                timestamp,
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

        Ok(())
    }
}
