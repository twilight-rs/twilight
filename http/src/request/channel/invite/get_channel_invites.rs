use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    response::marker::ListBody,
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
pub struct GetChannelInvites<'a> {
    channel_id: ChannelId,
    fut: Option<PendingResponse<'a, ListBody<Invite>>>,
    http: &'a Client,
}

impl<'a> GetChannelInvites<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetChannelInvites {
            channel_id: self.channel_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetChannelInvites<'_>, ListBody<Invite>);
