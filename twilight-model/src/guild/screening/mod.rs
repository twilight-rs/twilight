//! Types for guild screening.

mod application_field;
mod request_status;

use crate::{
    id::{
        Id,
        marker::{GuildMarker, JoinRequestMarker, UserMarker},
    },
    user::User,
    util::Timestamp,
};
use serde::{Deserialize, Serialize};

pub use self::{
    application_field::{ApplicationFieldResponse, MultipleChoiceFieldResponse, TextFieldResponse},
    request_status::JoinRequestStatus,
};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct JoinRequest {
    pub application_status: JoinRequestStatus,
    pub created_at: Timestamp,
    /// Applicant's responses on join request form.
    pub form_responses: Vec<ApplicationFieldResponse>,
    pub guild_id: Id<GuildMarker>,
    pub id: Id<JoinRequestMarker>,
    /// Reason for rejection. Only used when action is REJECTED.
    pub rejection_reason: Option<String>,
    pub reviewed_at: Option<Timestamp>,
    pub user_id: Id<UserMarker>,
    pub user: Option<User>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct JoinRequestList {
    /// The join requests, only returned with the `KICK_MEMBERS` permission
    pub guild_join_requests: Vec<JoinRequest>,
    /// Number of join requests with the given status, only returned when `status` is `SUBMITTED` or omitted.
    /// Apps that only have `MANAGE_GUILD` receive the count of pending join requests without the requests themselves.
    pub total: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        JoinRequestList: Clone,
        Debug,
        Eq,
        Hash,
        PartialEq,
        Send,
        Sync
    );
}
