use crate::{id::RoleId, user::User};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InviteStageInstance {
    /// Members speaking in the Stage.
    pub members: Vec<InviteStageInstanceMember>,
    /// Total number of users.
    pub participant_count: u64,
    /// Number of speakers.
    pub speaker_count: u64,
    /// Topic of the Stage instance.
    ///
    /// Between 1 and 120 characters.
    pub topic: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct InviteStageInstanceMember {
    /// Guild specific avatar hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// ISO 8601 timestamp of the date the member joined the guild.
    pub joined_at: String,
    /// Member's nickname, if there is one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<String>,
    /// Whether the member has passed the guild's membership screening requirements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,
    /// ISO 8601 timestamp of the date the member boosted the guild.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<String>,
    /// List of role IDs the user has.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<RoleId>,
    /// User data for the member.
    pub user: User,
}

#[cfg(test)]
mod tests {
    use super::{InviteStageInstance, InviteStageInstanceMember};
    use serde::{Deserialize, Serialize};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(
        InviteStageInstance: members,
        participant_count,
        speaker_count,
        topic
    );

    assert_fields!(
        InviteStageInstanceMember: avatar,
        joined_at,
        nick,
        pending,
        premium_since,
        roles,
        user
    );

    assert_impl_all!(
        InviteStageInstance: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize,
        Send,
        Sync
    );

    assert_impl_all!(
        InviteStageInstanceMember: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize,
        Send,
        Sync
    );
}
