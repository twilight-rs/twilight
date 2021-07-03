use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    routing::Route,
};
use serde::Serialize;
use twilight_model::{channel::PrivateChannel, id::UserId};

#[derive(Serialize)]
struct CreatePrivateChannelFields {
    recipient_id: UserId,
}

/// Create a group DM.
///
/// This endpoint is limited to 10 active group DMs.
pub struct CreatePrivateChannel<'a> {
    fields: CreatePrivateChannelFields,
    fut: Option<PendingResponse<'a, PrivateChannel>>,
    http: &'a Client,
}

impl<'a> CreatePrivateChannel<'a> {
    pub(crate) fn new(http: &'a Client, recipient_id: UserId) -> Self {
        Self {
            fields: CreatePrivateChannelFields { recipient_id },
            fut: None,
            http,
        }
    }
    fn start(&mut self) -> Result<(), Error> {
        let request = Request::builder(Route::CreatePrivateChannel)
            .json(&self.fields)?
            .build();

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(CreatePrivateChannel<'_>, PrivateChannel);
