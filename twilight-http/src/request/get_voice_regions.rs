use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::voice::VoiceRegion;

/// Get a list of voice regions that can be used when creating a guild.
#[must_use = "requests must be configured and executed"]
pub struct GetVoiceRegions<'a> {
    http: &'a Client,
}

impl<'a> GetVoiceRegions<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self { http }
    }
}

impl IntoFuture for GetVoiceRegions<'_> {
    type Output = Result<Response<ListBody<VoiceRegion>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<VoiceRegion>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetVoiceRegions<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetVoiceRegions))
    }
}
