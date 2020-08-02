use crate::channel::ConversionError;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::convert::TryFrom;

#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize_repr,
)]
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
            _ => return Err(ConversionError::MessageType(value)),
        };

        Ok(message_type)
    }
}
