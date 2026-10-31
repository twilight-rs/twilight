use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::ListBody},
};
use std::future::IntoFuture;
use twilight_model::{
    guild::invite::Invite,
    id::{Id, marker::ChannelMarker},
};

struct GetChannelInvitesPath {
    channel_id: Id<ChannelMarker>,
}

/// Get the invites for a guild channel.
///
/// Requires the [`MANAGE_CHANNELS`] permission. This method only works if the
/// channel is a guild channel.
///
/// [`MANAGE_CHANNELS`]: twilight_model::guild::Permissions::MANAGE_CHANNELS
#[must_use = "requests must be configured and executed"]
pub struct GetChannelInvites<'a> {
    http: &'a Client,
    path: GetChannelInvitesPath,
}

impl<'a> GetChannelInvites<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            http,
            path: GetChannelInvitesPath { channel_id },
        }
    }
}

impl IntoFuture for GetChannelInvites<'_> {
    type Output = Result<Response<ListBody<Invite>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<Invite>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetChannelInvites<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetChannelInvitesPath { channel_id } = self.path;

        let path = Path::builder()
            .resource("channels")
            .id(channel_id)
            .resource("invites")
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
