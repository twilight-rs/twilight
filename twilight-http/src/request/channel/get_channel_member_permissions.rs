//! Request builder and response body for getting a member's permissions in the
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
        marker::{ChannelMarker, UserMarker},
    },
};

/// Response body for the [`GetChannelMemberPermissions`] request.
#[derive(Clone, Debug, Deserialize)]
#[non_exhaustive]
pub struct GetChannelMemberPermissionsResponseBody {
    /// Computed permissions.
    pub permissions: Permissions,
}

struct GetChannelMemberPermissionsPath {
    channel_id: Id<ChannelMarker>,
    implicit: Option<bool>,
    user_id: Id<UserMarker>,
}

/// Get a member's computed permissions in a channel.
///
/// Includes their roles and member-specific overwrites. Threads use their
/// parent channel's overwrites.
///
/// Requires the [`VIEW_CHANNEL`] permission. Also requires the [`CONNECT`]
/// permission for [voice] and [stage] channels.
///
/// [`CONNECT`]: twilight_model::guild::Permissions::CONNECT
/// [`VIEW_CHANNEL`]: twilight_model::guild::Permissions::VIEW_CHANNEL
/// [stage]: twilight_model::channel::ChannelType::GuildStageVoice
/// [voice]: twilight_model::channel::ChannelType::GuildVoice
#[must_use = "requests must be configured and executed"]
pub struct GetChannelMemberPermissions<'a> {
    http: &'a Client,
    path: GetChannelMemberPermissionsPath,
}

impl<'a> GetChannelMemberPermissions<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        user_id: Id<UserMarker>,
    ) -> Self {
        Self {
            http,
            path: GetChannelMemberPermissionsPath {
                channel_id,
                implicit: None,
                user_id,
            },
        }
    }

    /// Whether to apply implicit permission masking for missing permissions.
    ///
    /// This applies implicit permission masking for [`VIEW_CHANNEL`] and
    /// [`CONNECT`] for [voice] and [stage] channels. Not applied when the
    /// target member is a bot.
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

impl IntoFuture for GetChannelMemberPermissions<'_> {
    type Output = Result<Response<GetChannelMemberPermissionsResponseBody>, Error>;

    type IntoFuture = ResponseFuture<GetChannelMemberPermissionsResponseBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetChannelMemberPermissions<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetChannelMemberPermissionsPath {
            channel_id,
            implicit,
            user_id,
        } = self.path;

        Ok(Request::from_route(&Route::GetChannelMemberPermissions {
            channel_id: channel_id.get(),
            implicit,
            user_id: user_id.get(),
        }))
    }
}
