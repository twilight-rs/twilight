use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize_repr)]
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
}
