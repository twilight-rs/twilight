use crate::{
    oauth::{
        team::TeamMembershipState,
        id::TeamId,
    },
    user::User,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TeamMember {
    pub membership_state: TeamMembershipState,
    pub permissions: Vec<String>,
    pub team_id: TeamId,
    pub user: User,
}
