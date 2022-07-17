use serde::{Deserialize, Serialize};

/// Type of a [`Message`].
///
/// [`Message`]: super::Message
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
    GuildMemberJoin,
    /// System message denoting a user nitro boosted a guild.
    UserPremiumSub,
    /// System message denoting a user nitro boosted a guild to level 1.
    UserPremiumSubTier1,
    /// System message denoting a user nitro boosted a guild to level 2.
    UserPremiumSubTier2,
    /// System message denoting a user nitro boosted a guild to level 3.
    UserPremiumSubTier3,
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
            7 => Self::GuildMemberJoin,
            8 => Self::UserPremiumSub,
            9 => Self::UserPremiumSubTier1,
            10 => Self::UserPremiumSubTier2,
            11 => Self::UserPremiumSubTier3,
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
            MessageType::GuildMemberJoin => 7,
            MessageType::UserPremiumSub => 8,
            MessageType::UserPremiumSubTier1 => 9,
            MessageType::UserPremiumSubTier2 => 10,
            MessageType::UserPremiumSubTier3 => 11,
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
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(&MessageType::Regular, &[Token::U8(0)]);
        serde_test::assert_tokens(&MessageType::RecipientAdd, &[Token::U8(1)]);
        serde_test::assert_tokens(&MessageType::RecipientRemove, &[Token::U8(2)]);
        serde_test::assert_tokens(&MessageType::Call, &[Token::U8(3)]);
        serde_test::assert_tokens(&MessageType::ChannelNameChange, &[Token::U8(4)]);
        serde_test::assert_tokens(&MessageType::ChannelIconChange, &[Token::U8(5)]);
        serde_test::assert_tokens(&MessageType::ChannelMessagePinned, &[Token::U8(6)]);
        serde_test::assert_tokens(&MessageType::GuildMemberJoin, &[Token::U8(7)]);
        serde_test::assert_tokens(&MessageType::UserPremiumSub, &[Token::U8(8)]);
        serde_test::assert_tokens(&MessageType::UserPremiumSubTier1, &[Token::U8(9)]);
        serde_test::assert_tokens(&MessageType::UserPremiumSubTier2, &[Token::U8(10)]);
        serde_test::assert_tokens(&MessageType::UserPremiumSubTier3, &[Token::U8(11)]);
        serde_test::assert_tokens(&MessageType::ChannelFollowAdd, &[Token::U8(12)]);
        serde_test::assert_tokens(&MessageType::GuildDiscoveryDisqualified, &[Token::U8(14)]);
        serde_test::assert_tokens(&MessageType::GuildDiscoveryRequalified, &[Token::U8(15)]);
        serde_test::assert_tokens(
            &MessageType::GuildDiscoveryGracePeriodInitialWarning,
            &[Token::U8(16)],
        );
        serde_test::assert_tokens(
            &MessageType::GuildDiscoveryGracePeriodFinalWarning,
            &[Token::U8(17)],
        );
        serde_test::assert_tokens(&MessageType::ThreadCreated, &[Token::U8(18)]);
        serde_test::assert_tokens(&MessageType::Reply, &[Token::U8(19)]);
        serde_test::assert_tokens(&MessageType::ChatInputCommand, &[Token::U8(20)]);
        serde_test::assert_tokens(&MessageType::ThreadStarterMessage, &[Token::U8(21)]);
        serde_test::assert_tokens(&MessageType::GuildInviteReminder, &[Token::U8(22)]);
        serde_test::assert_tokens(&MessageType::ContextMenuCommand, &[Token::U8(23)]);
        serde_test::assert_tokens(&MessageType::AutoModerationAction, &[Token::U8(24)]);
        serde_test::assert_tokens(&MessageType::Unknown(99), &[Token::U8(99)]);
    }

    #[test]
    fn conversions() {
        assert_eq!(MessageType::from(0), MessageType::Regular);
        assert_eq!(MessageType::from(1), MessageType::RecipientAdd);
        assert_eq!(MessageType::from(2), MessageType::RecipientRemove);
        assert_eq!(MessageType::from(3), MessageType::Call);
        assert_eq!(MessageType::from(4), MessageType::ChannelNameChange);
        assert_eq!(MessageType::from(5), MessageType::ChannelIconChange);
        assert_eq!(MessageType::from(6), MessageType::ChannelMessagePinned);
        assert_eq!(MessageType::from(7), MessageType::GuildMemberJoin);
        assert_eq!(MessageType::from(8), MessageType::UserPremiumSub);
        assert_eq!(MessageType::from(9), MessageType::UserPremiumSubTier1);
        assert_eq!(MessageType::from(10), MessageType::UserPremiumSubTier2);
        assert_eq!(MessageType::from(11), MessageType::UserPremiumSubTier3);
        assert_eq!(MessageType::from(12), MessageType::ChannelFollowAdd);
        assert_eq!(
            MessageType::from(14),
            MessageType::GuildDiscoveryDisqualified
        );
        assert_eq!(
            MessageType::from(15),
            MessageType::GuildDiscoveryRequalified
        );
        assert_eq!(
            MessageType::from(16),
            MessageType::GuildDiscoveryGracePeriodInitialWarning
        );
        assert_eq!(
            MessageType::from(17),
            MessageType::GuildDiscoveryGracePeriodFinalWarning
        );
        assert_eq!(MessageType::from(18), MessageType::ThreadCreated);
        assert_eq!(MessageType::from(19), MessageType::Reply);
        assert_eq!(MessageType::from(20), MessageType::ChatInputCommand);
        assert_eq!(MessageType::from(21), MessageType::ThreadStarterMessage);
        assert_eq!(MessageType::from(22), MessageType::GuildInviteReminder);
        assert_eq!(MessageType::from(23), MessageType::ContextMenuCommand);
        assert_eq!(MessageType::from(24), MessageType::AutoModerationAction);
        assert_eq!(MessageType::from(250), MessageType::Unknown(250));
    }
}
