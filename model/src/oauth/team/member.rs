use crate::{
    oauth::{id::TeamId, team::TeamMembershipState},
    user::User,
};
use std::hash::{Hash, Hasher};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, PartialEq)]
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
