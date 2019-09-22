use crate::user::User;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InviteMetadata {
    #[cfg(feature = "chrono")]
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    #[cfg(not(feature = "chrono"))]
    pub created_at: String,
    pub inviter: User,
    pub max_age: u64,
    pub max_uses: u64,
    pub revoked: bool,
    pub temporary: bool,
    pub uses: u64,
}
