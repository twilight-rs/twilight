use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::user::Connection;

/// Get the current user's connections.
///
/// Requires the `connections` `OAuth2` scope.
#[must_use = "requests must be configured and executed"]
pub struct GetCurrentUserConnections<'a> {
    http: &'a Client,
}

impl<'a> GetCurrentUserConnections<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self { http }
    }
}

impl IntoFuture for GetCurrentUserConnections<'_> {
    type Output = Result<Response<ListBody<Connection>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<Connection>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetCurrentUserConnections<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetUserConnections))
    }
}
