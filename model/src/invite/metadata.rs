use crate::user::User;

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InviteMetadata {
    pub created_at: String,
    pub inviter: User,
    pub max_age: u64,
    pub max_uses: u64,
    pub temporary: bool,
    pub uses: u64,
}
