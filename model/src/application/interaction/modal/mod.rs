mod data;

pub use self::data::{
    ModalComponentValue, ModalInteractionData, ModalInteractionDataActionRow,
    ModalInteractionDataComponent,
};

use super::InteractionType;
use crate::{
    guild::PartialMember,
    id::{ApplicationId, ChannelId, GuildId, InteractionId, UserId},
    user::User,
};
use serde::Serialize;

/// Information present in an [`Interaction::ModalSubmit`].
///
/// [`Interaction::ModalSubmit`]: super::Interaction::ModalSubmit
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename(serialize = "Interaction"))]
pub struct ModalSubmitInteraction {
    /// ID of the associated application.
    pub application_id: ApplicationId,
    /// ID of the channel the interaction was triggered from.
    pub channel_id: ChannelId,
    /// Data from the submitted modal.
    pub data: ModalInteractionData,
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
    /// Token of the interaction.
    pub token: String,
    /// User that triggered the interaction.
    ///
    /// Present when the command is used in a direct message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}

impl ModalSubmitInteraction {
    /// ID of the user that submitted the modal.
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
    use serde::Serialize;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, str::FromStr};

    use crate::{
        application::interaction::{modal::data::ModalInteractionDataActionRow, InteractionType},
        datetime::Timestamp,
        guild::PartialMember,
        id::{ApplicationId, ChannelId, GuildId, InteractionId, UserId},
        user::User,
    };

    use super::{
        data::{ModalComponentValue, ModalInteractionData, ModalInteractionDataComponent},
        ModalSubmitInteraction,
    };

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
        Serialize,
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
        fn user_id() -> UserId {
            UserId::new(7).expect("non zero")
        }

        let in_guild = ModalSubmitInteraction {
            application_id: ApplicationId::new(1).expect("non zero"),
            channel_id: ChannelId::new(1).expect("non zero"),
            data: ModalInteractionData {
                custom_id: "the-id".to_owned(),
                components: Vec::from([ModalInteractionDataActionRow {
                    components: Vec::from([ModalInteractionDataComponent {
                        custom_id: "input-1".to_owned(),
                        value: ModalComponentValue::InputText("got it".to_owned()),
                    }]),
                }]),
            },
            guild_id: Some(GuildId::new(1).expect("non zero")),
            id: InteractionId::new(1).expect("non zero"),
            kind: InteractionType::ModalSubmit,
            member: Some(PartialMember {
                avatar: None,
                deaf: false,
                joined_at: Timestamp::from_str("2020-02-02T02:02:02.020000+00:00")
                    .expect("invalid timestamp"),
                mute: false,
                nick: None,
                permissions: None,
                premium_since: None,
                roles: Vec::new(),
                user: Some(user(user_id())),
                communication_disabled_until: None,
            }),
            token: "TOKEN".to_owned(),
            user: None,
        };

        assert_eq!(Some(user_id()), in_guild.author_id());

        let in_dm = ModalSubmitInteraction {
            member: None,
            user: Some(user(user_id())),
            ..in_guild
        };
        assert_eq!(Some(user_id()), in_dm.author_id());
    }
}
