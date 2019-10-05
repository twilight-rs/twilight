use crate::guild::{Guild, UnavailableGuild};

#[allow(clippy::large_enum_variant)]
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[cfg_attr(feature = "serde-support", serde(untagged))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GuildStatus {
    Online(Guild),
    Offline(UnavailableGuild),
}

#[cfg(feature = "serde-support")]
mod serde_support {
    use super::GuildStatus;
    use crate::id::GuildId;
    use serde_mappable_seq::Key;

    impl Key<'_, GuildId> for GuildStatus {
        fn key(&self) -> GuildId {
            match self {
                Self::Online(g) => g.id,
                Self::Offline(u) => u.id,
            }
        }
    }
}
