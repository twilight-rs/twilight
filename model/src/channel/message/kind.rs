use crate::channel::ConversionError;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, Serialize_repr)]
#[repr(u8)]
pub enum MessageType {
    Regular = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelMessagePinned = 6,
    GuildMemberJoin = 7,
    UserPremiumSub = 8,
    UserPremiumSubTier1 = 9,
    UserPremiumSubTier2 = 10,
    UserPremiumSubTier3 = 11,
    ChannelFollowAdd = 12,
    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15,
    GuildDiscoveryGracePeriodInitialWarning = 16,
    GuildDiscoveryGracePeriodFinalWarning = 17,
    ThreadCreated = 18,
    /// Message is an inline reply.
    Reply = 19,
    /// Message is a chat input command.
    ChatInputCommand = 20,
    ThreadStarterMessage = 21,
    GuildInviteReminder = 22,
    ContextMenuCommand = 23,
    /// Message is an auto-moderation action.
    AutoModerationAction = 24,
}

impl TryFrom<u8> for MessageType {
    type Error = ConversionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let message_type = match value {
            0 => MessageType::Regular,
            1 => MessageType::RecipientAdd,
            2 => MessageType::RecipientRemove,
            3 => MessageType::Call,
            4 => MessageType::ChannelNameChange,
            5 => MessageType::ChannelIconChange,
            6 => MessageType::ChannelMessagePinned,
            7 => MessageType::GuildMemberJoin,
            8 => MessageType::UserPremiumSub,
            9 => MessageType::UserPremiumSubTier1,
            10 => MessageType::UserPremiumSubTier2,
            11 => MessageType::UserPremiumSubTier3,
            12 => MessageType::ChannelFollowAdd,
            14 => MessageType::GuildDiscoveryDisqualified,
            15 => MessageType::GuildDiscoveryRequalified,
            16 => MessageType::GuildDiscoveryGracePeriodInitialWarning,
            17 => MessageType::GuildDiscoveryGracePeriodFinalWarning,
            18 => MessageType::ThreadCreated,
            19 => MessageType::Reply,
            20 => MessageType::ChatInputCommand,
            21 => MessageType::ThreadStarterMessage,
            22 => MessageType::GuildInviteReminder,
            23 => MessageType::ContextMenuCommand,
            24 => MessageType::AutoModerationAction,
            _ => return Err(ConversionError::MessageType(value)),
        };

        Ok(message_type)
    }
}

#[cfg(test)]
mod tests {
    use super::{ConversionError, MessageType};
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
    }

    #[test]
    fn conversions() {
        assert_eq!(MessageType::try_from(0).unwrap(), MessageType::Regular);
        assert_eq!(MessageType::try_from(1).unwrap(), MessageType::RecipientAdd);
        assert_eq!(
            MessageType::try_from(2).unwrap(),
            MessageType::RecipientRemove
        );
        assert_eq!(MessageType::try_from(3).unwrap(), MessageType::Call);
        assert_eq!(
            MessageType::try_from(4).unwrap(),
            MessageType::ChannelNameChange
        );
        assert_eq!(
            MessageType::try_from(5).unwrap(),
            MessageType::ChannelIconChange
        );
        assert_eq!(
            MessageType::try_from(6).unwrap(),
            MessageType::ChannelMessagePinned
        );
        assert_eq!(
            MessageType::try_from(7).unwrap(),
            MessageType::GuildMemberJoin
        );
        assert_eq!(
            MessageType::try_from(8).unwrap(),
            MessageType::UserPremiumSub
        );
        assert_eq!(
            MessageType::try_from(9).unwrap(),
            MessageType::UserPremiumSubTier1
        );
        assert_eq!(
            MessageType::try_from(10).unwrap(),
            MessageType::UserPremiumSubTier2
        );
        assert_eq!(
            MessageType::try_from(11).unwrap(),
            MessageType::UserPremiumSubTier3
        );
        assert_eq!(
            MessageType::try_from(12).unwrap(),
            MessageType::ChannelFollowAdd
        );
        assert_eq!(
            MessageType::try_from(14).unwrap(),
            MessageType::GuildDiscoveryDisqualified
        );
        assert_eq!(
            MessageType::try_from(15).unwrap(),
            MessageType::GuildDiscoveryRequalified
        );
        assert_eq!(
            MessageType::try_from(16).unwrap(),
            MessageType::GuildDiscoveryGracePeriodInitialWarning
        );
        assert_eq!(
            MessageType::try_from(17).unwrap(),
            MessageType::GuildDiscoveryGracePeriodFinalWarning
        );
        assert_eq!(
            MessageType::try_from(18).unwrap(),
            MessageType::ThreadCreated
        );
        assert_eq!(MessageType::try_from(19).unwrap(), MessageType::Reply);
        assert_eq!(
            MessageType::try_from(20).unwrap(),
            MessageType::ChatInputCommand
        );
        assert_eq!(
            MessageType::try_from(21).unwrap(),
            MessageType::ThreadStarterMessage
        );
        assert_eq!(
            MessageType::try_from(22).unwrap(),
            MessageType::GuildInviteReminder
        );
        assert_eq!(
            MessageType::try_from(23).unwrap(),
            MessageType::ContextMenuCommand
        );
        assert_eq!(
            MessageType::try_from(24).unwrap(),
            MessageType::AutoModerationAction
        );
        assert_eq!(
            MessageType::try_from(250).unwrap_err(),
            ConversionError::MessageType(250)
        );
    }
}
