mod metadata;
mod target_user_type;

pub use self::{metadata::InviteMetadata, target_user_type::TargetUserType};

use crate::{channel::Channel, guild::PartialGuild, user::User};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Invite {
    pub approximate_member_count: Option<u64>,
    pub approximate_presence_count: Option<u64>,
    pub code: String,
    pub channel: Option<Channel>,
    pub guild: Option<PartialGuild>,
    pub target_user: Option<User>,
    pub target_user_type: Option<TargetUserType>,
}

impl Hash for Invite {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}
