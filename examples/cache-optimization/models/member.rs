use twilight_cache_inmemory::{model::ComputedInteractionMember, CacheableMember};
use twilight_model::{
    application::interaction::InteractionMember,
    gateway::payload::incoming::MemberUpdate,
    guild::{Member, PartialMember},
    id::{
        marker::{RoleMarker, UserMarker},
        Id,
    },
    util::{ImageHash, Timestamp},
};

#[derive(Clone, Debug, PartialEq)]
pub struct MinimalCachedMember {
    pub user_id: Id<UserMarker>,
    pub roles: Vec<Id<RoleMarker>>,
    pub avatar: Option<ImageHash>,
}

impl From<Member> for MinimalCachedMember {
    fn from(member: Member) -> Self {
        Self {
            user_id: member.user.id,
            roles: member.roles,
            avatar: member.avatar,
        }
    }
}

impl From<(Id<UserMarker>, PartialMember)> for MinimalCachedMember {
    fn from((user_id, member): (Id<UserMarker>, PartialMember)) -> Self {
        Self {
            user_id,
            roles: member.roles,
            avatar: member.avatar,
        }
    }
}

impl From<ComputedInteractionMember> for MinimalCachedMember {
    fn from(member: ComputedInteractionMember) -> Self {
        Self {
            user_id: member.user_id,
            roles: member.interaction_member.roles,
            avatar: member.avatar,
        }
    }
}

impl PartialEq<Member> for MinimalCachedMember {
    fn eq(&self, other: &Member) -> bool {
        self.user_id == other.user.id && self.roles == other.roles && self.avatar == other.avatar
    }
}

impl PartialEq<PartialMember> for MinimalCachedMember {
    fn eq(&self, other: &PartialMember) -> bool {
        other
            .user
            .as_ref()
            .is_some_and(|user| user.id == self.user_id)
            && self.roles == other.roles
            && self.avatar == other.avatar
    }
}

impl PartialEq<InteractionMember> for MinimalCachedMember {
    fn eq(&self, other: &InteractionMember) -> bool {
        self.roles == other.roles && self.avatar == other.avatar
    }
}

impl CacheableMember for MinimalCachedMember {
    fn avatar(&self) -> Option<ImageHash> {
        self.avatar
    }

    fn communication_disabled_until(&self) -> Option<Timestamp> {
        None
    }

    fn deaf(&self) -> Option<bool> {
        None
    }

    fn mute(&self) -> Option<bool> {
        None
    }

    fn roles(&self) -> &[Id<RoleMarker>] {
        &self.roles
    }

    fn update_with_member_update(&mut self, member_update: &MemberUpdate) {
        self.user_id = member_update.user.id;
        self.roles.clone_from(&member_update.roles);
        self.avatar = member_update.avatar;
    }
}
