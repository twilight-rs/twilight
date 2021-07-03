use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    routing::Route,
};
use twilight_model::{channel::StageInstance, id::ChannelId};

/// Gets the stage instance associated with a stage channel, if it exists.
pub struct GetStageInstance<'a> {
    channel_id: ChannelId,
    fut: Option<PendingResponse<'a, StageInstance>>,
    http: &'a Client,
}

impl<'a> GetStageInstance<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetStageInstance {
            channel_id: self.channel_id.0,
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetStageInstance<'_>, StageInstance);
