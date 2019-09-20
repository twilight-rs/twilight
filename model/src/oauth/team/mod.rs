mod member;
mod membership_state;

pub use self::{
    member::TeamMember,
    membership_state::TeamMembershipState,
};

use crate::{
    id::UserId,
    oauth::id::TeamId,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Team {
    pub id: TeamId,
    pub icon: Option<String>,
    pub members: Vec<TeamMember>,
    pub owner_user_id: UserId,
}
