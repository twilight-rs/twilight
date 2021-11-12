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
        /// HypeSquad events coordinator.
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
        #[deprecated(note = "the name has changed to `STAFF`", since = "0.7.2")]
        const DISCORD_EMPLOYEE = 1;
        #[deprecated(note = "the name has changed to `PARTNER`", since = "0.7.2")]
        const DISCORD_PARTNER = 1 << 1;
        #[deprecated(note = "the name has changed to `HYPESQUAD`", since = "0.7.2")]
        const HYPESQUAD_EVENTS = 1 << 2;
        #[deprecated(note = "the name has changed to `BUG_HUNTER_LEVEL_1`", since = "0.7.2")]
        const BUG_HUNTER = 1 << 3;
        #[deprecated(note = "the name has changed to `HYPESQUAD_ONLINE_HOUSE_1`", since = "0.7.2")]
        const HOUSE_BRAVERY = 1 << 6;
        #[deprecated(note = "the name has changed to `HYPESQUAD_ONLINE_HOUSE_2`", since = "0.7.2")]
        const HOUSE_BRILLIANCE = 1 << 7;
        #[deprecated(note = "the name has changed to `HYPESQUAD_ONLINE_HOUSE_3`", since = "0.7.2")]
        const HOUSE_BALANCE = 1 << 8;
        #[deprecated(note = "the name has changed to `PREMIUM_EARLY_SUPPORTER`", since = "0.7.2")]
        const EARLY_SUPPORTER = 1 << 9;
        #[deprecated(note = "the name has changed to `TEAM_PSEUDO_USER`", since = "0.7.2")]
        const TEAM_USER = 1 << 10;
        #[deprecated(note = "the name has changed to `VERIFIED_DEVELOPER`", since = "0.7.2")]
        const VERIFIED_BOT_DEVELOPER = 1 << 17;
        #[deprecated(note = "the name has changed to `CERTIFIED_MONITOR`", since = "0.7.2")]
        const DISCORD_CERTIFIED_MODERATOR = 1 << 18;
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
