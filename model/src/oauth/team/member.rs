use crate::{
    oauth::{id::TeamId, team::TeamMembershipState},
    user::User,
};

#[cfg_attr(
    feature = "serde-support",
    derive(serde::Deserialize, serde::Serialize)
)]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TeamMember {
    pub membership_state: TeamMembershipState,
    pub permissions: Vec<String>,
    pub team_id: TeamId,
    pub user: User,
}
