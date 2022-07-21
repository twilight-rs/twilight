use bitflags::bitflags;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};

bitflags! {
    pub struct UserFlags: u64 {
        /// Discord Employee.
        const STAFF = 1;
        /// Partnered server owner.
        const PARTNER = 1 << 1;
        /// HypeSquad events member.
        const HYPESQUAD = 1 << 2;
        /// Bug hunter level 1.
        const BUG_HUNTER_LEVEL_1 = 1 << 3;
        /// House Bravery member.
        const HYPESQUAD_ONLINE_HOUSE_1 = 1 << 6;
        /// House Brilliance member.
        const HYPESQUAD_ONLINE_HOUSE_2 = 1 << 7;
        /// House Balance member.
        const HYPESQUAD_ONLINE_HOUSE_3 = 1 << 8;
        /// Early Nitro supporter.
        const PREMIUM_EARLY_SUPPORTER = 1 << 9;
        /// User is in a team.
        const TEAM_PSEUDO_USER = 1 << 10;
        /// Bug hunter level 2.
        const BUG_HUNTER_LEVEL_2 = 1 << 14;
        /// Verified bot.
        const VERIFIED_BOT = 1 << 16;
        /// Early verified bot developer.
        const VERIFIED_DEVELOPER = 1 << 17;
        /// Discord certified moderator.
        const CERTIFIED_MODERATOR = 1 << 18;
        /// Bot uses only HTTP interactions and is shown in the online member
        /// list.
        const BOT_HTTP_INTERACTIONS = 1 << 19;
    }
}

impl<'de> Deserialize<'de> for UserFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

impl Serialize for UserFlags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.bits())
    }
}
