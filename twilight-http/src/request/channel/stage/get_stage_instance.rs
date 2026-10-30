use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    channel::StageInstance,
    id::{Id, marker::ChannelMarker},
};

struct GetStageInstancePath {
    channel_id: Id<ChannelMarker>,
}

/// Gets the stage instance associated with a stage channel, if it exists.
#[must_use = "requests must be configured and executed"]
pub struct GetStageInstance<'a> {
    http: &'a Client,
    path: GetStageInstancePath,
}

impl<'a> GetStageInstance<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<ChannelMarker>) -> Self {
        Self {
            http,
            path: GetStageInstancePath { channel_id },
        }
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
        let GetStageInstancePath { channel_id } = self.path;

        let path = Path::builder()
            .resource("stage-instances")
            .id(channel_id)
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}
