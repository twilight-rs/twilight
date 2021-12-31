use super::UpdateChannelPermissionConfigured;
use crate::client::Client;
use twilight_model::{
    channel::permission_overwrite::PermissionOverwriteType,
    guild::Permissions,
    id::{
        marker::{ChannelMarker, RoleMarker, UserMarker},
        Id,
    },
};

/// Update the permissions for a role or a user in a channel.
///
/// # Examples:
///
/// Create permission overrides for a role to view the channel, but not send messages:
///
/// ```no_run
/// use twilight_http::Client;
/// use twilight_model::guild::Permissions;
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let channel_id = Id::new(123);
/// let allow = Permissions::VIEW_CHANNEL;
/// let deny = Permissions::SEND_MESSAGES;
/// let role_id = Id::new(432);
///
/// client.update_channel_permission(channel_id, allow, deny)
///     .role(role_id)
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct UpdateChannelPermission<'a> {
    allow: Permissions,
    channel_id: Id<ChannelMarker>,
    deny: Permissions,
    http: &'a Client,
}

impl<'a> UpdateChannelPermission<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
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

    /// Specify this override to be for a member.
    pub const fn member(self, user_id: Id<UserMarker>) -> UpdateChannelPermissionConfigured<'a> {
        self.configure(PermissionOverwriteType::Member(user_id))
    }

    /// Specify this override to be for a role.
    pub const fn role(self, role_id: Id<RoleMarker>) -> UpdateChannelPermissionConfigured<'a> {
        self.configure(PermissionOverwriteType::Role(role_id))
    }

    const fn configure(
        self,
        target: PermissionOverwriteType,
    ) -> UpdateChannelPermissionConfigured<'a> {
        UpdateChannelPermissionConfigured::new(
            self.http,
            self.channel_id,
            self.allow,
            self.deny,
            target,
        )
    }
}
