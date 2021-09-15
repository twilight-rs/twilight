use crate::{
    client::Client,
    request::{IntoRequest, Request},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{channel::StageInstance, id::ChannelId};

/// Gets the stage instance associated with a stage channel, if it exists.
#[must_use = "requests must be configured and executed"]
pub struct GetStageInstance<'a> {
    channel_id: ChannelId,
    http: &'a Client,
}

impl<'a> GetStageInstance<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self { channel_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<StageInstance> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for GetStageInstance<'_> {
    fn into_request(self) -> Result<Request, crate::Error> {
        Ok(Request::from_route(&Route::GetStageInstance {
            channel_id: self.channel_id.get(),
        }))
    }
}
