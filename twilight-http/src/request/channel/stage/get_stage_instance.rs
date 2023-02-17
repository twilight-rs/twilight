use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    channel::StageInstance,
    id::{marker::ChannelMarker, Id},
};

/// Gets the stage instance associated with a stage channel, if it exists.
#[must_use = "requests must be configured and executed"]
pub struct GetStageInstance<'a> {
    channel_id: Id<ChannelMarker>,
    http: &'a Client,
}

impl<'a> GetStageInstance<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self { channel_id, http }
    }
}

impl IntoFuture for GetStageInstance<'_> {
    type Output = Result<Response<StageInstance>, Error>;

    type IntoFuture = ResponseFuture<StageInstance>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetStageInstance<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetStageInstance {
            channel_id: self.channel_id.get(),
        }))
    }
}
