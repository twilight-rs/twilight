use crate::id::RoleId;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PartialMember {
    pub deaf: bool,
    pub joined_at: Option<String>,
    pub mute: bool,
    pub roles: Vec<RoleId>,
}
