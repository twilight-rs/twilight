#[cfg_attr(
    feature = "serde-support",
    derive(serde_repr::Deserialize_repr, serde_repr::Serialize_repr)
)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum TeamMembershipState {
    Invited = 1,
    Accepted = 2,
}
