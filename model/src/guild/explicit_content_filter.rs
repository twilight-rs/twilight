use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Copy, Debug, Deserialize_repr, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize_repr,
)]
#[repr(u8)]
pub enum ExplicitContentFilter {
    None = 0,
    MembersWithoutRole = 1,
    AllMembers = 2,
}
