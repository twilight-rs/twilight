use super::{prelude::*, update_channel_permission_configured::UpdateChannelPermissionConfigured};
use dawn_model::{
    guild::Permissions,
    id::{ChannelId, RoleId, UserId},
};

pub struct UpdateChannelPermission<'a> {
    allow: Permissions,
    channel_id: ChannelId,
    deny: Permissions,
    http: &'a Client,
}

impl<'a> UpdateChannelPermission<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: ChannelId,
        allow: Permissions,
        deny: Permissions,
    ) -> Self {
        Self {
            allow,
            channel_id,
            deny,
            http,
        }
    }

    pub fn member(self, user_id: impl Into<UserId>) -> UpdateChannelPermissionConfigured<'a> {
        self.configure("member", user_id.into().0)
    }

    pub fn role(self, role_id: impl Into<RoleId>) -> UpdateChannelPermissionConfigured<'a> {
        self.configure("role", role_id.into().0)
    }

    fn configure(
        self,
        kind: impl Into<String>,
        target_id: u64,
    ) -> UpdateChannelPermissionConfigured<'a> {
        UpdateChannelPermissionConfigured::new(
            self.http,
            self.channel_id,
            self.allow,
            self.deny,
            kind,
            target_id,
        )
    }
}
