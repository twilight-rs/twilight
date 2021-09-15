use crate::{
    client::Client,
    request::Request,
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::voice::VoiceRegion;

use super::IntoRequest;

/// Get a list of voice regions that can be used when creating a guild.
#[must_use = "requests must be configured and executed"]
pub struct GetVoiceRegions<'a> {
    http: &'a Client,
}

impl<'a> GetVoiceRegions<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self { http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<VoiceRegion>> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for GetVoiceRegions<'_> {
    fn into_request(self) -> Result<Request, crate::Error> {
        Ok(Request::from_route(&Route::GetVoiceRegions))
    }
}
