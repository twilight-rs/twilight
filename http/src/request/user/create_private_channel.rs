use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use serde::Serialize;
use twilight_model::{
    channel::PrivateChannel,
    id::{marker::UserMarker, Id},
};

#[derive(Serialize)]
struct CreatePrivateChannelFields {
    recipient_id: Id<UserMarker>,
}

/// Create a group DM.
///
/// This endpoint is limited to 10 active group DMs.
#[must_use = "requests must be configured and executed"]
pub struct CreatePrivateChannel<'a> {
    fields: CreatePrivateChannelFields,
    http: &'a Client,
}

impl<'a> CreatePrivateChannel<'a> {
    pub(crate) const fn new(http: &'a Client, recipient_id: Id<UserMarker>) -> Self {
        Self {
            fields: CreatePrivateChannelFields { recipient_id },
            http,
        }
    }
    pub fn exec(self) -> ResponseFuture<PrivateChannel> {
        let request = Request::builder(&Route::CreatePrivateChannel);

        let request = match request.json(&self.fields) {
            Ok(request) => request,
            Err(source) => return ResponseFuture::error(source),
        };

        self.http.request(request.build())
    }
}
