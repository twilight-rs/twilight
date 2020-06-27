mod metadata;
mod target_user_type;

pub use self::{metadata::InviteMetadata, target_user_type::TargetUserType};

use crate::{channel::Channel, guild::PartialGuild, user::User};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Invite {
    pub approximate_member_count: Option<u64>,
    pub approximate_presence_count: Option<u64>,
    pub channel: Channel,
    pub code: String,
    pub guild: Option<PartialGuild>,
    pub inviter: Option<User>,
    pub target_user_type: Option<TargetUserType>,
    pub target_user: Option<User>,
}
