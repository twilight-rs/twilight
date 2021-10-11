use crate::{
    client::Client,
    error::Error,
    request::{IntoRequest, Request},
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{id::ChannelId, invite::Invite};

/// Get the invites for a guild channel.
///
/// Requires the [`MANAGE_CHANNELS`] permission. This method only works if the
/// channel is of type [`GuildChannel`].
///
/// [`MANAGE_CHANNELS`]: twilight_model::guild::Permissions::MANAGE_CHANNELS
/// [`GuildChannel`]: twilight_model::channel::GuildChannel
#[must_use = "requests must be configured and executed"]
pub struct GetChannelInvites<'a> {
    channel_id: ChannelId,
    http: &'a Client,
}

impl<'a> GetChannelInvites<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self { channel_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<Invite>> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for GetChannelInvites<'_> {
    fn into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetChannelInvites {
            channel_id: self.channel_id.get(),
        }))
    }
}
