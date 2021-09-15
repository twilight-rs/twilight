use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use twilight_model::oauth::CurrentApplicationInfo;

use super::IntoRequest;

#[must_use = "requests must be configured and executed"]
pub struct GetUserApplicationInfo<'a> {
    http: &'a Client,
}

impl<'a> GetUserApplicationInfo<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self { http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<CurrentApplicationInfo> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for GetUserApplicationInfo<'_> {
    fn into_request(self) -> Result<Request, crate::Error> {
        Ok(Request::from_route(&Route::GetCurrentUserApplicationInfo))
    }
}
