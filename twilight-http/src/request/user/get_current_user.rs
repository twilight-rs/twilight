use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::user::CurrentUser;

/// Get information about the current user.
#[must_use = "requests must be configured and executed"]
pub struct GetCurrentUser<'a> {
    http: &'a Client,
}

impl<'a> GetCurrentUser<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self { http }
    }
}

impl IntoFuture for GetCurrentUser<'_> {
    type Output = Result<Response<CurrentUser>, Error>;

    type IntoFuture = ResponseFuture<CurrentUser>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetCurrentUser<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetCurrentUser))
    }
}
