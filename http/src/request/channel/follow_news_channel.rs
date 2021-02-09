use crate::request::prelude::*;
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
    fut: Option<Pending<'a, FollowedChannel>>,
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

    fn start(&mut self) -> Result<()> {
        let request = Request::from((
            crate::json_to_vec(&self.fields).map_err(HttpError::json)?,
            Route::FollowNewsChannel {
                channel_id: self.channel_id.0,
            },
        ));

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(FollowNewsChannel<'_>, FollowedChannel);
