use crate::guild::{Guild, UnavailableGuild};

#[allow(clippy::large_enum_variant)]
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[cfg_attr(feature = "serde-support", serde(untagged))]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum GuildStatus {
    OnlineGuild(Guild),
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
                GuildStatus::OnlineGuild(g) => g.id,
                GuildStatus::Offline(u) => u.id,
            }
        }
    }
}
