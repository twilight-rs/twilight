mod data;

pub use self::data::{
    ModalInteractionData, ModalInteractionDataActionRow, ModalInteractionDataComponent,
};

use crate::{
    application::interaction::InteractionType,
    channel::Message,
    guild::{PartialMember, Permissions},
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, InteractionMarker, UserMarker},
        Id,
    },
    user::User,
};
use serde::Serialize;

/// Information present in an [`Interaction::ModalSubmit`].
///
/// [`Interaction::ModalSubmit`]: super::Interaction::ModalSubmit
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename(serialize = "Interaction"))]
pub struct ModalSubmitInteraction {
    /// Permissions the app or bot has within the channel the
    /// interaction was sent from.
    pub app_permissions: Option<Permissions>,
    /// ID of the associated application.
    pub application_id: Id<ApplicationMarker>,
    /// ID of the channel the interaction was invoked in.
    pub channel_id: Id<ChannelMarker>,
    /// Data from the submitted modal.
    pub data: ModalInteractionData,
    /// ID of the guild the interaction was invoked in.
    pub guild_id: Option<Id<GuildMarker>>,
    /// Guild's preferred locale.
    ///
    /// Present when the command is used in a guild.
    ///
    /// Defaults to `en-US`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_locale: Option<String>,
    /// ID of the interaction.
    pub id: Id<InteractionMarker>,
    /// Type of the interaction.
    ///
    /// Should always be `InteractionType::ModalSubmit`.
    #[serde(rename = "type")]
    pub kind: InteractionType,
    /// Selected language of the user who invoked the interaction.
    pub locale: String,
    /// Member that invoked the interaction.
    ///
    /// Present when the command is used in a guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<PartialMember>,
    /// Message object, if the modal comes from a message component interaction.
    ///
    /// This is currently *not* validated by the Discord API and may be spoofed
    /// by malicious users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    /// Token of the interaction.
    pub token: String,
    /// User that invoked the interaction.
    ///
    /// Present when the command is used in a direct message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

impl ModalSubmitInteraction {
    /// ID of the user that invoked the interaction.
    ///
    /// This will first check for the [`member`]'s
    /// [`user`][`PartialMember::user`]'s ID and, if not present, then check the
    /// [`user`]'s ID.
    ///
    /// [`member`]: Self::member
    /// [`user`]: Self::user
    pub const fn author_id(&self) -> Option<Id<UserMarker>> {
        super::author_id(self.user.as_ref(), self.member.as_ref())
    }

    /// Whether the interaction was invoked in a DM.
    pub const fn is_dm(&self) -> bool {
        self.user.is_some()
    }

    /// Whether the interaction was invoked in a guild.
    pub const fn is_guild(&self) -> bool {
        self.member.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ModalInteractionData, ModalInteractionDataActionRow, ModalInteractionDataComponent,
        ModalSubmitInteraction,
    };
    use crate::{
        application::{
            component::ComponentType,
            interaction::{tests::user, InteractionType},
        },
        guild::{PartialMember, Permissions},
        id::{
            marker::{
                ApplicationMarker, ChannelMarker, GuildMarker, InteractionMarker, UserMarker,
            },
            Id,
        },
        util::datetime::{Timestamp, TimestampParseError},
    };
    use serde::Serialize;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, str::FromStr};

    assert_fields!(
        ModalSubmitInteraction: application_id,
        channel_id,
        data,
        guild_id,
        id,
        kind,
        member,
        token,
        user
    );
    assert_impl_all!(
        ModalSubmitInteraction: Clone,
        Debug,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    const USER_ID: Id<UserMarker> = Id::new(7);

    #[test]
    fn author_id() -> Result<(), TimestampParseError> {
        let joined_at = Timestamp::from_str("2020-02-02T02:02:02.020000+00:00")?;

        let in_guild = ModalSubmitInteraction {
            app_permissions: Some(Permissions::SEND_MESSAGES),
            application_id: Id::<ApplicationMarker>::new(1),
            channel_id: Id::<ChannelMarker>::new(1),
            data: ModalInteractionData {
                custom_id: "the-id".to_owned(),
                components: Vec::from([ModalInteractionDataActionRow {
                    components: Vec::from([ModalInteractionDataComponent {
                        custom_id: "input-1".to_owned(),
                        kind: ComponentType::TextInput,
                        value: "got it".into(),
                    }]),
                }]),
            },
            guild_id: Some(Id::<GuildMarker>::new(1)),
            guild_locale: Some("de".to_owned()),
            id: Id::<InteractionMarker>::new(1),
            kind: InteractionType::ModalSubmit,
            locale: "en-GB".to_owned(),
            member: Some(PartialMember {
                avatar: None,
                deaf: false,
                joined_at,
                mute: false,
                nick: None,
                permissions: None,
                premium_since: None,
                roles: Vec::new(),
                user: Some(user(USER_ID)),
                communication_disabled_until: None,
            }),
            message: None,
            token: "TOKEN".to_owned(),
            user: None,
        };

        assert_eq!(Some(USER_ID), in_guild.author_id());
        assert!(in_guild.is_guild());

        let in_dm = ModalSubmitInteraction {
            member: None,
            user: Some(user(USER_ID)),
            ..in_guild
        };
        assert_eq!(Some(USER_ID), in_dm.author_id());
        assert!(in_dm.is_dm());

        Ok(())
    }
}
