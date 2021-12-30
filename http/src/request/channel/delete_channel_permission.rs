use super::DeleteChannelPermissionConfigured;
use crate::client::Client;
use twilight_model::id::{
    marker::{ChannelMarker, RoleMarker, UserMarker},
    Id,
};

/// Clear the permissions for a target ID in a channel.
///
/// The target ID must be set with one of the associated methods.
#[must_use = "requests must be configured and executed"]
pub struct DeleteChannelPermission<'a> {
    channel_id: Id<ChannelMarker>,
    http: &'a Client,
}

impl<'a> DeleteChannelPermission<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self { channel_id, http }
    }

    /// Delete an override for an member.
    pub const fn member(self, user_id: Id<UserMarker>) -> DeleteChannelPermissionConfigured<'a> {
        self.configure(user_id.get())
    }

    /// Delete an override for an role.
    pub const fn role(self, role_id: Id<RoleMarker>) -> DeleteChannelPermissionConfigured<'a> {
        self.configure(role_id.get())
    }

    const fn configure(self, target_id: u64) -> DeleteChannelPermissionConfigured<'a> {
        DeleteChannelPermissionConfigured::new(self.http, self.channel_id, target_id)
    }
}
