use crate::{
    client::Client,
    request::{IntoRequest, Request},
    response::ResponseFuture,
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
#[must_use = "requests must be configured and executed"]
pub struct CreatePrivateChannel<'a> {
    fields: CreatePrivateChannelFields,
    http: &'a Client,
}

impl<'a> CreatePrivateChannel<'a> {
    pub(crate) const fn new(http: &'a Client, recipient_id: UserId) -> Self {
        Self {
            fields: CreatePrivateChannelFields { recipient_id },
            http,
        }
    }
    pub fn exec(self) -> ResponseFuture<PrivateChannel> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for CreatePrivateChannel<'_> {
    fn into_request(self) -> Result<Request, crate::Error> {
        let request = Request::builder(&Route::CreatePrivateChannel);

        let request = request.json(&self.fields)?;

        Ok(request.build())
    }
}
