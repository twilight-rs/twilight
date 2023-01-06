use crate::guild::Permissions;
use serde::{Deserialize, Serialize};

/// Type of a [`Message`].
///
/// Refer to [Discord Docs/Message Types] for more information.
///
/// [Discord Docs/Message Types]: https://discord.com/developers/docs/resources/channel#message-object-message-types
/// [`Message`]: super::Message
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum MessageType {
    /// Regular message.
    Regular,
    /// System message denoting a recipient has been added to a group.
    RecipientAdd,
    /// System message denoting a recipient has been removed from a group.
    RecipientRemove,
    /// System message denoting a call state, e.g. missed, started.
    Call,
    /// System message denoting a channel's name has been changed.
    ChannelNameChange,
    /// System message denoting a channel's icon has been changed.
    ChannelIconChange,
    /// System message denoting a message has been pinned.
    ChannelMessagePinned,
    /// System message denoting a member has joined a guild.
    UserJoin,
    /// System message denoting a user nitro boosted a guild.
    GuildBoost,
    /// System message denoting a user nitro boosted a guild to level 1.
    GuildBoostTier1,
    /// System message denoting a user nitro boosted a guild to level 2.
    GuildBoostTier2,
    /// System message denoting a user nitro boosted a guild to level 3.
    GuildBoostTier3,
    /// System message denoting a channel has been followed.
    ChannelFollowAdd,
    /// System message denoting a guild has been disqualified for Server Discovery.
    GuildDiscoveryDisqualified,
    /// System message denoting a guild has been redisqualified for Server Discovery.
    GuildDiscoveryRequalified,
    /// System message denoting an initial warning for Server Discovery disqualification.
    GuildDiscoveryGracePeriodInitialWarning,
    /// System message denoting a final warning for Server Discovery disqualification.
    GuildDiscoveryGracePeriodFinalWarning,
    ThreadCreated,
    /// Message is an inline reply.
    Reply,
    /// Message is a chat input command.
    ChatInputCommand,
    ThreadStarterMessage,
    GuildInviteReminder,
    ContextMenuCommand,
    /// Message is an auto moderation action.
    AutoModerationAction,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl MessageType {
    /// Whether the message can be deleted, not taking permissions into account.
    /// Some message types can't be deleted, even by server administrators.
    ///
    /// Some message types can only be deleted with certain permissions. For
    /// example, [`AutoModerationAction`][`Self::AutoModerationAction`] can only
    /// be deleted if the user has the
    /// [Manage Messages] permission.
    ///
    /// To check whether a message can be deleted while taking permissions into
    /// account, use
    /// [`deletable_with_permissions`][`Self::deletable_with_permissions`].
    ///
    /// [Manage Messages]: Permissions::MANAGE_MESSAGES
    pub const fn deletable(self) -> bool {
        matches!(
            self,
            Self::Regular
                | Self::ChannelMessagePinned
                | Self::UserJoin
                | Self::GuildBoost
                | Self::GuildBoostTier1
                | Self::GuildBoostTier2
                | Self::GuildBoostTier3
                | Self::ChannelFollowAdd
                | Self::ThreadCreated
                | Self::Reply
                | Self::ChatInputCommand
                | Self::GuildInviteReminder
                | Self::ContextMenuCommand
                | Self::AutoModerationAction
        )
    }

    /// Whether the message can be deleted, taking permissions into account.
    /// Some message types can't be deleted, even by server administrators.
    ///
    /// Some message types can only be deleted with certain permissions. For
    /// example, [`AutoModerationAction`][`Self::AutoModerationAction`] can only
    /// be deleted if the user has the [Manage Messages] permission.
    ///
    /// To check whether a message can be deleted *without* taking permissions
    /// into account, use [`deletable`][`Self::deletable`].
    ///
    /// [Manage Messages]: Permissions::MANAGE_MESSAGES
    pub const fn deletable_with_permissions(self, permissions: Permissions) -> bool {
        let required_permissions = match self {
            Self::AutoModerationAction => Permissions::MANAGE_MESSAGES,
            _ => Permissions::empty(),
        };

        if !permissions.contains(required_permissions) {
            return false;
        }

        self.deletable()
    }
}

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Regular,
            1 => Self::RecipientAdd,
            2 => Self::RecipientRemove,
            3 => Self::Call,
            4 => Self::ChannelNameChange,
            5 => Self::ChannelIconChange,
            6 => Self::ChannelMessagePinned,
            7 => Self::UserJoin,
            8 => Self::GuildBoost,
            9 => Self::GuildBoostTier1,
            10 => Self::GuildBoostTier2,
            11 => Self::GuildBoostTier3,
            12 => Self::ChannelFollowAdd,
            14 => Self::GuildDiscoveryDisqualified,
            15 => Self::GuildDiscoveryRequalified,
            16 => Self::GuildDiscoveryGracePeriodInitialWarning,
            17 => Self::GuildDiscoveryGracePeriodFinalWarning,
            18 => Self::ThreadCreated,
            19 => Self::Reply,
            20 => Self::ChatInputCommand,
            21 => Self::ThreadStarterMessage,
            22 => Self::GuildInviteReminder,
            23 => Self::ContextMenuCommand,
            24 => Self::AutoModerationAction,
            unknown => Self::Unknown(unknown),
        }
    }
}

impl From<MessageType> for u8 {
    fn from(value: MessageType) -> Self {
        match value {
            MessageType::Regular => 0,
            MessageType::RecipientAdd => 1,
            MessageType::RecipientRemove => 2,
            MessageType::Call => 3,
            MessageType::ChannelNameChange => 4,
            MessageType::ChannelIconChange => 5,
            MessageType::ChannelMessagePinned => 6,
            MessageType::UserJoin => 7,
            MessageType::GuildBoost => 8,
            MessageType::GuildBoostTier1 => 9,
            MessageType::GuildBoostTier2 => 10,
            MessageType::GuildBoostTier3 => 11,
            MessageType::ChannelFollowAdd => 12,
            MessageType::GuildDiscoveryDisqualified => 14,
            MessageType::GuildDiscoveryRequalified => 15,
            MessageType::GuildDiscoveryGracePeriodInitialWarning => 16,
            MessageType::GuildDiscoveryGracePeriodFinalWarning => 17,
            MessageType::ThreadCreated => 18,
            MessageType::Reply => 19,
            MessageType::ChatInputCommand => 20,
            MessageType::ThreadStarterMessage => 21,
            MessageType::GuildInviteReminder => 22,
            MessageType::ContextMenuCommand => 23,
            MessageType::AutoModerationAction => 24,
            MessageType::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MessageType;
    use crate::guild::Permissions;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        MessageType: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    #[test]
    fn message_type() {
        const MAP: &[(MessageType, u8, bool)] = &[
            (MessageType::Regular, 0, true),
            (MessageType::RecipientAdd, 1, false),
            (MessageType::RecipientRemove, 2, false),
            (MessageType::Call, 3, false),
            (MessageType::ChannelNameChange, 4, false),
            (MessageType::ChannelIconChange, 5, false),
            (MessageType::ChannelMessagePinned, 6, true),
            (MessageType::UserJoin, 7, true),
            (MessageType::GuildBoost, 8, true),
            (MessageType::GuildBoostTier1, 9, true),
            (MessageType::GuildBoostTier2, 10, true),
            (MessageType::GuildBoostTier3, 11, true),
            (MessageType::ChannelFollowAdd, 12, true),
            (MessageType::GuildDiscoveryDisqualified, 14, false),
            (MessageType::GuildDiscoveryRequalified, 15, false),
            (
                MessageType::GuildDiscoveryGracePeriodInitialWarning,
                16,
                false,
            ),
            (
                MessageType::GuildDiscoveryGracePeriodFinalWarning,
                17,
                false,
            ),
            (MessageType::ThreadCreated, 18, true),
            (MessageType::Reply, 19, true),
            (MessageType::ChatInputCommand, 20, true),
            (MessageType::ThreadStarterMessage, 21, false),
            (MessageType::GuildInviteReminder, 22, true),
            (MessageType::ContextMenuCommand, 23, true),
            (MessageType::AutoModerationAction, 24, true),
        ];

        for (message_type, number, deletable) in MAP {
            assert_eq!(*message_type, MessageType::from(*number));
            assert_eq!(*number, u8::from(*message_type));
            assert_eq!(*deletable, message_type.deletable());
            serde_test::assert_tokens(message_type, &[Token::U8(*number)]);
        }
    }

    #[test]
    fn deletable_with_permissions() {
        assert!(MessageType::AutoModerationAction
            .deletable_with_permissions(Permissions::MANAGE_MESSAGES));
        assert!(!MessageType::AutoModerationAction.deletable_with_permissions(Permissions::empty()));
    }
}
