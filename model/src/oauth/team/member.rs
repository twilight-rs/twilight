use crate::{
    oauth::{
        team::TeamMembershipState,
        id::TeamId,
    },
    user::User,
};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TeamMember {
    pub membership_state: TeamMembershipState,
    pub permissions: Vec<String>,
    pub team_id: TeamId,
    pub user: User,
}

impl Hash for TeamMember {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.team_id.hash(state);
        self.user.id.hash(state);
    }
}
