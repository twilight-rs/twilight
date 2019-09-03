use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Debug, Eq, Deserialize_repr, Ord, PartialEq, PartialOrd, Serialize_repr)]
#[repr(u8)]
pub enum TeamMembershipState {
    Invited = 1,
    Accepted = 2,
}
