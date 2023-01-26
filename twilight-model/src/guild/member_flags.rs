use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

bitflags! {
    /// Flags associated with a [`Member`].
    ///
    /// Refer to [Discord's documentation] for more information.
    ///
    /// [`Member`]: crate::guild::Member
    /// [Discord's documentation]: https://discord.com/developers/docs/resources/guild#guild-member-object-guild-member-flags
    pub struct MemberFlags: u64 {
        /// Member has left and rejoined the guild.
        const DID_REJOIN = 1 << 0;
        /// Member has completed onboarding.
        const COMPLETED_ONBOARDING = 1 << 1;
        /// Member is exempt from guild verification requirements.
        const BYPASSES_VERIFICATION = 1 << 2;
        /// Member has started onboarding.
        const STARTED_ONBOARDING = 1 << 3;
    }
}

impl<'de> Deserialize<'de> for MemberFlags {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_bits_truncate(u64::deserialize(deserializer)?))
    }
}

impl Serialize for MemberFlags {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.bits())
    }
}
