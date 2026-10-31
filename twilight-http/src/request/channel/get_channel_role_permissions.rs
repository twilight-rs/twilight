//! Request builder and response body for getting a role's permissions in the
//! context of a channel.

use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Deserialize;
use std::future::IntoFuture;
use twilight_model::{
    guild::Permissions,
    id::{
        Id,
        marker::{ChannelMarker, RoleMarker},
    },
};

/// Response body for the [`GetChannelRolePermissions`] request.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct GetChannelRolePermissionsResponseBody {
    /// Computed permissions.
    pub permissions: Permissions,
}

struct GetChannelRolePermissionsPath {
    channel_id: Id<ChannelMarker>,
    implicit: Option<bool>,
    role_id: Id<RoleMarker>,
}

/// Get the permissions granted by the specified role together with `@everyone`,
/// applying their channel overwrites.
///
/// Other roles and member-specific overwrites are not included. Threads use
/// their parent channel's overwrites.
///
/// Requires the [`VIEW_CHANNEL`] permission. Also requires the [`CONNECT`]
/// permission for [voice] and [stage] channels.
///
/// [`CONNECT`]: twilight_model::guild::Permissions::CONNECT
/// [`VIEW_CHANNEL`]: twilight_model::guild::Permissions::VIEW_CHANNEL
/// [stage]: twilight_model::channel::ChannelType::GuildStageVoice
/// [voice]: twilight_model::channel::ChannelType::GuildVoice
#[must_use = "requests must be configured and executed"]
pub struct GetChannelRolePermissions<'a> {
    http: &'a Client,
    path: GetChannelRolePermissionsPath,
}

impl<'a> GetChannelRolePermissions<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        role_id: Id<RoleMarker>,
    ) -> Self {
        Self {
            http,
            path: GetChannelRolePermissionsPath {
                channel_id,
                implicit: None,
                role_id,
            },
        }
    }

    /// Whether to apply implicit permission masking for missing permissions.
    ///
    /// This applies implicit permission masking for [`VIEW_CHANNEL`] and
    /// [`CONNECT`] for [voice] and [stage] channels.
    ///
    /// Discord defaults this to true.
    ///
    /// [`CONNECT`]: twilight_model::guild::Permissions::CONNECT
    /// [`VIEW_CHANNEL`]: twilight_model::guild::Permissions::VIEW_CHANNEL
    /// [stage]: twilight_model::channel::ChannelType::GuildStageVoice
    /// [voice]: twilight_model::channel::ChannelType::GuildVoice
    pub const fn implicit(mut self, implicit: bool) -> Self {
        self.path.implicit = Some(implicit);

        self
    }
}

impl IntoFuture for GetChannelRolePermissions<'_> {
    type Output = Result<Response<GetChannelRolePermissionsResponseBody>, Error>;

    type IntoFuture = ResponseFuture<GetChannelRolePermissionsResponseBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetChannelRolePermissions<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetChannelRolePermissionsPath {
            channel_id,
            implicit,
            role_id,
        } = self.path;

        Ok(Request::from_route(&Route::GetChannelRolePermissions {
            channel_id: channel_id.get(),
            implicit,
            role_id: role_id.get(),
        }))
    }
}
