use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InviteMetadata {
    pub created_at: String,
    pub inviter: User,
    pub max_age: u64,
    pub max_uses: u64,
    pub temporary: bool,
    pub uses: u64,
}
