use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    channel::Channel,
    id::{marker::UserMarker, Id},
};

#[derive(Serialize)]
struct CreatePrivateChannelFields {
    recipient_id: Id<UserMarker>,
}

/// Create a DM channel with a user.
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
}

impl IntoFuture for CreatePrivateChannel<'_> {
    type Output = Result<Response<Channel>, Error>;

    type IntoFuture = ResponseFuture<Channel>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreatePrivateChannel<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::CreatePrivateChannel)
            .json(&self.fields)
            .build()
    }
}
