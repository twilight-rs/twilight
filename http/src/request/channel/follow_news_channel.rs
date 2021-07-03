use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    routing::Route,
};
use serde::Serialize;
use twilight_model::{channel::FollowedChannel, id::ChannelId};

#[derive(Default, Serialize)]
struct FollowNewsChannelFields {
    webhook_channel_id: ChannelId,
}

/// Follow a news channel by [`ChannelId`]s.
pub struct FollowNewsChannel<'a> {
    channel_id: ChannelId,
    fields: FollowNewsChannelFields,
    fut: Option<PendingResponse<'a, FollowedChannel>>,
    http: &'a Client,
}

impl<'a> FollowNewsChannel<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: ChannelId,
        webhook_channel_id: ChannelId,
    ) -> Self {
        Self {
            channel_id,
            fut: None,
            http,
            fields: FollowNewsChannelFields { webhook_channel_id },
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::builder(Route::FollowNewsChannel {
            channel_id: self.channel_id.0,
        })
        .json(&self.fields)?
        .build();

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(FollowNewsChannel<'_>, FollowedChannel);
