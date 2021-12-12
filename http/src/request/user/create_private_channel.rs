use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
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
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreatePrivateChannel<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let request = Request::builder(&Route::CreatePrivateChannel);

        let request = request.json(&self.fields)?;

        Ok(request.build())
    }
}
