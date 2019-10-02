#[cfg_attr(
    feature = "serde-support",
    derive(serde_repr::Deserialize_repr, serde_repr::Serialize_repr)
)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(u8)]
pub enum ExplicitContentFilter {
    None = 0,
    MembersWithoutRole = 1,
    AllMembers = 2,
}
